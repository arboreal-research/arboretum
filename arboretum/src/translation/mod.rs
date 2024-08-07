use actix::{Actor, Addr, Context};

use crate::graph::GraphActor;

pub struct ReactiveTranslationActor {
    graph_addr: Addr<GraphActor>,
}

impl ReactiveTranslationActor {
    pub fn new(graph_addr: Addr<GraphActor>) -> Self {
        Self { graph_addr }
    }
}

impl Actor for ReactiveTranslationActor {
    type Context = Context<Self>;
}
