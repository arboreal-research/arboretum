use std::{net::ToSocketAddrs, sync::Arc};

use actix_web::{
    post,
    web::{self},
    App, HttpServer, Responder,
};
use arboretum_query::{GraphQuery, GraphQueryExecutor};

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
async fn test_query(
    request: web::Json<GraphQuery>,
    executor: web::Data<ApiServerState>,
) -> impl Responder {
    web::Json(executor.query_executor.run(&request.0))
}

pub async fn api_server<U>(state: ApiServerState, bind_addr: U) -> Result<(), std::io::Error>
where
    U: ToSocketAddrs,
{
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(test_query)
    })
    .bind(bind_addr)?
    .run()
    .await
}
