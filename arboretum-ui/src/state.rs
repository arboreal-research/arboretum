use force_graph::DefaultNodeIdx;
use force_graph::ForceGraph;
use force_graph::NodeData;
use force_graph::SimulationParameters;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

pub struct NodeProps {
    pub color: JsValue,
    pub radius: f64,
    pub name: String,
}

pub struct EdgeProps {
    pub color: JsValue,
    pub name: String,
}

pub struct State {
    // Force directed graph.
    pub graph: ForceGraph<NodeProps, EdgeProps>,
    pub last_frame_timestamp: f64,

    // View
    pub scale: f64,
    pub translate_x: f64,
    pub translate_y: f64,

    // Panning
    pub is_panning: bool,
    pub pan_start_x: f64,
    pub pan_start_y: f64,
    pub last_x: f64,
    pub last_y: f64,

    // Node Selection
    pub last_click: Option<(f64, f64)>,
    pub highlight_node: Option<DefaultNodeIdx>,
    pub highlight_color: JsValue,
}

impl State {
    pub fn new(timestamp: f64, highlight_color: JsValue) -> Self {
        Self {
            scale: 1.0,
            translate_x: 0.0,
            translate_y: 0.0,
            is_panning: false,
            pan_start_x: 0.0,
            pan_start_y: 0.0,
            last_x: 0.0,
            last_y: 0.0,
            last_click: None,
            highlight_node: None,
            graph: ForceGraph::new(SimulationParameters {
                force_charge: 12000.0,
                force_spring: 0.3,
                force_max: 250.0,
                node_speed: 500.0,
                damping_factor: 0.98,
            }),
            highlight_color,
            last_frame_timestamp: timestamp,
        }
    }

    pub fn reset(&mut self) {
        self.graph.clear();
        self.scale = 1.0;
        self.translate_x = 0.0;
        self.translate_y = 0.0;
        self.last_click = None;
        self.highlight_node = None;
    }

    pub fn add_node(&mut self, mass: f32, props: NodeProps) -> DefaultNodeIdx {
        self.graph.add_node(NodeData {
            x: (random() * 3000.0) as f32,
            y: (random() * 3000.0) as f32,
            user_data: props,
            is_anchor: false,
            mass,
        })
    }

    pub fn add_edge(&mut self, a: DefaultNodeIdx, b: DefaultNodeIdx, props: EdgeProps) {
        self.graph
            .add_edge(a, b, force_graph::EdgeData { user_data: props })
    }

    pub fn update(&mut self, dt: f64) {
        self.graph.update(0.03 as f32);

        // Update the highlighted node if one was clicked
        if let Some((click_x, click_y)) = self.last_click {
            let mut highlight_node = None;
            self.graph
                .visit_nodes(|node: &force_graph::Node<NodeProps>| {
                    let dist = ((node.x() as f64 - click_x).powi(2)
                        + (node.y() as f64 - click_y).powi(2))
                    .sqrt();
                    if dist < node.data.user_data.radius {
                        highlight_node = Some(node.index().clone());
                    }
                });
            self.highlight_node = highlight_node;
        }
    }
}
