use actix::{Handler, Message};
use simple_triplestore::{rdf::Entity, traits::TripleStoreQuery, Query};

use super::{EdgeProps, GraphActor, GraphError, GraphTripleStore, NodeProps};

pub struct GraphReadRequest(pub Query<Entity>);

impl Message for GraphReadRequest {
    type Result = Result<
        <GraphTripleStore as TripleStoreQuery<Entity, NodeProps, EdgeProps>>::QueryResult,
        GraphError,
    >;
}

impl Handler<GraphReadRequest> for GraphActor {
    type Result = <GraphReadRequest as Message>::Result;
    fn handle(&mut self, msg: GraphReadRequest, ctx: &mut Self::Context) -> Self::Result {
        self.db.run(msg.0).map_err(|_| GraphError)
    }
}
