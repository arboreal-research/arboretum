use std::collections::HashMap;

use actix::{Actor, Context, Handler, Message, Recipient};
use simple_triplestore::{
    mem::MemHashIndex,
    prelude::*,
    rdf::{Entity, RdfTripleStore},
    MemTripleStore, Triple, UlidIdGenerator,
};
use ulid::Ulid;

mod modify;
pub use modify::*;

mod read;
pub use read::*;

mod subscribe;
pub use subscribe::*;

pub type NodeProps = ();
pub type EdgeProps = ();

pub type GraphTripleStore = RdfTripleStore<
    NodeProps,
    EdgeProps,
    MemHashIndex<String, Ulid>,
    MemTripleStore<Ulid, NodeProps, EdgeProps>,
>;

#[derive(Debug)]
pub struct GraphError;

pub struct GraphActor {
    db: GraphTripleStore,
    s_subs: HashMap<
        Entity,
        (
            Vec<Recipient<GraphEvent>>,
            HashMap<Entity, Vec<Recipient<GraphEvent>>>,
            HashMap<Entity, Vec<Recipient<GraphEvent>>>,
        ),
    >,
    p_subs: HashMap<
        Entity,
        (
            Vec<Recipient<GraphEvent>>,
            HashMap<Entity, Vec<Recipient<GraphEvent>>>,
        ),
    >,
    o_subs: HashMap<Entity, Vec<Recipient<GraphEvent>>>,
}

impl GraphActor {
    pub fn new() -> Result<Self, GraphError> {
        let index = MemHashIndex::new();
        let triplestore = MemTripleStore::new(UlidIdGenerator::new());

        Ok(Self {
            db: GraphTripleStore::new(index, triplestore),
            s_subs: HashMap::new(),
            p_subs: HashMap::new(),
            o_subs: HashMap::new(),
        })
    }

    fn find_matching_subs(&self, triple: &Triple<Entity>) -> Vec<&Recipient<GraphEvent>> {
        let mut result = Vec::new();

        if let Some((v, sp_subs, so_subs)) = self.s_subs.get(&triple.sub) {
            result.extend(v);

            if let Some(v) = sp_subs.get(&triple.pred) {
                result.extend(v);
            }
            if let Some(v) = so_subs.get(&triple.obj) {
                result.extend(v);
            }
        }
        if let Some((v, po_subs)) = self.p_subs.get(&triple.pred) {
            result.extend(v);

            if let Some(v) = po_subs.get(&triple.obj) {
                result.extend(v);
            }
        }
        if let Some(v) = self.o_subs.get(&triple.obj) {
            result.extend(v);
        }

        result
    }
}

impl Actor for GraphActor {
    type Context = Context<Self>;
}

#[cfg(test)]
mod test {
    use actix::{Actor, System};
    use simple_triplestore::{query, rdf::Entity, traits::TripleStoreIter, Triple};
    use ulid::Ulid;

    use crate::graph::GraphActor;

    use super::{GraphModifyRequest, GraphReadRequest};

    #[actix::test]
    async fn test_insert_remove() {
        let graph_actor = GraphActor::new().unwrap().start();

        let result = graph_actor
            .send(GraphReadRequest(query! { [Entity::Ulid(Ulid(1))] -?-> ? }))
            .await
            .unwrap()
            .unwrap();

        assert_eq!(
            result
                .iter_edges(simple_triplestore::EdgeOrder::SPO)
                .map(|e| e.unwrap())
                .collect::<Vec<_>>(),
            [].to_vec()
        );

        graph_actor.do_send(
            GraphModifyRequest::new()
                .add_node(Entity::Ulid(Ulid(1)), ())
                .add_node(Entity::Ulid(Ulid(2)), ())
                .add_edge(
                    Triple {
                        sub: Entity::Ulid(Ulid(1)),
                        pred: Entity::Ulid(Ulid(123)),
                        obj: Entity::Ulid(Ulid(2)),
                    },
                    (),
                ),
        );

        let result = graph_actor
            .send(GraphReadRequest(query! { [Entity::Ulid(Ulid(1))] -?-> ? }))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            result
                .iter_edges(simple_triplestore::EdgeOrder::SPO)
                .map(|e| e.unwrap())
                .collect::<Vec<_>>(),
            [(
                Triple {
                    sub: Entity::Ulid(Ulid(1)),
                    pred: Entity::Ulid(Ulid(123)),
                    obj: Entity::Ulid(Ulid(2)),
                },
                ()
            )]
            .to_vec()
        );

        graph_actor.do_send(GraphModifyRequest::new().del_edge(Triple {
            sub: Entity::Ulid(Ulid(1)),
            pred: Entity::Ulid(Ulid(123)),
            obj: Entity::Ulid(Ulid(2)),
        }));

        let result = graph_actor
            .send(GraphReadRequest(query! { [Entity::Ulid(Ulid(1))] -?-> ? }))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            result
                .iter_edges(simple_triplestore::EdgeOrder::SPO)
                .map(|e| e.unwrap())
                .collect::<Vec<_>>(),
            [].to_vec()
        );
    }
}
