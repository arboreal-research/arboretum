use actix::{Actor, Addr, Context};

use crate::graph::GraphActor;

pub struct ClangAnnotationActor {
    graph_addr: Addr<GraphActor>,
}

impl ClangAnnotationActor {
    pub fn new(graph_addr: Addr<GraphActor>) -> Self {
        Self { graph_addr }
    }
}

impl Actor for ClangAnnotationActor {
    type Context = Context<Self>;
}
