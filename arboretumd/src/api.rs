use std::net::ToSocketAddrs;

use actix_web::{
    web::{self},
    App, HttpServer,
};

#[derive(Clone)]
pub struct ApiServerState {}

impl ApiServerState {
    pub fn new() -> Self {
        Self {}
    }
}

pub async fn api_server<U>(state: ApiServerState, bind_addr: U) -> Result<(), std::io::Error>
where
    U: ToSocketAddrs,
{
    HttpServer::new(move || App::new().app_data(web::Data::new(state.clone())))
        .bind(bind_addr)?
        .run()
        .await
}
