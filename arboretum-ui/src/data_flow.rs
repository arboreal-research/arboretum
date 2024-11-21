use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, Window};

use crate::ViewTransform;

pub struct DataFlowViewMode {
    pub view_transform: ViewTransform,
}

impl DataFlowViewMode {
    pub fn new() -> Self {
        Self {
            view_transform: ViewTransform::new(),
        }
    }

    pub fn update(&mut self, _dt: f64) {}

    pub fn reset(&mut self) {}

    pub fn render(
        &mut self,
        _dt: f64,
        mouse_position: Option<(f64, f64)>,
        last_click: Option<(f64, f64)>,
        ctx: &CanvasRenderingContext2d,
    ) -> Result<(), JsValue> {
        Ok(())
    }
}
