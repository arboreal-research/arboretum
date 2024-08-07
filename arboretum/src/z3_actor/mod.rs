use actix::{Actor, Addr, Context};

use crate::graph::GraphActor;

pub struct Z3Actor {
    graph_addr: Addr<GraphActor>,
}

impl Z3Actor {
    pub fn new(graph_addr: Addr<GraphActor>) -> Self {
        Self { graph_addr }
    }
}

impl Actor for Z3Actor {
    type Context = Context<Self>;
}
