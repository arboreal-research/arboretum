use actix::{Actor, Addr, Context};

use crate::graph::GraphActor;

pub struct NetworkingActor {
    graph_addr: Addr<GraphActor>,
}

impl NetworkingActor {
    pub fn new(graph_addr: Addr<GraphActor>) -> Self {
        Self { graph_addr }
    }
}

impl Actor for NetworkingActor {
    type Context = Context<Self>;
}
