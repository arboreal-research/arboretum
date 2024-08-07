use actix::{Actor, Addr, AsyncContext, Context, Handler};
use simple_triplestore::rdf::Entity;

use crate::graph::{GraphActor, GraphEvent, GraphSubscribeRequest};

pub struct RustAnnotationActor {
    graph_addr: Addr<GraphActor>,
}

impl RustAnnotationActor {
    pub fn new(graph_addr: Addr<GraphActor>) -> Self {
        Self { graph_addr }
    }
}

impl Actor for RustAnnotationActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Subscribe to subjects which have the clang::TranslationUnit class.
        self.graph_addr.do_send(GraphSubscribeRequest {
            sub: crate::graph::GraphSubscription::PO(
                Entity::String("/meta/class".into()),
                Entity::String("/clang/TranslationUnitDecl".into()),
            ),
            callback: ctx.address().recipient(),
        })

        // Search for
    }
}

impl Handler<GraphEvent> for RustAnnotationActor {
    type Result = ();
    fn handle(&mut self, msg: GraphEvent, ctx: &mut Self::Context) -> Self::Result {}
}
