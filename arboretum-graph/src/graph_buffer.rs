use std::collections::{hash_map::Entry, HashMap, HashSet};

use crate::{constant::*, map_id::MapId, merge_u64, split_u64, Domain, Value};

/// Buffer for node properties and edges which can live in any number of domains.
///
/// This is the most common way to construct new graph data (though using MmapGraphBuilder directly is also possible).
///
/// The underlying implementation of MmapGraph for each domain will be chosen based on the number of nodes, edges, and
/// whether or not the domain contains data from one or more graph id's.
pub struct GraphBuffer {
    edges: HashMap<Domain, HashMap<(u64, u64, u64), Option<Value>>>,
    node_props: HashMap<Domain, HashMap<u64, Value>>,
    named_nodes: HashMap<u64, String>,
    uses_local_graph_id: bool,
}

impl GraphBuffer {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            node_props: HashMap::new(),
            named_nodes: HashMap::new(),
            uses_local_graph_id: false,
        }
    }

    pub fn empty(&self) -> bool {
        self.edges.len() == 0 && self.node_props.len() == 0
    }

    pub fn uses_local_graph_id(&self) -> bool {
        self.uses_local_graph_id
    }

    pub fn substitute_graph_id(self, graph_id: u32) -> GraphBuffer {
        let mut new_self = self.map_id(&|id: u64| rebuild_id(graph_id, id));
        new_self.uses_local_graph_id = false;
        new_self
    }

    pub fn substitute_id(self, subst: HashMap<u64, u64>) -> GraphBuffer {
        self.map_id(&|id: u64| *subst.get(&id).unwrap_or(&id))
    }

    pub fn named_nodes(&self) -> &HashMap<u64, String> {
        &self.named_nodes
    }

    pub fn take_named_nodes(&mut self) -> HashMap<u64, String> {
        std::mem::take(&mut self.named_nodes)
    }

    pub fn into_data_by_domain<'a>(
        mut self,
    ) -> impl Iterator<
        Item = (
            Domain,
            (HashMap<u64, Value>, HashMap<(u64, u64, u64), Option<Value>>),
        ),
    > {
        let node_props_domains = self
            .node_props
            .keys()
            .map(|k| k.clone())
            .collect::<HashSet<Domain>>();

        let named_node_domains = self
            .named_nodes
            .keys()
            .map(|id| Domain::Single((id >> 32) as u32))
            .collect::<HashSet<Domain>>();

        let edge_domains = self
            .edges
            .keys()
            .map(|k| k.clone())
            .collect::<HashSet<Domain>>();

        let domains = node_props_domains
            .union(&edge_domains)
            .map(|d| d.clone())
            .collect::<HashSet<_>>()
            .union(&named_node_domains)
            .map(|d| d.clone())
            .collect::<HashSet<_>>();

        domains.into_iter().map(
            move |domain| -> (
                Domain,
                (HashMap<u64, Value>, HashMap<(u64, u64, u64), Option<Value>>),
            ) {
                let node_props = self
                    .node_props
                    .remove(&domain)
                    .unwrap_or_else(|| HashMap::new());

                let edges = self.edges.remove(&domain).unwrap_or_else(|| HashMap::new());

                (domain.clone(), (node_props, edges))
            },
        )
    }

    pub fn add_named_node<S>(&mut self, id: u64, name: S)
    where
        S: AsRef<str>,
    {
        let graph_id = (id >> 32) as u32;

        if graph_id == LOCAL_GRAPH_ID {
            self.uses_local_graph_id = true;
        }

        self.named_nodes.insert(id, name.as_ref().to_string());
    }

    pub fn add_named_node_with_props<S>(&mut self, id: u64, name: S, props: Value)
    where
        S: AsRef<str>,
    {
        let graph_id = (id >> 32) as u32;

        if graph_id == LOCAL_GRAPH_ID {
            self.uses_local_graph_id = true;
        }

        self.named_nodes.insert(id, name.as_ref().to_string());

        match self.node_props.entry(Domain::Single(graph_id)) {
            Entry::Occupied(mut o) => {
                o.get_mut().insert(id, props);
            }
            Entry::Vacant(v) => {
                v.insert(HashMap::from([(id, props)]));
            }
        };
    }

    pub fn add_node_with_props(&mut self, id: u64, props: Value) {
        let graph_id = (id >> 32) as u32;

        if graph_id == LOCAL_GRAPH_ID {
            self.uses_local_graph_id = true;
        }

        match self.node_props.entry(Domain::Single(graph_id)) {
            Entry::Occupied(mut o) => {
                o.get_mut().insert(id, props);
            }
            Entry::Vacant(v) => {
                v.insert(HashMap::from([(id, props)]));
            }
        };
    }

    pub fn add_edge(&mut self, triple: (u64, u64, u64)) {
        let s_graph = (triple.0 >> 32) as u32;
        let p_graph = (triple.1 >> 32) as u32;
        let o_graph = (triple.2 >> 32) as u32;

        if s_graph == LOCAL_GRAPH_ID || p_graph == LOCAL_GRAPH_ID || o_graph == LOCAL_GRAPH_ID {
            self.uses_local_graph_id = true;
        }

        match self
            .edges
            .entry(Domain::from_iter([s_graph, p_graph, o_graph]).unwrap())
        {
            Entry::Occupied(mut e) => {
                e.get_mut().insert(triple, None);
            }
            Entry::Vacant(e) => {
                e.insert(HashMap::from([(triple, None)]));
            }
        };
    }

    pub fn add_edge_with_props(&mut self, triple: (u64, u64, u64), props: Value) {
        let s_graph = (triple.0 >> 32) as u32;
        let p_graph = (triple.1 >> 32) as u32;
        let o_graph = (triple.2 >> 32) as u32;

        if s_graph == LOCAL_GRAPH_ID || p_graph == LOCAL_GRAPH_ID || o_graph == LOCAL_GRAPH_ID {
            self.uses_local_graph_id = true;
        }

        match self
            .edges
            .entry(Domain::from_iter([s_graph, p_graph, o_graph]).unwrap())
        {
            Entry::Occupied(mut e) => {
                e.get_mut().insert(triple, Some(props));
            }
            Entry::Vacant(e) => {
                e.insert(HashMap::from([(triple, Some(props))]));
            }
        };
    }

    pub fn extend_with(
        &mut self,
        domain: Domain,
        (node_props, edges): (HashMap<u64, Value>, HashMap<(u64, u64, u64), Option<Value>>),
    ) {
        match self.node_props.entry(domain.clone()) {
            Entry::Occupied(mut o) => {
                o.get_mut().extend(node_props);
            }
            Entry::Vacant(v) => {
                v.insert(node_props);
            }
        };

        match self.edges.entry(domain) {
            Entry::Occupied(mut o) => {
                o.get_mut().extend(edges);
            }
            Entry::Vacant(v) => {
                v.insert(edges);
            }
        };
    }

    pub fn domains(&self) -> HashSet<&Domain> {
        let mut result = HashSet::new();

        result.extend(self.node_props.keys());
        result.extend(self.edges.keys());

        result
    }
}

impl std::fmt::Debug for GraphBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\nGraphBuffer{\n")?;
        f.write_fmt(format_args!("  # named: {}\n", self.named_nodes.len()))?;
        for domain in self.domains() {
            f.write_fmt(format_args!("  {:?}:\n", domain))?;
            f.write_fmt(format_args!(
                "    # node props: {}\n",
                self.node_props
                    .get(domain)
                    .map(|props| props.len())
                    .unwrap_or(0)
            ))?;
            f.write_fmt(format_args!(
                "    # edges: {}\n",
                self.edges.get(domain).map(|edges| edges.len()).unwrap_or(0)
            ))?;
        }
        f.write_str("}\n")
    }
}

impl MapId for GraphBuffer {
    fn map_id<F>(self, f: &F) -> GraphBuffer
    where
        F: Fn(u64) -> u64 + Send + Sync,
    {
        let mut result = GraphBuffer::new();

        // Node Props
        self.node_props.into_iter().for_each(|(_, props)| {
            props.into_iter().for_each(|(id, value)| {
                result.add_node_with_props(f(id), value);
            });
        });

        // Named Nodes
        self.named_nodes
            .into_iter()
            .for_each(|(id, name)| result.add_named_node(f(id), name));

        // Edges
        self.edges.into_iter().for_each(|(_, edges)| {
            edges.into_iter().for_each(|(edge_key, value)| {
                if let Some(value) = value {
                    result.add_edge_with_props(edge_key.map_id(f), value)
                } else {
                    result.add_edge(edge_key.map_id(f))
                }
            });
        });

        result
    }
}

pub(crate) fn rebuild_id(graph_id: u32, id: u64) -> u64 {
    let (high_id, low_id) = split_u64(id);
    if high_id == LOCAL_GRAPH_ID {
        merge_u64(graph_id, low_id)
    } else {
        id
    }
}
