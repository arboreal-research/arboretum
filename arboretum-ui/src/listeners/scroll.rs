use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, WheelEvent, Window};

use crate::{render_frame, State};

pub(super) fn setup(
    canvas: HtmlCanvasElement,
    window: Window,
    context: CanvasRenderingContext2d,
    state: Rc<RefCell<State>>,
) {
    let canvas2 = canvas.clone();
    let closure = Closure::wrap(Box::new(move |event: WheelEvent| {
        {
            event.prevent_default();
            let mut state = state.borrow_mut();

            // Get the mouse position relative to the canvas
            let rect = canvas.get_bounding_client_rect();
            let mouse_x = event.client_x() as f64 - rect.left();
            let mouse_y = event.client_y() as f64 - rect.top();

            let scale_factor = if event.delta_y() < 0.0 { 1.1 } else { 0.9 };
            let new_scale = state.scale * scale_factor;

            // Calculate the translation adjustments
            let translate_x = mouse_x - mouse_x * new_scale / state.scale
                + state.translate_x * new_scale / state.scale;
            let translate_y = mouse_y - mouse_y * new_scale / state.scale
                + state.translate_y * new_scale / state.scale;

            // Update the state with the new scale and translation
            state.scale = new_scale;
            state.translate_x = translate_x;
            state.translate_y = translate_y;
        }

        render_frame(
            &context.clone(),
            &window.clone(),
            &canvas.clone(),
            state.clone(),
            0.0,
        );
    }) as Box<dyn FnMut(WheelEvent)>);

    canvas2
        .add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())
        .expect("Failed to add zoom event listener");
    closure.forget();
}
