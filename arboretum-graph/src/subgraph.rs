use crate::{mmap, SledGraph, SubgraphConfig};
use arboretum_core::{constant::*, merge_u64, split_u64, ArchivedValue, Prefix, Value};
use num::NumCast;
use rkyv::{Deserialize, Infallible};
use std::{collections::HashMap, path::Path};

#[derive(Debug)]
pub enum Subgraph {
    MmapGraph16 {
        graph_id: u32,
        mmap: mmap::MmapGraph<u16, Value, Value>,
    },
    MmapGraph32 {
        graph_id: u32,
        mmap: mmap::MmapGraph<u32, Value, Value>,
    },
    MmapGraph64 {
        mmap: mmap::MmapGraph<u64, Value, Value>,
    },
    SledGraph64 {
        g: SledGraph<Value, Value>,
    },
}

impl Subgraph {
    pub fn get_memory_usage(&self) -> anyhow::Result<usize> {
        match self {
            Subgraph::MmapGraph16 { mmap, .. } => mmap.get_memory_usage(),
            Subgraph::MmapGraph32 { mmap, .. } => mmap.get_memory_usage(),
            Subgraph::MmapGraph64 { mmap } => mmap.get_memory_usage(),
            Subgraph::SledGraph64 { g } => g.get_memory_usage(),
        }
    }

    pub fn from_config(subgraph_path: &Path, config: &SubgraphConfig) -> anyhow::Result<Self> {
        Ok(match config {
            SubgraphConfig::MmapGraph16 {
                subgraph_id: graph_id,
                rel_path: path,
            } => Subgraph::MmapGraph16 {
                graph_id: *graph_id,
                mmap: mmap::MmapGraph::from_file(subgraph_path.join(path))?,
            },
            SubgraphConfig::MmapGraph32 {
                subgraph_id: graph_id,
                rel_path: path,
            } => Subgraph::MmapGraph32 {
                graph_id: *graph_id,
                mmap: mmap::MmapGraph::from_file(subgraph_path.join(path))?,
            },
            SubgraphConfig::MmapGraph64 { rel_path: path } => Subgraph::MmapGraph64 {
                mmap: mmap::MmapGraph::from_file(subgraph_path.join(path))?,
            },
            SubgraphConfig::SledGraph64 { rel_path: path } => Subgraph::SledGraph64 {
                g: SledGraph::from_file(subgraph_path.join(path))?,
            },
        })
    }

    pub fn extend_with(
        &self,
        data: (HashMap<u64, Value>, HashMap<(u64, u64, u64), Option<Value>>),
    ) -> anyhow::Result<bool> {
        Ok(match self {
            Subgraph::SledGraph64 { g } => {
                g.extend_with(data)?;
                true
            }
            _ => false,
        })
    }

    pub fn impl_name(&self) -> String {
        match self {
            Subgraph::MmapGraph16 { .. } => SUBGRAPH_IMPL_MMAP16.to_string(),
            Subgraph::MmapGraph32 { .. } => SUBGRAPH_IMPL_MMAP32.to_string(),
            Subgraph::MmapGraph64 { .. } => SUBGRAPH_IMPL_MMAP64.to_string(),
            Subgraph::SledGraph64 { .. } => SUBGRAPH_IMPL_SLED64.to_string(),
        }
    }

    pub fn get_node_props(&self, id: u64) -> anyhow::Result<Option<Value>> {
        let (high_id, low_id) = split_u64(id);
        Ok(match self {
            Subgraph::MmapGraph16 { graph_id, mmap } => {
                if *graph_id == high_id {
                    mmap.get_node_props(low_id as u16)
                        .map(|ar: &ArchivedValue| -> Value {
                            <ArchivedValue as Deserialize<Value, rkyv::Infallible>>::deserialize(
                                ar,
                                &mut Infallible,
                            )
                            .unwrap()
                        })
                } else {
                    None
                }
            }

            Subgraph::MmapGraph32 { graph_id, mmap } => {
                if *graph_id == high_id {
                    mmap.get_node_props(low_id)
                        .map(|ar: &ArchivedValue| -> Value {
                            <ArchivedValue as Deserialize<Value, rkyv::Infallible>>::deserialize(
                                ar,
                                &mut Infallible,
                            )
                            .unwrap()
                        })
                } else {
                    None
                }
            }

            Subgraph::MmapGraph64 { mmap } => {
                mmap.get_node_props(id).map(|ar: &ArchivedValue| -> Value {
                    <ArchivedValue as Deserialize<Value, rkyv::Infallible>>::deserialize(
                        ar,
                        &mut Infallible,
                    )
                    .unwrap()
                })
            }

            Subgraph::SledGraph64 { g } => g.get_node_props(id)?,
        })
    }

    pub fn iter_node_props<'a>(
        &'a self,
    ) -> anyhow::Result<Box<dyn Iterator<Item = (u64, Value)> + 'a>> {
        Ok(match self {
            Subgraph::MmapGraph16 { graph_id, mmap } => {
                Box::new(mmap.iter_node_props().map(|(low_id, props)| {
                    (
                        merge_u64(*graph_id, *low_id as u32),
                        <ArchivedValue as Deserialize<Value, rkyv::Infallible>>::deserialize(
                            props,
                            &mut Infallible,
                        )
                        .unwrap(),
                    )
                }))
            }
            Subgraph::MmapGraph32 { graph_id, mmap } => {
                Box::new(mmap.iter_node_props().map(|(low_id, props)| {
                    (
                        merge_u64(*graph_id, *low_id),
                        <ArchivedValue as Deserialize<Value, rkyv::Infallible>>::deserialize(
                            props,
                            &mut Infallible,
                        )
                        .unwrap(),
                    )
                }))
            }
            Subgraph::MmapGraph64 { mmap } => {
                Box::new(mmap.iter_node_props().map(|(id, props)| {
                    (
                        *id,
                        <ArchivedValue as Deserialize<Value, rkyv::Infallible>>::deserialize(
                            props,
                            &mut Infallible,
                        )
                        .unwrap(),
                    )
                }))
            }
            Subgraph::SledGraph64 { g } => Box::new(g.iter_node_props()?),
        })
    }

    pub fn prefix_edges_spo<'a>(
        &'a self,
        prefix: Prefix<u64>,
    ) -> anyhow::Result<Box<dyn Iterator<Item = (u64, u64, u64, Option<Value>)> + 'a>> {
        Ok(match self {
            Subgraph::MmapGraph16 { graph_id, mmap } => Box::new(
                mmap.prefix_edges_spo(prefix.into())
                    .map(|edge| rebuild_edge(graph_id, edge)),
            ),
            Subgraph::MmapGraph32 { graph_id, mmap } => Box::new(
                mmap.prefix_edges_spo(prefix.into())
                    .map(|edge| rebuild_edge(graph_id, edge)),
            ),
            Subgraph::MmapGraph64 { mmap } => {
                Box::new(mmap.prefix_edges_spo(prefix).map(|(a, b, c, e)| {
                    (
                        *a,
                        *b,
                        *c,
                        e.map(|e| {
                            <ArchivedValue as Deserialize<Value, rkyv::Infallible>>::deserialize(
                                e,
                                &mut Infallible,
                            )
                            .unwrap()
                        }),
                    )
                }))
            }
            Subgraph::SledGraph64 { g } => Box::new(g.prefix_edges_spo(prefix)?),
        })
    }

    pub fn prefix_edges_pos<'a>(
        &'a self,
        prefix: Prefix<u64>,
    ) -> anyhow::Result<Box<dyn Iterator<Item = (u64, u64, u64, Option<Value>)> + 'a>> {
        Ok(match self {
            Subgraph::MmapGraph16 { graph_id, mmap } => Box::new(
                mmap.prefix_edges_pos(prefix.into())
                    .map(|edge| rebuild_edge(graph_id, edge)),
            ),

            Subgraph::MmapGraph32 { graph_id, mmap } => Box::new(
                mmap.prefix_edges_pos(prefix.into())
                    .map(|edge| rebuild_edge(graph_id, edge)),
            ),

            Subgraph::MmapGraph64 { mmap } => {
                Box::new(mmap.prefix_edges_pos(prefix).map(|(a, b, c, e)| {
                    (
                        *a,
                        *b,
                        *c,
                        e.map(|e| {
                            <ArchivedValue as Deserialize<Value, rkyv::Infallible>>::deserialize(
                                e,
                                &mut Infallible,
                            )
                            .unwrap()
                        }),
                    )
                }))
            }
            Subgraph::SledGraph64 { g } => Box::new(g.prefix_edges_pos(prefix)?),
        })
    }

    pub fn prefix_edges_osp<'a>(
        &'a self,
        prefix: Prefix<u64>,
    ) -> anyhow::Result<Box<dyn Iterator<Item = (u64, u64, u64, Option<Value>)> + 'a>> {
        Ok(match self {
            Subgraph::MmapGraph16 { graph_id, mmap } => Box::new(
                mmap.prefix_edges_osp(prefix.into())
                    .map(|edge| rebuild_edge(graph_id, edge)),
            ),
            Subgraph::MmapGraph32 { graph_id, mmap } => Box::new(
                mmap.prefix_edges_osp(prefix.into())
                    .map(|edge| rebuild_edge(graph_id, edge)),
            ),
            Subgraph::MmapGraph64 { mmap } => {
                Box::new(mmap.prefix_edges_osp(prefix).map(|(a, b, c, e)| {
                    (
                        *a,
                        *b,
                        *c,
                        e.map(|e| {
                            <ArchivedValue as Deserialize<Value, rkyv::Infallible>>::deserialize(
                                e,
                                &mut Infallible,
                            )
                            .unwrap()
                        }),
                    )
                }))
            }
            Subgraph::SledGraph64 { g } => Box::new(g.prefix_edges_osp(prefix)?),
        })
    }
}

fn rebuild_edge<I: NumCast>(
    graph_id: &u32,
    edge: (&I, &I, &I, Option<&ArchivedValue>),
) -> (u64, u64, u64, Option<Value>) {
    (
        merge_u64(*graph_id, edge.0.to_u32().unwrap()),
        merge_u64(*graph_id, edge.1.to_u32().unwrap()),
        merge_u64(*graph_id, edge.2.to_u32().unwrap()),
        edge.3.map(|e| {
            <ArchivedValue as Deserialize<Value, rkyv::Infallible>>::deserialize(e, &mut Infallible)
                .unwrap()
        }),
    )
}
