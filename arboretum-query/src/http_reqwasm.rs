use crate::Error;
use reqwasm::http::Request;

pub struct HttpGraphQueryExecutor {
    endpoint: String,
}

impl HttpGraphQueryExecutor {
    pub async fn run(&self, query: &crate::GraphQuery) -> Result<crate::GraphQueryResponse, Error> {
        let response = Request::post(&self.endpoint)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&query).unwrap())
            .send()
            .await;
        Ok(response?.json().await?)
    }
}

impl From<reqwasm::Error> for Error {
    fn from(e: reqwasm::Error) -> Self {
        Error::Message(e.to_string())
    }
}
