use std::{cell::RefCell, rc::Rc};

use arboretum_ui_protocol::Query;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Window};

mod state;
use state::*;

mod query;
use query::*;

mod listeners;

mod render;
pub use render::render_frame;

enum Error {
    JsError(JsValue),
    SerdeWasmBindgenError(serde_wasm_bindgen::Error),
}

impl From<JsValue> for Error {
    fn from(e: JsValue) -> Self {
        Error::JsError(e)
    }
}

impl From<serde_wasm_bindgen::Error> for Error {
    fn from(e: serde_wasm_bindgen::Error) -> Self {
        Error::SerdeWasmBindgenError(e)
    }
}

fn set_canvas_size(canvas: &HtmlCanvasElement, window: &Window) {
    let width = window.inner_width().unwrap().as_f64().unwrap();
    let height = window.inner_height().unwrap().as_f64().unwrap();
    canvas.set_width(width as u32);
    canvas.set_height(height as u32);
}

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    // Get the document object from the web-sys crate.
    let window: Window = web_sys::window().expect("should have a Window");
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    set_canvas_size(&canvas, &window);

    // Create the graph
    let state = Rc::new(RefCell::new(State::new(
        window.performance().unwrap().now(),
        JsValue::from_str("rgb(231, 240, 220)"),
    )));

    // Run an initial query for the data.
    run_query(window.clone(), Query::TestQuery, state.clone());

    listeners::setup(
        canvas.clone(),
        window.clone(),
        context.clone(),
        state.clone(),
    );

    render::schedule_render(context, window.clone(), canvas.clone(), state)?;

    Ok(())
}
