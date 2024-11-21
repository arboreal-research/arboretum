use crate::{MmapGraphBuilder, MmapGraphBuilderOptions};
use arboretum_core::{Domain, Value};
use std::collections::HashMap;

pub enum SmartMmapBuilder {
    U16(MmapGraphBuilder<u16, Value, Value>),
    U32(MmapGraphBuilder<u32, Value, Value>),
    U64(MmapGraphBuilder<u64, Value, Value>),
}

impl
    From<(
        Domain,
        (HashMap<u64, Value>, HashMap<(u64, u64, u64), Option<Value>>),
    )> for SmartMmapBuilder
{
    fn from(
        (domain, (node_props, edges)): (
            Domain,
            (HashMap<u64, Value>, HashMap<(u64, u64, u64), Option<Value>>),
        ),
    ) -> Self {
        let num_nodes = node_props.len();
        let num_edges = edges.len();

        if let Domain::Single(_) = &domain {
            if num_nodes < u16::MAX as usize && num_edges < u16::MAX as usize {
                // U16
                let mut builder =
                    MmapGraphBuilder::<u16, Value, Value>::new(MmapGraphBuilderOptions::default());
                for (id, value) in node_props {
                    builder.set_node_with_props(id as u16, value.clone());
                }
                for ((s, p, o), value) in edges {
                    match value {
                        Some(value) => {
                            builder
                                .add_edge_with_props((s as u16, p as u16, o as u16), value.clone());
                        }
                        None => builder.add_edge((s as u16, p as u16, o as u16)),
                    };
                }
                return SmartMmapBuilder::U16(builder);
            } else if num_nodes < u32::MAX as usize && num_edges < u32::MAX as usize {
                // U32
                let mut builder = MmapGraphBuilder::new(MmapGraphBuilderOptions::default());
                for (id, value) in node_props {
                    builder.set_node_with_props(id as u32, value.clone());
                }
                for ((s, p, o), value) in edges {
                    match value {
                        Some(value) => {
                            builder
                                .add_edge_with_props((s as u32, p as u32, o as u32), value.clone());
                        }
                        None => builder.add_edge((s as u32, p as u32, o as u32)),
                    };
                }
                return SmartMmapBuilder::U32(builder);
            }
        }

        // U64
        let mut builder = MmapGraphBuilder::new(MmapGraphBuilderOptions::default());
        for (id, value) in node_props {
            builder.set_node_with_props(id as u64, value.clone());
        }
        for (triple, value) in edges {
            match value {
                Some(value) => {
                    builder.add_edge_with_props(triple, value.clone());
                }
                None => builder.add_edge(triple),
            };
        }
        SmartMmapBuilder::U64(builder)
    }
}
