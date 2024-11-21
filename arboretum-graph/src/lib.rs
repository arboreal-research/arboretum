mod cache;
mod error;
mod memory_info;
mod mmap;
mod root_graph;
mod sled;
mod smart_mmap_builder;
mod subgraph;
mod subgraph_config;
mod subgraph_entry;

pub use crate::sled::{graph::SledGraph, string_storage::SledStringStorage};
pub use cache::SubgraphCacheStrategy;
pub use error::Error;
pub use memory_info::{get_memory_info, MemoryInfo};
pub use mmap::{MmapGraph, MmapGraphBuilder, MmapGraphBuilderOptions, MmapGraphRangeIter};
pub use root_graph::{GraphOptions, RootGraph};
pub use subgraph::Subgraph;
pub use subgraph_config::SubgraphConfig;

#[macro_export]
macro_rules! query {
    ($mmap:expr, $sub:tt -?-> ?) => {{
        use arboretum_core::Prefix;
        $mmap.prefix_edges_spo(Prefix::One($sub))
    }};

    ($mmap:expr, ? -$pred:tt-> ?) => {{
        $mmap.prefix_edges_pos(Prefix::One($pred))
    }};

    ($mmap:expr, ? -?-> $obj:tt) => {{
        $mmap.prefix_edges_osp(Prefix::One($pred))
    }};

    ($mmap:expr, $sub:tt -$pred:tt-> ?) => {{
        $mmap.prefix_edges_spo(Prefix::Two($sub, $pred))
    }};

    ($mmap:expr, ? -$pred:tt-> $obj:tt) => {{
        $mmap.prefix_edges_pos(Prefix::Two($pred, $obj))
    }};

    ($mmap:expr, $sub:tt -?-> $obj:tt) => {{
        $mmap.prefix_edges_osp(Prefix::Two($obj, $sub))
    }};

    ($mmap:expr, $sub:tt -$pred:tt-> $obj:tt) => {{
        $mmap.prefix_edges_spo(Prefix::Three($sub, $pred, $obj))
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
        use arboretum_core::Prefix;
        $mmap.par_prefix_edges_spo(Prefix::One($sub))
    }};

    ($mmap:expr, ? -$pred:tt-> ?) => {{
        use $crate::Prefix;
        $mmap.par_prefix_edges_pos(Prefix::One($pred))
    }};

    ($mmap:expr, $sub:tt -$pred:tt-> ?) => {{
        use $crate::Prefix;
        $mmap.par_prefix_edges_spo(Prefix::Two($sub, $pred))
    }};

    ($mmap:expr, ? -$pred:tt-> $obj:tt) => {{
        use $crate::Prefix;
        $mmap.par_prefix_edges_pos(Prefix::Two($pred, $obj))
    }};

    ($mmap:expr, $sub:tt -?-> $obj:tt) => {{
        use $crate::Prefix;
        $mmap.par_prefix_edges_osp(Prefix::Two($obj, $sub))
    }};

    ($mmap:expr, $sub:tt -$pred:tt-> $obj:tt) => {{
        use $crate::Prefix;
        $mmap.par_prefix_edges_spo(Prefix::Three($sub, $pred, $obj))
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
