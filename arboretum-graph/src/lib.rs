use rkyv::{Archive, Deserialize, Serialize};

pub mod constant;

mod cache;
mod domain;
mod error;
mod graph_buffer;
mod map_id;
mod memory_info;
mod mmap;
mod root_graph;
mod sled;
mod smart_mmap_builder;
mod subgraph;
mod subgraph_config;
mod subgraph_entry;
mod types;

pub use crate::sled::{graph::SledGraph, string_storage::SledStringStorage};
pub use domain::Domain;
pub use error::Error;
pub use graph_buffer::GraphBuffer;
pub use memory_info::{get_memory_info, MemoryInfo};
pub use mmap::{MmapGraph, MmapGraphBuilder, MmapGraphBuilderOptions, MmapGraphRangeIter};
pub use root_graph::RootGraph;
pub use subgraph::Subgraph;
pub use subgraph_config::SubgraphConfig;
pub use types::{IdType, PropsType};

#[derive(Clone, Debug, Archive, Serialize, Deserialize)]
#[archive_attr(derive(Debug))]
pub enum Value {
    Unsigned(u64),
    Signed(i64),
    Double(f64),
    String(String),
}

#[macro_export]
macro_rules! query {
    ($mmap:expr, $sub:tt -?-> ?) => {{
        $mmap.prefix_edges_spo(($sub, None))
    }};

    ($mmap:expr, $sub:tt -$pred:tt-> ?) => {{
        $mmap.prefix_edges_spo(($sub, Some(($pred, None))))
    }};

    ($mmap:expr, $sub:tt -$pred:tt-> $obj:tt) => {{
        $mmap.prefix_edges_spo(($sub, Some(($pred, $obj))))
    }};

    ($mmap:expr, ? -$pred:tt-> ?) => {{
        $mmap.prefix_edges_pos(($pred, None))
    }};

    ($mmap:expr, ? -$pred:tt-> $obj:tt) => {{
        $mmap.prefix_edges_pos(($pred, Some(($obj, None))))
    }};

    ($mmap:expr, $sub:tt -?-> $obj:tt) => {{
        $mmap.prefix_edges_osp(($obj, Some(($sub, None))))
    }};

    ///////////////////////////////////////////////////////////////////////////
    ($mmap:expr, $domains:expr, $sub:tt -?-> ?) => {{
        $mmap.prefix_edges_spo_with_extra_domains($domains, ($sub, None))
    }};

    ($mmap:expr, $domains:expr, $sub:tt -$pred:tt-> ?) => {{
        $mmap.prefix_edges_spo_with_extra_domains($domains, ($sub, Some(($pred, None))))
    }};

    ($mmap:expr, $domains:expr, $sub:tt -$pred:tt-> $obj:tt) => {{
        $mmap.prefix_edges_spo_with_extra_domains($domains, ($sub, Some(($pred, $obj))))
    }};

    ($mmap:expr, $domains:expr, ? -$pred:tt-> ?) => {{
        $mmap.prefix_edges_pos_with_extra_domains($domains, ($pred, None))
    }};

    ($mmap:expr, $domains:expr, ? -$pred:tt-> $obj:tt) => {{
        $mmap.prefix_edges_pos_with_extra_domains($domains, ($pred, Some(($obj, None))))
    }};

    ($mmap:expr, $domains:expr, $sub:tt -?-> $obj:tt) => {{
        $mmap.prefix_edges_osp_with_extra_domains($domains, ($obj, Some(($sub, None))))
    }};
}

#[macro_export]
macro_rules! par_query {
    ($mmap:expr, $sub:tt -?-> ?) => {{
        $mmap.par_prefix_edges_spo(($sub, None))
    }};

    ($mmap:expr, $sub:tt -$pred:tt-> ?) => {{
        $mmap.par_prefix_edges_spo(($sub, Some(($pred, None))))
    }};

    ($mmap:expr, $sub:tt -$pred:tt-> $obj:tt) => {{
        $mmap.par_prefix_edges_spo(($sub, Some(($pred, $obj))))
    }};

    ($mmap:expr, ? -$pred:tt-> ?) => {{
        $mmap.par_prefix_edges_pos(($pred, None))
    }};

    ($mmap:expr, ? -$pred:tt-> $obj:tt) => {{
        $mmap.par_prefix_edges_pos(($pred, Some(($obj, None))))
    }};

    ($mmap:expr, $sub:tt -?-> $obj:tt) => {{
        $mmap.par_prefix_edges_osp(($obj, Some(($sub, None))))
    }};

    ///////////////////////////////////////////////////////////////////////////
    ($mmap:expr, $domains:expr, $sub:tt -?-> ?) => {{
        $mmap.par_prefix_edges_spo_with_extra_domains($domains, ($sub, None))
    }};

    ($mmap:expr, $domains:expr, $sub:tt -$pred:tt-> ?) => {{
        $mmap.par_prefix_edges_spo_with_extra_domains($domains, ($sub, Some(($pred, None))))
    }};

    ($mmap:expr, $domains:expr, $sub:tt -$pred:tt-> $obj:tt) => {{
        $mmap.par_prefix_edges_spo_with_extra_domains($domains, ($sub, Some(($pred, $obj))))
    }};

    ($mmap:expr, $domains:expr, ? -$pred:tt-> ?) => {{
        $mmap.par_prefix_edges_pos_with_extra_domains($domains, ($pred, None))
    }};

    ($mmap:expr, $domains:expr, ? -$pred:tt-> $obj:tt) => {{
        $mmap.par_prefix_edges_pos_with_extra_domains($domains, ($pred, Some(($obj, None))))
    }};

    ($mmap:expr, $domains:expr, $sub:tt -?-> $obj:tt) => {{
        $mmap.par_prefix_edges_osp_with_extra_domains(($obj, Some(($sub, None))))
    }};
}

pub(crate) fn split_u64(value: u64) -> (u32, u32) {
    let high = (value >> 32) as u32;
    let low = value as u32;
    (high, low)
}

pub(crate) fn merge_u64(high: u32, low: u32) -> u64 {
    ((high as u64) << 32) + (low as u64)
}
