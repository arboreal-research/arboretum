use arboretum_graph::RootGraph;

use crate::{Error, GraphQuery, GraphQueryExecutor, GraphQueryResponse};

pub struct LocalGraphQueryExecutor {
    graph: RootGraph,
}

impl LocalGraphQueryExecutor {
    pub fn new(graph: RootGraph) -> Self {
        LocalGraphQueryExecutor { graph }
    }
}

fn rewrite_edges(v: Vec<(u64, u64, u64, Option<arboretum_graph::Value>)>) -> GraphQueryResponse {
    GraphQueryResponse::Edges(
        v.into_iter()
            .map(|(s, p, o, v)| (s, p, o, v.map(|v| v.into())))
            .collect(),
    )
}

impl GraphQueryExecutor for LocalGraphQueryExecutor {
    fn run_blocking(&self, query: &GraphQuery) -> Result<GraphQueryResponse, Error> {
        Ok(match query {
            GraphQuery::SPO(prefix) => self
                .graph
                .prefix_edges_spo(prefix.clone())
                .map(rewrite_edges)?,
            GraphQuery::POS(prefix) => self
                .graph
                .prefix_edges_pos(prefix.clone())
                .map(rewrite_edges)?,
            GraphQuery::OSP(prefix) => self
                .graph
                .prefix_edges_osp(prefix.clone())
                .map(rewrite_edges)?,
            GraphQuery::NodeProps(id) => {
                GraphQueryResponse::NodeProps(self.graph.get_node_props(*id)?.map(|v| v.into()))
            }
            GraphQuery::NodeName(id) => {
                GraphQueryResponse::NodeName(self.graph.get_node_name(*id)?)
            }
            GraphQuery::NodeId(name) => {
                GraphQueryResponse::NodeId(self.graph.get_named_node(name)?)
            }
        })
    }
}
