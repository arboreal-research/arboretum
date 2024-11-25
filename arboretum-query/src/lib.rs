use arboretum_core::{Domain, Prefix, Value};
use serde::{Deserialize, Serialize};

mod path;
pub use path::*;

#[cfg(feature = "http_reqwest")]
pub mod http_reqwest;

#[cfg(feature = "http_reqwasm")]
pub mod http_reqwasm;

#[cfg(feature = "local")]
pub mod local;

pub trait GraphQueryExecutor: Send + Sync {
    fn run_blocking(&self, query: &GraphQuery) -> Result<GraphQueryResponse, Error>;
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GraphQuery {
    SPO(Prefix<u64>),
    POS(Prefix<u64>),
    OSP(Prefix<u64>),

    GlobalSPO(Prefix<u64>),
    GlobalPOS(Prefix<u64>),
    GlobalOSP(Prefix<u64>),

    ExtraSPO(Prefix<u64>, Vec<Domain>),
    ExtraPOS(Prefix<u64>, Vec<Domain>),
    ExtraOSP(Prefix<u64>, Vec<Domain>),

    NodeProps(u64),
    NodeName(u64),
    NodeId(String),

    RegularPathQuery(Path),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GraphQueryResponse {
    Edges(Vec<(u64, u64, u64, Option<Value>)>),
    NodeProps(Option<Value>),
    NodeName(Option<String>),
    NodeId(Option<u64>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Error {
    Message(String),
}
