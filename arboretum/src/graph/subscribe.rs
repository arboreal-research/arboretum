use std::collections::HashMap;

use actix::{Handler, Message, Recipient};
use simple_triplestore::{rdf::Entity, Triple};

use super::GraphActor;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum GraphSubscription {
    S(Entity),
    SP(Entity, Entity),
    SO(Entity, Entity),
    P(Entity),
    PO(Entity, Entity),
    O(Entity),
}

pub enum GraphAction {
    Add,
    Del,
}

pub struct GraphSubscribeRequest {
    pub sub: GraphSubscription,
    pub callback: Recipient<GraphEvent>,
}

impl Message for GraphSubscribeRequest {
    type Result = ();
}

pub struct GraphEvent {
    pub(crate) triple: Triple<Entity>,
    pub(crate) action: GraphAction,
}

impl Message for GraphEvent {
    type Result = ();
}

impl Handler<GraphSubscribeRequest> for GraphActor {
    type Result = <GraphSubscribeRequest as Message>::Result;
    fn handle(
        &mut self,
        graph_sub: GraphSubscribeRequest,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        match graph_sub.sub {
            GraphSubscription::S(sub) => match self.s_subs.entry(sub) {
                std::collections::hash_map::Entry::Occupied(mut o) => {
                    o.get_mut().0.push(graph_sub.callback);
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert((
                        Vec::from([graph_sub.callback]),
                        HashMap::new(),
                        HashMap::new(),
                    ));
                }
            },
            GraphSubscription::SP(sub, pred) => match self.s_subs.entry(sub) {
                std::collections::hash_map::Entry::Occupied(mut o) => {
                    match o.get_mut().1.entry(pred) {
                        std::collections::hash_map::Entry::Occupied(mut o) => {
                            o.get_mut().push(graph_sub.callback);
                        }
                        std::collections::hash_map::Entry::Vacant(v) => {
                            v.insert(Vec::from([graph_sub.callback]));
                        }
                    }
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert((
                        Vec::new(),
                        HashMap::from([(pred, Vec::from([graph_sub.callback]))]),
                        HashMap::new(),
                    ));
                }
            },
            GraphSubscription::SO(sub, obj) => match self.s_subs.entry(sub) {
                std::collections::hash_map::Entry::Occupied(mut o) => {
                    match o.get_mut().1.entry(obj) {
                        std::collections::hash_map::Entry::Occupied(mut o) => {
                            o.get_mut().push(graph_sub.callback);
                        }
                        std::collections::hash_map::Entry::Vacant(v) => {
                            v.insert(Vec::from([graph_sub.callback]));
                        }
                    }
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert((
                        Vec::new(),
                        HashMap::new(),
                        HashMap::from([(obj, Vec::from([graph_sub.callback]))]),
                    ));
                }
            },
            GraphSubscription::P(pred) => match self.p_subs.entry(pred) {
                std::collections::hash_map::Entry::Occupied(mut o) => {
                    o.get_mut().0.push(graph_sub.callback);
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert((Vec::from([graph_sub.callback]), HashMap::new()));
                }
            },
            GraphSubscription::PO(pred, obj) => match self.p_subs.entry(pred) {
                std::collections::hash_map::Entry::Occupied(mut o) => {
                    match o.get_mut().1.entry(obj) {
                        std::collections::hash_map::Entry::Occupied(mut o) => {
                            o.get_mut().push(graph_sub.callback);
                        }
                        std::collections::hash_map::Entry::Vacant(v) => {
                            v.insert(Vec::from([graph_sub.callback]));
                        }
                    }
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert((
                        Vec::new(),
                        HashMap::from([(obj, Vec::from([graph_sub.callback]))]),
                    ));
                }
            },
            GraphSubscription::O(obj) => match self.o_subs.entry(obj) {
                std::collections::hash_map::Entry::Occupied(mut o) => {
                    o.get_mut().push(graph_sub.callback);
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert(Vec::from([graph_sub.callback]));
                }
            },
        }
    }
}
