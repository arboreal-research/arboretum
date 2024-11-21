use crate::Error;
use reqwasm::http::Request;

pub struct HttpGraphQueryExecutor {
    endpoint: String,
}

impl HttpGraphQueryExecutor {
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }

    pub async fn run(&self, query: &crate::GraphQuery) -> Result<crate::GraphQueryResponse, Error> {
        let response = Request::post(&self.endpoint)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&query).unwrap())
            .send()
            .await;

        let response: Result<Result<crate::GraphQueryResponse, Error>, reqwasm::Error> =
            response?.json().await;

        match response {
            Ok(r) => r,
            Err(e) => Err(Error::Message(e.to_string())),
        }
    }
}

impl From<reqwasm::Error> for Error {
    fn from(e: reqwasm::Error) -> Self {
        Error::Message(e.to_string())
    }
}
