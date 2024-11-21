use std::{
    fmt::Debug,
    path::Path,
    sync::{Arc, Mutex},
};

use crate::{Error, Subgraph, SubgraphConfig};
use arboretum_core::Domain;

#[derive(Clone)]
pub struct SubgraphEntry {
    inner: Arc<SubgraphEntryInner>,
}

impl Debug for SubgraphEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\nSubgraphEntry{\n")?;
        f.write_fmt(format_args!("  graph_id: {}\n", self.inner.graph_id))?;
        f.write_fmt(format_args!("  domain: {:?}\n", self.inner.domain))?;
        f.write_fmt(format_args!("  config: {:?}\n", self.inner.config))?;
        f.write_str("}\n")
    }
}

struct SubgraphEntryInner {
    graph_id: u32,
    domain: Domain,
    config: SubgraphConfig,
    graph: Mutex<Option<Arc<Subgraph>>>,
}

impl SubgraphEntry {
    pub fn new(graph_id: u32, domain: Domain, config: SubgraphConfig) -> Self {
        Self {
            inner: Arc::new(SubgraphEntryInner {
                graph_id,
                domain,
                config,
                graph: Mutex::new(None),
            }),
        }
    }

    pub fn graph_id(&self) -> u32 {
        self.inner.graph_id
    }

    pub fn domain(&self) -> &Domain {
        &self.inner.domain
    }

    pub fn config(&self) -> &SubgraphConfig {
        &self.inner.config
    }

    pub fn size(&self, subgraphs_path: &Path) -> Result<usize, Error> {
        let lock = self.inner.graph.lock()?;

        if let Some(subgraph) = lock.as_ref() {
            subgraph.get_memory_usage()
        } else {
            self.inner.config.get_memory_estimate(subgraphs_path)
        }
    }

    pub fn is_mutable(&self) -> bool {
        self.inner.config.is_mutable()
    }

    pub fn load<P: AsRef<Path>>(&self, subgraphs_path: P) -> Result<Arc<Subgraph>, Error> {
        let mut lock = self.inner.graph.lock()?;

        match lock.as_ref() {
            Some(subgraph) => Ok(subgraph.clone()),
            None => {
                let subgraph: Arc<Subgraph> = Arc::new(Subgraph::from_config(
                    subgraphs_path.as_ref(),
                    &self.inner.config,
                )?);
                *lock = Some(subgraph.clone());
                Ok(subgraph)
            }
        }
    }

    pub fn evict(&self) -> Result<(), Error> {
        self.inner.graph.lock()?.take();
        Ok(())
    }
}

impl std::hash::Hash for SubgraphEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.graph_id.hash(state)
    }
}

impl PartialEq for SubgraphEntry {
    fn eq(&self, other: &Self) -> bool {
        self.inner.graph_id.eq(&other.inner.graph_id)
    }
}

impl Eq for SubgraphEntry {}

impl PartialOrd for SubgraphEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.inner.graph_id.partial_cmp(&other.inner.graph_id)
    }
}

impl Ord for SubgraphEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.graph_id.cmp(&other.inner.graph_id)
    }
}
