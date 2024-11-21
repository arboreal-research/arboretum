use crate::{Error, GraphQuery, GraphQueryExecutor, GraphQueryResponse};
use arboretum_core::Value;
use arboretum_graph::RootGraph;

pub struct LocalGraphQueryExecutor {
    graph: RootGraph,
}

impl LocalGraphQueryExecutor {
    pub fn new(graph: RootGraph) -> Self {
        LocalGraphQueryExecutor { graph }
    }
}

fn rewrite_edges(v: Vec<(u64, u64, u64, Option<Value>)>) -> GraphQueryResponse {
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

            GraphQuery::GlobalSPO(prefix) => self
                .graph
                .prefix_edges_spo_global(prefix.clone())
                .map(rewrite_edges)?,
            GraphQuery::GlobalPOS(prefix) => self
                .graph
                .prefix_edges_pos_global(prefix.clone())
                .map(rewrite_edges)?,
            GraphQuery::GlobalOSP(prefix) => self
                .graph
                .prefix_edges_osp_global(prefix.clone())
                .map(rewrite_edges)?,

            GraphQuery::ExtraSPO(prefix, domains) => self
                .graph
                .prefix_edges_spo_with_extra_domains(prefix.clone(), domains.clone())
                .map(rewrite_edges)?,
            GraphQuery::ExtraPOS(prefix, domains) => self
                .graph
                .prefix_edges_pos_with_extra_domains(prefix.clone(), domains.clone())
                .map(rewrite_edges)?,
            GraphQuery::ExtraOSP(prefix, domains) => self
                .graph
                .prefix_edges_osp_with_extra_domains(prefix.clone(), domains.clone())
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

impl From<arboretum_graph::Error> for Error {
    fn from(e: arboretum_graph::Error) -> Self {
        Error::Message(format!("{:?}", e))
    }
}
