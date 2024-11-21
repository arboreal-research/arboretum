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
        let view_transform = state_ref.view_transform_mut();

        view_transform.is_panning = false;

        // Calculate the movement distance
        let dx = event.client_x() as f64 - view_transform.pan_start_x;
        let dy = event.client_y() as f64 - view_transform.pan_start_y;
        let distance = (dx * dx + dy * dy).sqrt();

        // Only consider it a click if the distance is below the threshold
        if distance < 5.0 {
            let x = (event.offset_x() as f64 - view_transform.translate_x) / view_transform.scale;
            let y = (event.offset_y() as f64 - view_transform.translate_y) / view_transform.scale;

            state_ref.last_click = Some((x, y));
        }
    }) as Box<dyn FnMut(MouseEvent)>);

    canvas2
        .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())
        .expect("Failed to add mouseup event listener");
    closure.forget();
}
