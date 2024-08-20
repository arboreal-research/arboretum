pub const SUBGRAPH_IMPL_MMAP16: &str = "mmap16";
pub const SUBGRAPH_IMPL_MMAP32: &str = "mmap32";
pub const SUBGRAPH_IMPL_MMAP64: &str = "mmap64";
pub const SUBGRAPH_IMPL_SLED64: &str = "sled64";

pub const LOCAL_GRAPH_ID: u32 = 0;

// EDGES
pub const SET_ITEM: u64 = 0;
pub const SUBGRAPH_ID: u64 = 1;
pub const SUBGRAPH_PATH: u64 = 2;
pub const SUBGRAPH_IMPL: u64 = 3;
pub const SUBGRAPH_DOMAIN: u64 = 4;

// CONSTANT NODES
pub const SUBGRAPHS_SET_NODE: u64 = 5;
pub const NEXT_GRAPH_ID_NODE: u64 = 6;
