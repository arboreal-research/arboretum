use std::{cell::RefCell, rc::Rc};

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
    let mut state = state.borrow_mut();

    {
        let view_transform = state.view_transform();

        // Clear the whole canvas.
        ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0).unwrap();
        ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

        // Adjust our transform to correspond to the current zoom/pan/etc.
        ctx.set_transform(
            view_transform.scale,
            0.0,
            0.0,
            view_transform.scale,
            view_transform.translate_x,
            view_transform.translate_y,
        )
        .unwrap();
    }

    state.render(dt, ctx)
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
        let dt = timestamp - state.borrow().force_directed.last_frame_timestamp;
        state.borrow_mut().force_directed.last_frame_timestamp = timestamp;

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
