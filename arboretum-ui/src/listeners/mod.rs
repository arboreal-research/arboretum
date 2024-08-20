use std::{cell::RefCell, rc::Rc};

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Window};

use crate::State;

mod mousedown;
mod mousemove;
mod mouseup;
mod resize;
mod scroll;
mod visibility;

pub(crate) fn setup(
    canvas: HtmlCanvasElement,
    window: Window,
    context: CanvasRenderingContext2d,
    state: Rc<RefCell<State>>,
) {
    resize::setup(
        canvas.clone(),
        window.clone(),
        context.clone(),
        state.clone(),
    );

    scroll::setup(
        canvas.clone(),
        window.clone(),
        context.clone(),
        state.clone(),
    );

    mousedown::setup(
        canvas.clone(),
        window.clone(),
        context.clone(),
        state.clone(),
    );

    mousemove::setup(
        canvas.clone(),
        window.clone(),
        context.clone(),
        state.clone(),
    );

    mouseup::setup(
        canvas.clone(),
        window.clone(),
        context.clone(),
        state.clone(),
    );

    visibility::setup(
        canvas.clone(),
        window.clone(),
        context.clone(),
        state.clone(),
    );
}
