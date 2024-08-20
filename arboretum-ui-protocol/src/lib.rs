use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Query {
    TestQuery,
}

impl Query {
    pub fn url(&self) -> String {
        match self {
            Query::TestQuery => "/api/test".into(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    QueryResults {
        nodes: Vec<(String, f32)>,
        edges: Vec<(usize, usize, String)>,
    },
}
