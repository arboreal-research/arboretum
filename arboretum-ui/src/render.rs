use std::{cell::RefCell, f64::consts::PI, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Window};

use crate::State;

pub fn render_frame(
    ctx: &CanvasRenderingContext2d,
    _window: &Window,
    canvas: &HtmlCanvasElement,
    state: Rc<RefCell<State>>,
    dt: f64,
) {
    state.borrow_mut().update(dt);

    let state = state.borrow();

    // Clear the whole canvas.
    ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0).unwrap();
    ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    // Adjust our transform to correspond to the current zoom/pan/etc.
    ctx.set_transform(
        state.scale,
        0.0,
        0.0,
        state.scale,
        state.translate_x,
        state.translate_y,
    )
    .unwrap();

    ctx.set_line_width(5.0);

    state.graph.visit_edges(|node1, node2, edge| {
        ctx.set_stroke_style(&edge.user_data.color);

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

    state.graph.visit_nodes(|node| {
        ctx.set_fill_style(&node.data.user_data.color);

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
        ctx.set_fill_style(&state.highlight_color);
        let text_measurements = ctx.measure_text(&node.data.user_data.name).unwrap();
        ctx.fill_text(
            &node.data.user_data.name,
            node.x() as f64 - text_measurements.width() / 2.0,
            node.y() as f64,
        )
        .unwrap();

        // Draw a highlight around the node if it has been clicked
        if let Some(node_idx) = state.highlight_node {
            if node_idx == node.index() {
                ctx.set_stroke_style(&state.highlight_color);
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
}

pub fn schedule_render(
    context: CanvasRenderingContext2d,
    window: Window,
    canvas: HtmlCanvasElement,
    state: Rc<RefCell<State>>,
) -> Result<(), JsValue> {
    let window2 = window.clone();

    // Create a new closure for the animation frame callback
    let callback = Closure::wrap(Box::new(move |timestamp: f64| {
        let dt = timestamp - state.borrow().last_frame_timestamp;
        state.borrow_mut().last_frame_timestamp = timestamp;

        render_frame(&context, &window, &canvas, state.clone(), dt);

        // Request the next frame
        schedule_render(
            context.clone(),
            window.clone(),
            canvas.clone(),
            state.clone(),
        )
        .expect("Failed to request next animation frame");
    }) as Box<dyn FnMut(f64)>);

    // Request the first animation frame
    window2
        .request_animation_frame(callback.as_ref().unchecked_ref())
        .expect("Failed to request animation frame");

    // Keep the closure alive
    callback.forget();

    Ok(())
}
