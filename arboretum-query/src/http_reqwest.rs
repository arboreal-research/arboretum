use crate::{GraphQuery, GraphQueryExecutor, GraphQueryResponse};

pub struct HttpGraphQueryExecutor {
    endpoint: String,
}

impl HttpGraphQueryExecutor {
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }
}

impl GraphQueryExecutor for HttpGraphQueryExecutor {
    fn run_blocking(&self, query: &GraphQuery) -> anyhow::Result<GraphQueryResponse> {
        use reqwest::blocking::Client;
        let client = Client::new();
        let response = client.post(&self.endpoint).json(&query).send()?;
        Ok(response.json()?)
    }
}

#[cfg(feature = "http_reqwest_async")]
impl HttpGraphQueryExecutor {
    pub async fn run_async(&self, query: &GraphQuery) -> anyhow::Result<GraphQueryResponse> {
        use reqwest::Client;
        let client = Client::new();
        let response = client.post(&self.endpoint).json(&query).send().await?;
        Ok(response.json().await?)
    }
}
