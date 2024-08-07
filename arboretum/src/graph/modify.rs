use std::collections::{HashMap, HashSet};

use actix::{Handler, Message};
use simple_triplestore::{prelude::*, rdf::Entity, Triple};

use super::{EdgeProps, GraphAction, GraphActor, GraphError, GraphEvent, NodeProps};

pub struct GraphModifyRequest {
    pub(crate) add_nodes: Vec<(Entity, NodeProps)>,
    pub(crate) del_nodes: Vec<(Entity)>,
    pub(crate) add_edges: Vec<(Triple<Entity>, EdgeProps)>,
    pub(crate) del_edges: Vec<Triple<Entity>>,
}

impl GraphModifyRequest {
    pub fn new() -> Self {
        Self {
            add_nodes: Vec::new(),
            del_nodes: Vec::new(),
            add_edges: Vec::new(),
            del_edges: Vec::new(),
        }
    }

    pub fn add_node(mut self, entity: Entity, props: NodeProps) -> Self {
        self.add_nodes.push((entity, props));
        self
    }

    pub fn del_node(mut self, entity: Entity) -> Self {
        self.del_nodes.push(entity);
        self
    }

    pub fn add_edge(mut self, triple: Triple<Entity>, props: NodeProps) -> Self {
        self.add_edges.push((triple, props));
        self
    }

    pub fn del_edge(mut self, triple: Triple<Entity>) -> Self {
        self.del_edges.push(triple);
        self
    }
}

impl Message for GraphModifyRequest {
    type Result = Result<(), GraphError>;
}

impl Handler<GraphModifyRequest> for GraphActor {
    type Result = <GraphModifyRequest as Message>::Result;
    fn handle(&mut self, msg: GraphModifyRequest, _ctx: &mut Self::Context) -> Self::Result {
        for node in msg.del_nodes {
            self.db.remove_node(node).map_err(|_| GraphError)?;
        }

        for (id, props) in msg.add_nodes {
            self.db.insert_node(id, props).map_err(|_| GraphError)?;
        }

        for triple in msg.del_edges {
            for sub in self.find_matching_subs(&triple) {
                sub.do_send(GraphEvent {
                    triple: triple.clone(),
                    action: GraphAction::Del,
                });
            }

            self.db.remove_edge(triple).map_err(|_| GraphError)?;
        }

        for (triple, props) in msg.add_edges {
            for sub in self.find_matching_subs(&triple) {
                sub.do_send(GraphEvent {
                    triple: triple.clone(),
                    action: GraphAction::Add,
                });
            }

            self.db.insert_edge(triple, props).map_err(|_| GraphError)?;
        }

        Ok(())
    }
}
