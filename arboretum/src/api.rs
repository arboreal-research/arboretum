use std::{
    collections::{hash_map::Entry, HashMap, HashSet, VecDeque},
    net::ToSocketAddrs,
};

use actix_web::{get, web, App, HttpServer, Responder};
use arboretum_graph::{query, RootGraph};
use arboretum_ui_protocol::{Query, Response};

#[get("/test")]
async fn test_query(graph: web::Data<RootGraph>) -> impl Responder {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    if let Some(root) = graph.get_named_node("clang::Decl").await.unwrap() {
        let mut q = VecDeque::from([(root, 7u32)]);

        nodes.push(("clang::Decl".into(), 4.0));

        let mut seen_nodes = HashMap::from([(root, 0)]);
        let mut seen_predicates: HashMap<u64, String> = HashMap::from([]);

        let mut visited = HashSet::new();
        while !q.is_empty() {
            let (id, ttl) = q.pop_front().unwrap();
            for (s, p, o, _) in query! {graph, id -?-> ?}.await.unwrap() {
                if ttl > 0 {
                    if visited.insert(o) {
                        q.push_back((o, ttl - 1));
                    }
                }

                let s = match seen_nodes.entry(s) {
                    Entry::Occupied(o) => *o.get(),
                    Entry::Vacant(v) => {
                        let idx = nodes.len();
                        v.insert(idx);
                        nodes.push((
                            graph
                                .get_node_name(s)
                                .await
                                .unwrap()
                                .unwrap_or_else(|| format!("{}", s)),
                            (10 * (7 - ttl + 1)) as f32,
                        ));
                        idx
                    }
                };

                let o = match seen_nodes.entry(o) {
                    Entry::Occupied(o) => *o.get(),
                    Entry::Vacant(v) => {
                        let idx = nodes.len();
                        v.insert(idx);
                        nodes.push((
                            graph
                                .get_node_name(o)
                                .await
                                .unwrap()
                                .unwrap_or_else(|| format!("{}", o)),
                            (10 * (7 - ttl + 1)) as f32,
                        ));
                        idx
                    }
                };

                let p = match seen_predicates.entry(p) {
                    Entry::Occupied(o) => o.get().clone(),
                    Entry::Vacant(v) => {
                        let name = graph
                            .get_node_name(p)
                            .await
                            .unwrap()
                            .unwrap_or_else(|| format!("{}", p));
                        v.insert(name.clone());
                        name
                    }
                };

                edges.push((s, o, p));
            }
        }
    }

    let result = Response::QueryResults { nodes, edges };

    web::Json(result)
}

pub async fn api_server<U>(graph: RootGraph, bind_addr: U) -> Result<(), std::io::Error>
where
    U: ToSocketAddrs,
{
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(graph.clone()))
            .service(test_query)
    })
    .bind(bind_addr)?
    .run()
    .await
}
