use actix::{Actor, Addr, Context};

use crate::graph::GraphActor;

pub struct ModuleLayoutActor {
    graph_addr: Addr<GraphActor>,
}

impl ModuleLayoutActor {
    pub fn new(graph_addr: Addr<GraphActor>) -> Self {
        Self { graph_addr }
    }
}

impl Actor for ModuleLayoutActor {
    type Context = Context<Self>;
}
