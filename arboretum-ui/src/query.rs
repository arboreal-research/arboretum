// use std::{cell::RefCell, rc::Rc};

// use arboretum_query::{GraphQuery, GraphQueryResponse};
// // use arboretum_query::{GraphQueryQuery, GraphQueryResponseResponse};
// use serde_wasm_bindgen::from_value;
// use wasm_bindgen::{JsCast, JsValue};
// use wasm_bindgen_futures::JsFuture;
// use web_sys::Window;

// use crate::{EdgeProps, NodeProps, State};

// pub fn fetch_full_file_listing(window: Window, state: Rc<RefCell<State>>) {}

// pub fn fetch_compiler_invocations(window: Window, state: Rc<RefCell<State>>) {}

// pub fn fetch_invocation_decl_hierarchy(
//     window: Window,
//     state: Rc<RefCell<State>>,
//     invocation_id: u64,
// ) {
// }
// pub fn fetch_invocation_args(window: Window, state: Rc<RefCell<State>>, invocation_id: u64) {}
// pub fn fetch_invocation_file_listing(
//     window: Window,
//     state: Rc<RefCell<State>>,
//     invocation_id: u64,
// ) {
// }

// pub fn run_query(window: Window, query: GraphQuery, state: Rc<RefCell<State>>) {
//     wasm_bindgen_futures::spawn_local(async move {
//         let response: web_sys::Response =
//             JsFuture::from(window.fetch_with_str("http://127.0.0.1:3000/api/query/"))
//                 .await
//                 .unwrap()
//                 .dyn_into()
//                 .unwrap();
//         let response: GraphQueryResponse =
//             from_value(JsFuture::from(response.json().unwrap()).await.unwrap()).unwrap();

//         match response {
//             GraphQueryResponse::Edges(vec) => todo!(),
//             GraphQueryResponse::NodeProps(value) => todo!(),
//             GraphQueryResponse::NodeName(_) => todo!(),
//             GraphQueryResponse::NodeId(_) => todo!(),
// GraphQueryResponse::QueryResults { nodes, edges } => {
//     let light_green = JsValue::from_str("rgb(114, 151, 98)");
//     let dark_green = JsValue::from_str("rgb(50, 64, 38)");

//     let mut state = state.borrow_mut();

//     let nodes = nodes
//         .iter()
//         .map(|(name, mass)| {
//             state.add_node(
//                 *mass,
//                 NodeProps {
//                     color: light_green.clone(),
//                     radius: 25.0,
//                     name: name.to_string(),
//                 },
//             )
//         })
//         .collect::<Vec<_>>();

//     for (a, b, name) in edges {
//         state.add_edge(
//             nodes[a],
//             nodes[b],
//             EdgeProps {
//                 color: dark_green.clone(),
//                 name: name.to_string(),
//             },
//         );
//     }
// }
//         }
//     });
// }
