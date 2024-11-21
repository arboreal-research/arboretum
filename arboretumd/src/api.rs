use std::{net::ToSocketAddrs, sync::Arc};

use actix_web::{
    post,
    web::{self},
    App, HttpServer, Responder,
};
use arboretum_query::{GraphQuery, GraphQueryExecutor};
use tracing::error;

#[derive(Clone)]
pub struct ApiServerState {
    query_executor: Arc<dyn GraphQueryExecutor + Sync>,
}

impl ApiServerState {
    pub fn new(query_executor: Arc<dyn GraphQueryExecutor + Sync>) -> Self {
        Self { query_executor }
    }
}

#[post("/api/query/")]
async fn api_query(
    request: web::Json<GraphQuery>,
    executor: web::Data<ApiServerState>,
) -> impl Responder {
    let data = request.0;

    error!(?data);
    let r = tokio::task::spawn_blocking(move || executor.query_executor.run_blocking(&data)).await;

    match r {
        Ok(r) => web::Json(r),
        Err(e) => web::Json(Err(arboretum_query::Error::Message(e.to_string()))),
    }
}

pub async fn api_server<U>(state: ApiServerState, bind_addr: U) -> Result<(), std::io::Error>
where
    U: ToSocketAddrs,
{
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(api_query)
    })
    .bind(bind_addr)?
    .run()
    .await
}
