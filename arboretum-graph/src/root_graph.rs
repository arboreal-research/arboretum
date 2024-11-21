use crate::{
    cache::{LruSubgraphCache, SubgraphCache, SubgraphCacheStrategy},
    error::Error,
    query,
    smart_mmap_builder::SmartMmapBuilder,
    subgraph_entry::SubgraphEntry,
    SledGraph, SledStringStorage, Subgraph, SubgraphConfig,
};
use arboretum_core::{constant::*, split_u64, Domain, GraphBuffer, Prefix, Value};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::sync::RwLock;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
use tracing::{error, trace};

#[derive(Debug)]
pub struct GraphOptions {
    pub cache_strategy: SubgraphCacheStrategy,
}

impl Default for GraphOptions {
    fn default() -> Self {
        Self {
            cache_strategy: SubgraphCacheStrategy::LRU {
                max_usage_bytes: crate::memory_info::get_memory_info()
                    .total_virtual_memory()
                    .unwrap() as usize,
            },
        }
    }
}

#[derive(Clone)]
pub struct RootGraph {
    inner: Arc<RwLock<RootGraphInner>>,
}

struct RootGraphInner {
    root_subgraph: SledGraph<Value, Value>,
    name_assoc: SledStringStorage,
    subgraph_by_id: HashMap<u32, SubgraphEntry>,
    subgraph_by_domain: HashMap<Domain, SubgraphEntry>,
    domain_by_graph_id: HashMap<u32, HashSet<Domain>>,
    subgraphs_path: PathBuf,
    entry_cache: Arc<Mutex<dyn SubgraphCache>>,
}

impl RootGraph {
    /// Load a RootGraph from a folder structure organized as follows:
    ///
    ///   <path>/subgraphs/root/
    ///
    ///   <path>/subgraphs/0/
    ///   <path>/subgraphs/1/
    ///         ...
    ///   <path/subgraphs/N/
    ///
    /// If subgraphs does not exist, or the root graph does not exist this will
    /// prepare the root subgraph for organizing information about subgraphs.
    #[tracing::instrument(level = "trace", skip(path))]
    pub fn from_folder<P: AsRef<Path>>(path: P, opts: GraphOptions) -> Result<RootGraph, Error> {
        trace!("path={:?}", path.as_ref());

        let mut initialize = false;
        let strings_path = path.as_ref().join("strings");
        trace!(?strings_path);

        let subgraphs_path = path.as_ref().join("subgraphs");
        trace!(?subgraphs_path);

        let root_subgraph_path = path.as_ref().join("root");
        trace!(?root_subgraph_path);

        if !root_subgraph_path.exists() {
            trace!("!root_subgraph_path.exists()");
            std::fs::create_dir_all(strings_path.clone())?;
            std::fs::create_dir_all(subgraphs_path.clone())?;
            std::fs::create_dir_all(root_subgraph_path.clone())?;
            initialize = true;
        }

        let root_subgraph = SledGraph::from_file(root_subgraph_path)?;

        // If we know we need to initialize the graph (i.e. it's new), then do so
        if initialize {
            assert!(root_subgraph.add_node()? == SET_ITEM);
            assert!(root_subgraph.add_node()? == SUBGRAPH_ID);
            assert!(root_subgraph.add_node()? == SUBGRAPH_PATH);
            assert!(root_subgraph.add_node()? == SUBGRAPH_IMPL);
            assert!(root_subgraph.add_node()? == SUBGRAPH_DOMAIN);
            assert!(root_subgraph.add_node()? == SUBGRAPHS_SET_NODE);
            assert!(root_subgraph.add_node_with_props(Value::Unsigned(1))? == NEXT_GRAPH_ID_NODE);
        }

        let entry_cache = match opts.cache_strategy {
            SubgraphCacheStrategy::LRU { max_usage_bytes } => {
                LruSubgraphCache::new(subgraphs_path.clone(), max_usage_bytes)
            }
        };

        // Construct the string storage
        let name_assoc = SledStringStorage::from_folder(strings_path)?;

        name_assoc.dump();

        let result = Self {
            inner: Arc::new(RwLock::new(RootGraphInner {
                root_subgraph,
                name_assoc,
                entry_cache: Arc::new(Mutex::new(entry_cache)),
                subgraph_by_id: HashMap::new(),
                subgraph_by_domain: HashMap::new(),
                domain_by_graph_id: HashMap::new(),
                subgraphs_path,
            })),
        };

        {
            let mut inner = result.inner.write()?;

            for (_, _, subgraph_node, _) in
                query! {inner.root_subgraph, SUBGRAPHS_SET_NODE -SET_ITEM-> ?}?
            {
                trace!(subgraph_node);
                let subgraph_id = query! {inner.root_subgraph, subgraph_node -SUBGRAPH_ID-> ? }?
                    .map(|(_, _, d, _)| inner.root_subgraph.get_node_props(d))
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .filter_map(|prop| match prop {
                        Some(Value::Unsigned(i)) => Some(i),
                        Some(other) => {
                            error!(
                                "Expected Value::Unsigned for SUBGRAPH_ID, but found {:?}",
                                other
                            );
                            None
                        }
                        None => {
                            error!("Expected Value::Unsigned for SUBGRAPH_ID, but found None");
                            None
                        }
                    })
                    .next()
                    .ok_or_else(|| Error::MalformedSubgraphConfig(subgraph_node as u32))?
                    as u32;

                let subgraph_path =
                    query! {inner.root_subgraph, subgraph_node -SUBGRAPH_PATH-> ? }?
                        .map(|(_, _, d, _)| inner.root_subgraph.get_node_props(d))
                        .collect::<Result<Vec<_>, _>>()?
                        .into_iter()
                        .filter_map(|props| match props {
                            Some(Value::String(s)) => Some(s),
                            Some(other) => {
                                error!(
                                    "Expected Value::String for SUBGRAPH_PATH, but found {:?}",
                                    other
                                );
                                None
                            }
                            None => {
                                error!("Expected Value::String for SUBGRAPH_PATH, but found None");
                                None
                            }
                        })
                        .next()
                        .ok_or_else(|| Error::MalformedSubgraphConfig(subgraph_node as u32))?;

                let subgraph_impl =
                    query! {inner.root_subgraph, subgraph_node -SUBGRAPH_IMPL-> ? }?
                        .map(|(_, _, d, _)| inner.root_subgraph.get_node_props(d))
                        .collect::<Result<Vec<_>, _>>()?
                        .into_iter()
                        .filter_map(|props| match props {
                            Some(Value::String(s)) => Some(s),
                            Some(other) => {
                                error!(
                                    "Expected Value::String for SUBGRAPH_IMPL, but found {:?}",
                                    other
                                );
                                None
                            }
                            None => {
                                error!("Expected Value::String for SUBGRAPH_IMPL, but found None");
                                None
                            }
                        })
                        .next()
                        .ok_or_else(|| Error::MalformedSubgraphConfig(subgraph_node as u32))?;

                let subgraph_domain = Domain::from_iter(
                    query! {inner.root_subgraph, subgraph_node -SUBGRAPH_DOMAIN-> ? }?
                        .map(|(_, _, d, _)| inner.root_subgraph.get_node_props(d))
                        .collect::<Result<Vec<_>, _>>()?
                        .into_iter()
                        .filter_map(|props| match props {
                            Some(Value::Unsigned(i)) => Some(i as u32),
                            Some(other) => {
                                error!(
                                    "Expected Value::Unsigned for SUBGRAPH_DOMAIN, but found {:?}",
                                    other
                                );
                                None
                            }
                            None => {
                                error!(
                                    "Expected Value::Unsigned for SUBGRAPH_DOMAIN, but found None"
                                );
                                None
                            }
                        }),
                )
                .ok_or_else(|| Error::MalformedSubgraphConfig(subgraph_node as u32))?;

                inner.add_subgraph(
                    subgraph_id,
                    subgraph_domain,
                    SubgraphConfig::from_impl_name(
                        subgraph_impl.as_str(),
                        subgraph_id,
                        subgraph_path.into(),
                    )
                    .ok_or_else(|| Error::MalformedSubgraphConfig(subgraph_node as u32))?,
                );
            }

            // Initialize the next_graph_id, if needed.
            match inner.root_subgraph.get_node_props(NEXT_GRAPH_ID_NODE)? {
                Some(Value::Unsigned(next_graph_id)) => {
                    trace!(next_graph_id);
                }
                _ => {
                    trace!("NEXT_GRAPH_ID_NODE did not already exist. Starting at 1.");
                    inner
                        .root_subgraph
                        .add_node_with_props(Value::Unsigned(1))?;
                }
            };
        }

        Ok(result)
    }

    pub fn get_node_name(&self, id: u64) -> Result<Option<String>, Error> {
        self.inner.read()?.name_assoc.get_str(id)
    }

    pub fn get_named_node<S>(&self, name: S) -> Result<Option<u64>, Error>
    where
        S: AsRef<str>,
    {
        self.inner.read()?.name_assoc.get_id(name.as_ref())
    }

    #[tracing::instrument(level = "trace", skip(self, buffer))]
    pub fn add_graph_buffer(
        &self,
        mut buffer: GraphBuffer,
    ) -> Result<(Vec<u32>, GraphBuffer), Error> {
        let mut graph_ids = Vec::new();

        let mut inner = self.inner.write()?;

        // First, allocate a new graph for any nodes with the local graph_id (i.e. 0).
        if buffer.uses_local_graph_id() {
            trace!("buffer.uses_local_graph_id()");
            // First, assign a fresh id.
            let graph_id = inner.fresh_graph_id()?;
            trace!(graph_id);
            graph_ids.push(graph_id);

            buffer = buffer.substitute_graph_id(graph_id);
        }

        // Next, we'll use the string storage to perform substitution on nodes which exist in other subgraphs.
        let subst = buffer
            .named_nodes()
            .par_iter()
            .filter_map(|(proposed_id, name)| {
                match inner.name_assoc.get_or_assoc(name, *proposed_id) {
                    Ok(actual_id) => {
                        if *proposed_id != actual_id {
                            Some(Ok((*proposed_id, actual_id)))
                        } else {
                            None
                        }
                    }
                    Err(e) => Some(Err(e)),
                }
            })
            .collect::<Result<HashMap<u64, u64>, Error>>()?;
        trace!("# subst {}", subst.len());
        if subst.len() > 0 {
            buffer = buffer.substitute_id(subst);
        }

        let mut failure_buffer = GraphBuffer::new();

        for (domain, data) in buffer.into_data_by_domain() {
            trace!(?domain);

            // Determine if this subgraph already exists.
            let subgraph_entry = inner.subgraph_by_domain.get(&domain).cloned();
            trace!(?subgraph_entry);

            if let Some(subgraph_entry) = subgraph_entry {
                // The subgraph needs to be mutable (i.e. sled), or else we cannot process these edges.
                if subgraph_entry.is_mutable() {
                    inner.load(&subgraph_entry)?.extend_with(data)?;
                    graph_ids.push(subgraph_entry.graph_id());
                } else {
                    failure_buffer.extend_with(domain, data);
                }
            } else {
                // Assign a fresh graph_id, if it isn't a single id domain
                let graph_id = if let Domain::Single(graph_id) = domain {
                    graph_id
                } else {
                    inner.fresh_graph_id()?
                };
                trace!(graph_id);
                graph_ids.push(graph_id);

                let subgraphs_path = inner.subgraphs_path.clone();
                trace!(?subgraphs_path);

                std::fs::create_dir(subgraphs_path.join(graph_id.to_string()))?;

                match (domain.clone(), data).into() {
                    SmartMmapBuilder::U16(b) => {
                        let rel_path = PathBuf::new().join(graph_id.to_string()).join("mmap16");
                        trace!(?rel_path);
                        b.build(subgraphs_path.join(rel_path.as_path()))?;
                        inner.add_subgraph_internal(
                            graph_id,
                            rel_path,
                            SUBGRAPH_IMPL_MMAP16,
                            domain,
                        )?;
                    }
                    SmartMmapBuilder::U32(b) => {
                        let rel_path = PathBuf::new().join(graph_id.to_string()).join("mmap32");
                        trace!(?rel_path);
                        b.build(subgraphs_path.join(rel_path.as_path()))?;
                        inner.add_subgraph_internal(
                            graph_id,
                            rel_path,
                            SUBGRAPH_IMPL_MMAP32,
                            domain,
                        )?;
                    }
                    SmartMmapBuilder::U64(b) => {
                        let rel_path = PathBuf::new().join(graph_id.to_string()).join("mmap64");
                        trace!(?rel_path);
                        b.build(subgraphs_path.join(rel_path.as_path()))?;
                        inner.add_subgraph_internal(
                            graph_id,
                            rel_path,
                            SUBGRAPH_IMPL_MMAP64,
                            domain,
                        )?;
                    }
                }
            }
        }

        Ok((graph_ids, failure_buffer))
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn add_sled64_subgraph(&self) -> Result<SubgraphEntry, Error> {
        let mut inner = self.inner.write()?;

        // First, assign a fresh id.
        let graph_id = inner.fresh_graph_id()?;
        trace!(graph_id);

        std::fs::create_dir(self.subgraphs_path()?.join(graph_id.to_string()))?;

        inner.add_subgraph_internal(
            graph_id,
            graph_id.to_string().into(),
            SUBGRAPH_IMPL_SLED64,
            Domain::Single(graph_id),
        )
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn add_sled64_subgraph_with_domain(&self, domain: Domain) -> Result<SubgraphEntry, Error> {
        let mut inner = self.inner.write()?;

        // First, assign a fresh id.
        let graph_id = inner.fresh_graph_id()?;
        trace!(graph_id);

        // Create the folder for this new graph.
        std::fs::create_dir(self.subgraphs_path()?.join(graph_id.to_string()))?;

        inner.add_subgraph_internal(
            graph_id,
            graph_id.to_string().into(),
            SUBGRAPH_IMPL_SLED64,
            domain,
        )
    }

    pub fn subgraphs_path(&self) -> Result<PathBuf, Error> {
        let lock = self.inner.read()?;
        Ok(lock.subgraphs_path.clone())
    }

    pub fn subgraphs(&self) -> Result<Vec<SubgraphEntry>, Error> {
        let lock = self.inner.read()?;
        Ok(lock.subgraph_by_id.values().map(|v| v.clone()).collect())
    }

    pub fn get_subgraph_by_id(&self, subgraph_id: u32) -> Result<Option<SubgraphEntry>, Error> {
        let lock = self.inner.read()?;
        Ok(lock.subgraph_by_id.get(&subgraph_id).cloned())
    }

    pub fn get_subgraph_by_domain(&self, domain: Domain) -> Result<Option<SubgraphEntry>, Error> {
        let lock = self.inner.read()?;
        Ok(lock.subgraph_by_domain.get(&domain).cloned())
    }

    pub fn find_associated_domains(
        &self,
        subgraph_id: u32,
    ) -> Result<Option<HashSet<Domain>>, Error> {
        let lock = self.inner.read()?;
        Ok(lock.domain_by_graph_id.get(&subgraph_id).cloned())
    }
}

impl RootGraph {
    pub fn get_node_props(&self, id: u64) -> Result<Option<Value>, Error> {
        let graph_id = (id >> 32) as u32;

        let subgraph = self
            .inner
            .read()?
            .subgraph_by_domain
            .get(&Domain::Single(graph_id))
            .cloned();

        Ok(subgraph
            .map(|subgraph| -> Result<Option<Value>, Error> {
                Ok(self.inner.read()?.load(&subgraph)?.get_node_props(id)?)
            })
            .transpose()?
            .flatten())
    }

    pub fn prefix_edges_spo(
        &self,
        prefix: Prefix<u64>,
    ) -> Result<Vec<(u64, u64, u64, Option<Value>)>, Error> {
        self.prefix_edges_spo_with_extra_domains(prefix, [])
    }

    pub fn prefix_edges_pos(
        &self,
        prefix: Prefix<u64>,
    ) -> Result<Vec<(u64, u64, u64, Option<Value>)>, Error> {
        self.prefix_edges_pos_with_extra_domains(prefix, [])
    }

    pub fn prefix_edges_osp(
        &self,
        prefix: Prefix<u64>,
    ) -> Result<Vec<(u64, u64, u64, Option<Value>)>, Error> {
        self.prefix_edges_osp_with_extra_domains(prefix, [])
    }

    pub fn prefix_edges_spo_global(
        &self,
        prefix: Prefix<u64>,
    ) -> Result<Vec<(u64, u64, u64, Option<Value>)>, Error> {
        let lock = self.inner.read()?;
        let mut extra_domains = HashSet::new();
        prefix.apply(|id| {
            lock.domain_by_graph_id
                .get(&split_u64(id).0)
                .map(|id_domains| extra_domains.extend(id_domains.clone()));
        });

        self.prefix_edges_spo_with_extra_domains(prefix, extra_domains)
    }

    pub fn prefix_edges_pos_global(
        &self,
        prefix: Prefix<u64>,
    ) -> Result<Vec<(u64, u64, u64, Option<Value>)>, Error> {
        let lock = self.inner.read()?;
        let mut extra_domains = HashSet::new();
        prefix.apply(|id| {
            let graph_id = split_u64(id).0;
            error!(graph_id);
            let id_domains = lock.domain_by_graph_id.get(&graph_id);
            error!(?id_domains);

            id_domains.map(|id_domains| extra_domains.extend(id_domains.clone()));
        });
        error!(?extra_domains);

        self.prefix_edges_pos_with_extra_domains(prefix, extra_domains)
    }

    pub fn prefix_edges_osp_global(
        &self,
        prefix: Prefix<u64>,
    ) -> Result<Vec<(u64, u64, u64, Option<Value>)>, Error> {
        let lock = self.inner.read()?;
        let mut extra_domains = HashSet::new();
        prefix.apply(|id| {
            lock.domain_by_graph_id
                .get(&split_u64(id).0)
                .map(|id_domains| extra_domains.extend(id_domains.clone()));
        });

        self.prefix_edges_osp_with_extra_domains(prefix, extra_domains)
    }

    pub fn prefix_edges_spo_with_extra_domains<I>(
        &self,
        prefix: Prefix<u64>,
        extra_domains: I,
    ) -> Result<Vec<(u64, u64, u64, Option<Value>)>, Error>
    where
        I: IntoIterator<Item = Domain>,
    {
        let domains = prefix_domain(prefix.clone()).chain(extra_domains.into_iter());

        let subgraphs;
        {
            let lock = self.inner.read()?;
            subgraphs = domains
                .filter_map(|domain| lock.subgraph_by_domain.get(&domain).cloned())
                .collect::<Vec<_>>();
        }

        let inner_clone = self.inner.clone();
        Ok(subgraphs
            .par_iter()
            .map(move |subgraph| -> Result<_, Error> {
                Ok(inner_clone
                    .read()?
                    .load(subgraph)?
                    .prefix_edges_spo(prefix.clone())?
                    .collect::<Vec<_>>())
            })
            .try_reduce(
                || Vec::new(),
                |mut a, b| {
                    a.extend(b.into_iter());
                    Ok(a)
                },
            )?)
    }

    pub fn prefix_edges_pos_with_extra_domains<I>(
        &self,
        prefix: Prefix<u64>,
        extra_domains: I,
    ) -> Result<Vec<(u64, u64, u64, Option<Value>)>, Error>
    where
        I: IntoIterator<Item = Domain>,
    {
        let domains = prefix_domain(prefix.clone()).chain(extra_domains.into_iter());

        let subgraphs;
        {
            let lock = self.inner.read()?;
            subgraphs = domains
                .filter_map(|domain| lock.subgraph_by_domain.get(&domain).cloned())
                .collect::<Vec<_>>();
        }

        let inner_clone = self.inner.clone();
        Ok(subgraphs
            .par_iter()
            .map(
                |subgraph| -> Result<Vec<(u64, u64, u64, Option<Value>)>, Error> {
                    Ok(inner_clone
                        .read()?
                        .load(subgraph)?
                        .prefix_edges_pos(prefix.clone())?
                        .collect::<Vec<_>>())
                },
            )
            .try_reduce(
                || Vec::new(),
                |mut a, b| {
                    a.extend(b.into_iter());
                    Ok(a)
                },
            )?)
    }

    pub fn prefix_edges_osp_with_extra_domains<I>(
        &self,
        prefix: Prefix<u64>,
        extra_domains: I,
    ) -> Result<Vec<(u64, u64, u64, Option<Value>)>, Error>
    where
        I: IntoIterator<Item = Domain>,
    {
        let domains = prefix_domain(prefix.clone()).chain(extra_domains.into_iter());

        let subgraphs;
        {
            let lock = self.inner.read()?;
            subgraphs = domains
                .filter_map(|domain| lock.subgraph_by_domain.get(&domain).cloned())
                .collect::<Vec<_>>();
        }

        let inner_clone = self.inner.clone();
        Ok(subgraphs
            .par_iter()
            .map(
                move |subgraph| -> Result<Vec<(u64, u64, u64, Option<Value>)>, Error> {
                    Ok(inner_clone
                        .read()?
                        .load(subgraph)?
                        .prefix_edges_osp(prefix.clone())?
                        .collect::<Vec<_>>())
                },
            )
            .try_reduce(
                || Vec::new(),
                |mut a, b| {
                    a.extend(b.into_iter());
                    Ok(a)
                },
            )?)
    }
}

impl RootGraphInner {
    fn fresh_graph_id(&self) -> Result<u32, Error> {
        Ok(match self.root_subgraph.fetch_and_update_node_props(
            NEXT_GRAPH_ID_NODE,
            |old| match old {
                Some(Value::Unsigned(u)) => Some(Value::Unsigned(*u + 1)),
                Some(other) => {
                    error!(
                        "Expected Value::Unsigned for NEXT_GRAPH_ID_NODE, but found {:?}",
                        other
                    );
                    None
                }
                None => {
                    error!("Expected Value::Unsigned for NEXT_GRAPH_ID_NODE, but found None");
                    None
                }
            },
        )? {
            Some(Value::Unsigned(u)) => u,
            _ => Err(Error::MalformedRootGraphNextSubgraphId)?,
        } as u32)
    }

    fn add_subgraph(
        &mut self,
        subgraph_id: u32,
        domain: Domain,
        config: SubgraphConfig,
    ) -> SubgraphEntry {
        let subgraph_ref = SubgraphEntry::new(subgraph_id, domain.clone(), config);

        self.subgraph_by_id
            .insert(subgraph_id, subgraph_ref.clone());

        match domain {
            Domain::Single(a) => {
                self.associate_graph_with_domain(a, domain.clone());
            }
            Domain::Double(a, b) => {
                self.associate_graph_with_domain(a, domain.clone());
                self.associate_graph_with_domain(b, domain.clone());
            }
            Domain::Triple(a, b, c) => {
                self.associate_graph_with_domain(a, domain.clone());
                self.associate_graph_with_domain(b, domain.clone());
                self.associate_graph_with_domain(c, domain.clone());
            }
        }

        self.subgraph_by_domain.insert(domain, subgraph_ref.clone());

        subgraph_ref
    }

    fn associate_graph_with_domain(&mut self, graph_id: u32, domain: Domain) {
        match self.domain_by_graph_id.entry(graph_id) {
            std::collections::hash_map::Entry::Occupied(mut o) => {
                o.get_mut().insert(domain);
            }
            std::collections::hash_map::Entry::Vacant(v) => {
                v.insert(HashSet::from([domain]));
            }
        }
        error!(?self.domain_by_graph_id);
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn add_subgraph_internal(
        &mut self,
        graph_id: u32,
        rel_path: PathBuf,
        impl_name: &str,
        domain: Domain,
    ) -> Result<SubgraphEntry, Error> {
        // Add it to the root graph.
        let graph_node = self.root_subgraph.add_node()?;
        trace!(graph_node);

        self.root_subgraph.add_edge((
            graph_node,
            SUBGRAPH_ID,
            self.root_subgraph
                .add_node_with_props(Value::Unsigned(graph_id as u64))?,
        ))?;
        self.root_subgraph.add_edge((
            graph_node,
            SUBGRAPH_IMPL,
            self.root_subgraph
                .add_node_with_props(Value::String(impl_name.to_string()))?,
        ))?;
        self.root_subgraph.add_edge((
            graph_node,
            SUBGRAPH_PATH,
            self.root_subgraph
                .add_node_with_props(Value::String(rel_path.to_string_lossy().to_string()))?,
        ))?;

        let mut emit_domain_component = |g: u32| -> Result<_, Error> {
            self.root_subgraph.add_edge((
                graph_node,
                SUBGRAPH_DOMAIN,
                self.root_subgraph
                    .add_node_with_props(Value::Unsigned(g as u64))?,
            ))?;

            Ok(())
        };
        match domain {
            Domain::Single(a) => {
                emit_domain_component(a)?;
            }
            Domain::Double(a, b) => {
                emit_domain_component(a)?;
                emit_domain_component(b)?;
            }
            Domain::Triple(a, b, c) => {
                emit_domain_component(a)?;
                emit_domain_component(b)?;
                emit_domain_component(c)?;
            }
        }

        self.root_subgraph
            .add_edge((SUBGRAPHS_SET_NODE, SET_ITEM, graph_node))?;

        // Add it to the union graph and return it.
        Ok(self.add_subgraph(
            graph_id,
            domain,
            SubgraphConfig::from_impl_name(impl_name, graph_id, rel_path).unwrap(),
        ))
    }

    pub(crate) fn load(&self, entry: &SubgraphEntry) -> Result<Arc<Subgraph>, Error> {
        // Notify the cache of our intent.
        self.entry_cache.lock()?.notify_used(entry.clone())?;
        Ok(entry.load(self.subgraphs_path.clone())?)
    }
}

fn prefix_domain(prefix: Prefix<u64>) -> impl Iterator<Item = Domain> {
    let mut domains = HashSet::new();

    match prefix {
        Prefix::One(a) => {
            domains.insert(Domain::Single((a >> 32) as u32));
        }

        Prefix::Two(a, b) => {
            let a = (a >> 32) as u32;
            let b = (b >> 32) as u32;
            Domain::from_iter([a, b]).map(|d| domains.insert(d));
            domains.insert(Domain::Single(a));
            domains.insert(Domain::Single(b));
        }

        Prefix::Three(a, b, c) => {
            let a = (a >> 32) as u32;
            let b = (b >> 32) as u32;
            let c = (c >> 32) as u32;
            Domain::from_iter([a, b, c]).map(|d| domains.insert(d));
            Domain::from_iter([a, b]).map(|d| domains.insert(d));
            Domain::from_iter([b, c]).map(|d| domains.insert(d));
            Domain::from_iter([a, c]).map(|d| domains.insert(d));
            domains.insert(Domain::Single(a));
            domains.insert(Domain::Single(b));
            domains.insert(Domain::Single(c));
        }
    }

    domains.into_iter()
}

#[cfg(test)]
mod test {
    use arboretum_core::{GraphBuffer, Value};

    use super::RootGraph;

    #[test]
    fn test_create() {
        let dir = tempdir::TempDir::new("root").unwrap();

        let root = RootGraph::from_folder(dir.path(), Default::default()).unwrap();

        let a = root.add_sled64_subgraph().unwrap();

        let mut buffer = GraphBuffer::new();
        buffer.add_node_with_props(1, Value::String("foo1".into()));
        buffer.add_node_with_props(2, Value::String("foo2".into()));
        buffer.add_node_with_props(3, Value::String("foo3".into()));

        buffer.add_node_with_props(
            ((a.graph_id() as u64) << 32) + 1,
            Value::String("bar1".into()),
        );
        buffer.add_node_with_props(
            ((a.graph_id() as u64) << 32) + 2,
            Value::String("bar2".into()),
        );
        buffer.add_node_with_props(
            ((a.graph_id() as u64) << 32) + 3,
            Value::String("bar3".into()),
        );

        buffer.add_edge_with_props(
            (1, 0, ((a.graph_id() as u64) << 32) + 1),
            Value::String("foo1 -> bar1".into()),
        );
        buffer.add_edge_with_props(
            (
                2,
                ((a.graph_id() as u64) << 32),
                ((a.graph_id() as u64) << 32) + 2,
            ),
            Value::String("foo2 -> bar2".into()),
        );
        buffer.add_edge_with_props(
            (((a.graph_id() as u64) << 32) + 3, 0, 3),
            Value::String("bar3 -> foo3".into()),
        );

        let (_, failures) = root.add_graph_buffer(buffer).unwrap();
        assert!(failures.empty());

        for (i, subgraph) in root.subgraphs().unwrap().into_iter().enumerate() {
            println!("Entry {}: {:?}", i, subgraph);
        }
    }
}
