use reqwasm::http::Request;

pub struct HttpGraphQueryExecutor {
    endpoint: String,
}

impl HttpGraphQueryExecutor {
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }

    pub async fn run(
        &self,
        query: &crate::GraphQuery,
    ) -> anyhow::Result<crate::GraphQueryResponse> {
        let response = Request::post(&self.endpoint)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&query).unwrap())
            .send()
            .await;

        let response: anyhow::Result<Result<crate::GraphQueryResponse, String>, reqwasm::Error> =
            response?.json().await;

        match response {
            Ok(r) => r.map_err(|e| anyhow::anyhow!(e)),
            Err(e) => Err(anyhow::anyhow!(e)),
        }
    }
}
