use crate::ViewTransform;
use force_graph::{DefaultNodeIdx, ForceGraph, NodeData, SimulationParameters};
use std::f64::consts::PI;
use wasm_bindgen::JsValue;
use web_sys::{js_sys::Math::random, CanvasRenderingContext2d, Window};

pub struct NodeProps {
    pub color: String,
    pub radius: f64,
    pub name: String,
}

pub struct EdgeProps {
    pub color: String,
    pub name: String,
}

pub struct ForceDirectedViewMode {
    pub view_transform: ViewTransform,

    // Force directed graph.
    pub graph: ForceGraph<NodeProps, EdgeProps>,
    pub last_frame_timestamp: f64,

    // Node Selection
    pub highlight_node: Option<DefaultNodeIdx>,
    pub highlight_color: String,
}

impl ForceDirectedViewMode {
    pub fn setup(&mut self, window: &Window) {
        self.last_frame_timestamp = window.performance().unwrap().now();

        let light_green = "rgb(114, 151, 98)";
        let dark_green = "rgb(50, 64, 38)";
        let a = self.add_node(
            100.0,
            NodeProps {
                color: light_green.to_string(),
                radius: 25.0,
                name: "A".into(),
            },
        );

        let b = self.add_node(
            100.0,
            NodeProps {
                color: light_green.to_string(),
                radius: 25.0,
                name: "B".into(),
            },
        );

        let c = self.add_node(
            100.0,
            NodeProps {
                color: light_green.to_string(),
                radius: 25.0,
                name: "C".into(),
            },
        );

        self.add_edge(
            a,
            b,
            EdgeProps {
                color: dark_green.to_string(),
                name: "".into(),
            },
        );

        self.add_edge(
            b,
            c,
            EdgeProps {
                color: dark_green.to_string(),
                name: "".into(),
            },
        );
        self.add_edge(
            c,
            a,
            EdgeProps {
                color: dark_green.to_string(),
                name: "".into(),
            },
        );
    }

    pub fn new() -> Self {
        Self {
            view_transform: ViewTransform::new(),
            highlight_node: None,
            graph: ForceGraph::new(SimulationParameters {
                force_charge: 12000.0,
                force_spring: 0.3,
                force_max: 250.0,
                node_speed: 500.0,
                damping_factor: 0.98,
            }),
            highlight_color: "rgb(231, 240, 220)".to_string(),
            last_frame_timestamp: 0.0,
        }
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

    pub fn reset(&mut self) {
        self.graph.clear();
        self.highlight_node = None;
    }

    pub fn update(
        &mut self,
        _dt: f64,
        _mouse_position: Option<(f64, f64)>,
        last_click: Option<(f64, f64)>,
    ) {
        self.graph.update(0.03 as f32);

        // Update the highlighted node if one was clicked
        if let Some((click_x, click_y)) = last_click {
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

    pub fn render(
        &mut self,
        _dt: f64,
        _mouse_position: Option<(f64, f64)>,
        _last_click: Option<(f64, f64)>,
        ctx: &CanvasRenderingContext2d,
    ) -> Result<(), JsValue> {
        ctx.set_line_width(5.0);

        self.graph.visit_edges(|node1, node2, edge| {
            ctx.set_stroke_style_str(&edge.user_data.color);

            let x1 = node1.x() as f64;
            let y1 = node1.y() as f64;
            let x2 = node2.x() as f64;
            let y2 = node2.y() as f64;

            // Draw the edge
            ctx.begin_path();
            ctx.move_to(x1, y1);
            ctx.line_to(x2, y2);
            ctx.stroke();

            // Draw the edge label
            let mid_x = (x1 + x2) / 2.0;
            let mid_y = (y1 + y2) / 2.0;
            let text_measurements = ctx.measure_text(&edge.user_data.name).unwrap();
            ctx.fill_text(
                &edge.user_data.name,
                mid_x - text_measurements.width() / 2.0,
                mid_y,
            )
            .unwrap();
        });

        self.graph.visit_nodes(|node| {
            ctx.set_fill_style_str(&node.data.user_data.color);

            // Draw the node circle
            ctx.begin_path();
            ctx.arc(
                node.x() as f64,
                node.y() as f64,
                node.data.user_data.radius,
                0.0,
                2.0 * PI,
            )
            .unwrap();
            ctx.fill();

            // Draw the label nearby
            ctx.set_fill_style_str(&self.highlight_color);
            let text_measurements = ctx.measure_text(&node.data.user_data.name).unwrap();
            ctx.fill_text(
                &node.data.user_data.name,
                node.x() as f64 - text_measurements.width() / 2.0,
                node.y() as f64,
            )
            .unwrap();

            // Draw a highlight around the node if it has been clicked
            if let Some(node_idx) = self.highlight_node {
                if node_idx == node.index() {
                    ctx.set_stroke_style_str(&self.highlight_color);
                    ctx.begin_path();
                    ctx.arc(
                        node.x() as f64,
                        node.y() as f64,
                        node.data.user_data.radius * 1.25,
                        0.0,
                        2.0 * PI,
                    )
                    .unwrap();
                    ctx.stroke();
                }
            }
        });

        Ok(())
    }
}
