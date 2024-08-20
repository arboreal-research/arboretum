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
    let canvas2 = canvas.clone();
    let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
        let mut state_ref = state.borrow_mut();
        if state_ref.is_panning {
            let cur_x = event.client_x() as f64;
            let cur_y = event.client_y() as f64;

            let dx = cur_x - state_ref.last_x;
            let dy = cur_y - state_ref.last_y;

            state_ref.last_x = cur_x;
            state_ref.last_y = cur_y;

            state_ref.translate_x += dx;
            state_ref.translate_y += dy;
        }
    }) as Box<dyn FnMut(MouseEvent)>);

    canvas2
        .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
        .expect("Failed to add movemove event listener");
    closure.forget();
}
