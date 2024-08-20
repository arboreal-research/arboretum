use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent, Window};

use crate::State;

pub(super) fn setup(
    canvas: HtmlCanvasElement,
    _window: Window,
    _context: CanvasRenderingContext2d,
    state: Rc<RefCell<State>>,
) {
    let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
        let mut state = state.borrow_mut();
        state.is_panning = true;
        state.pan_start_x = event.client_x() as f64;
        state.pan_start_y = event.client_y() as f64;
        state.last_x = event.client_x() as f64;
        state.last_y = event.client_y() as f64;
    }) as Box<dyn FnMut(MouseEvent)>);

    canvas
        .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
        .expect("Failed to add mousedown event listener");
    closure.forget();
}
