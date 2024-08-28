use arboretum_graph::{Prefix, Value};
use serde::{Deserialize, Serialize};

#[cfg(feature = "http_reqwest")]
pub mod http_reqwest;

#[cfg(feature = "http_reqwasm")]
pub mod http_reqwasm;

pub mod local;

pub trait GraphQueryExecutor: Send + Sync {
    fn run_blocking(&self, query: &GraphQuery) -> Result<GraphQueryResponse, Error>;
}

#[derive(Serialize, Deserialize)]
pub enum GraphQuery {
    SPO(Prefix<u64>),
    POS(Prefix<u64>),
    OSP(Prefix<u64>),
    NodeProps(u64),
    NodeName(u64),
    NodeId(String),
}

#[derive(Serialize, Deserialize)]
pub enum GraphQueryResponse {
    Edges(Vec<(u64, u64, u64, Option<Value>)>),
    NodeProps(Option<Value>),
    NodeName(Option<String>),
    NodeId(Option<u64>),
}

#[derive(Serialize, Deserialize)]
pub enum Error {
    Message(String),
}

impl From<arboretum_graph::Error> for Error {
    fn from(e: arboretum_graph::Error) -> Self {
        Error::Message(format!("{:?}", e))
    }
}
