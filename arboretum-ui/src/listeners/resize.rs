use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Window};

use crate::{set_canvas_size, State};

pub(super) fn setup(
    canvas: HtmlCanvasElement,
    window: Window,
    _context: CanvasRenderingContext2d,
    state: Rc<RefCell<State>>,
) {
    set_canvas_size(&canvas, &window);

    let window2 = window.clone();
    let closure = Closure::wrap(Box::new(move || {
        let mut state = state.borrow_mut();

        let new_width = window.inner_width().unwrap().as_f64().unwrap();
        let new_height = window.inner_height().unwrap().as_f64().unwrap();

        let cur_width = canvas.width() as f64;
        let cur_height = canvas.height() as f64;

        // Recalculate scale to maintain zoom level
        let scale_x = new_width / cur_width;
        let scale_y = new_height / cur_height;
        state.scale *= (scale_x + scale_y) / 2.0; // Averaging scales for simplicity

        // Adjust translate to keep the view centered
        state.translate_x *= scale_x;
        state.translate_y *= scale_y;

        set_canvas_size(&canvas, &window);
    }) as Box<dyn FnMut()>);
    window2
        .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
        .expect("Failed to add resize event listener");
    closure.forget(); // Keep the closure alive
}
