use crate::{Error, GraphQuery, GraphQueryExecutor, GraphQueryResponse};

pub struct HttpGraphQueryExecutor {
    endpoint: String,
}

impl GraphQueryExecutor for HttpGraphQueryExecutor {
    fn run_blocking(&self, query: &GraphQuery) -> Result<GraphQueryResponse, Error> {
        use reqwest::blocking::Client;
        let client = Client::new();
        let response = client.post(&self.endpoint).json(&query).send()?;
        Ok(response.json()?)
    }
}

#[cfg(feature = "http_reqwest_async")]
impl HttpGraphQueryExecutor {
    pub async fn run_async(&self, query: &GraphQuery) -> Result<GraphQueryResponse, Error> {
        use reqwest::Client;
        let client = Client::new();
        let response = client.post(&self.endpoint).json(&query).send().await?;
        Ok(response.json().await?)
    }
}

#[cfg(feature = "http_reqwest")]
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Message(e.to_string())
    }
}
