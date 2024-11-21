use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::ViewTransform;

pub struct InvocationsViewMode {
    pub view_transform: ViewTransform,
}

impl InvocationsViewMode {
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
        _mouse_position: Option<(f64, f64)>,
        _last_click: Option<(f64, f64)>,
        _ctx: &CanvasRenderingContext2d,
    ) -> Result<(), JsValue> {
        Ok(())
    }
}
