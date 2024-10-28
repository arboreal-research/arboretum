pub(crate) const SUBGRAPH_IMPL_MMAP16: &str = "mmap16";
pub(crate) const SUBGRAPH_IMPL_MMAP32: &str = "mmap32";
pub(crate) const SUBGRAPH_IMPL_MMAP64: &str = "mmap64";
pub(crate) const SUBGRAPH_IMPL_SLED64: &str = "sled64";

/// The id which represents nodes which have not yet been assigned a subgraph id.
///
/// This allows graphs to be constructed via batches without needing to first request
/// and obtain an id for the new nodes.
pub const LOCAL_GRAPH_ID: u32 = 0;

// EDGES
pub(crate) const SET_ITEM: u64 = 0;
pub(crate) const SUBGRAPH_ID: u64 = 1;
pub(crate) const SUBGRAPH_PATH: u64 = 2;
pub(crate) const SUBGRAPH_IMPL: u64 = 3;
pub(crate) const SUBGRAPH_DOMAIN: u64 = 4;

// CONSTANT NODES
pub(crate) const SUBGRAPHS_SET_NODE: u64 = 5;
pub(crate) const NEXT_GRAPH_ID_NODE: u64 = 6;
