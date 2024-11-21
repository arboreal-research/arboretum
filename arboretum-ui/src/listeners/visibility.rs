use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{CanvasRenderingContext2d, Event, HtmlCanvasElement, VisibilityState, Window};

use crate::State;

pub(super) fn setup(
    _canvas: HtmlCanvasElement,
    _window: Window,
    _context: CanvasRenderingContext2d,
    state: Rc<RefCell<State>>,
) {
    let state = state.clone();
    let closure = Closure::wrap(Box::new(move |_: Event| {
        let mut state = state.borrow_mut();
        if web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .visibility_state()
            == VisibilityState::Visible
        {
            state.force_directed.last_frame_timestamp =
                web_sys::window().unwrap().performance().unwrap().now();
        }
    }) as Box<dyn FnMut(Event)>);

    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback("visibilitychange", closure.as_ref().unchecked_ref())
        .expect("Failed to add visibility event listener");
    closure.forget();
}
