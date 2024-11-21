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

        let cur_x = event.client_x() as f64;
        let cur_y = event.client_y() as f64;

        state_ref.mouse_position = Some((cur_x, cur_y));

        let view_transform = state_ref.view_transform_mut();
        if view_transform.is_panning {
            let dx = cur_x - view_transform.last_x;
            let dy = cur_y - view_transform.last_y;

            view_transform.last_x = cur_x;
            view_transform.last_y = cur_y;

            view_transform.translate_x += dx;
            view_transform.translate_y += dy;
        }
    }) as Box<dyn FnMut(MouseEvent)>);

    canvas2
        .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
        .expect("Failed to add movemove event listener");
    closure.forget();
}
