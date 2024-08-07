use actix::{Actor, Addr, Context};

use crate::graph::GraphActor;

pub struct RustSyntaxActor {
    graph_addr: Addr<GraphActor>,
}

impl RustSyntaxActor {
    pub fn new(graph_addr: Addr<GraphActor>) -> Self {
        Self { graph_addr }
    }
}

impl Actor for RustSyntaxActor {
    type Context = Context<Self>;
}
