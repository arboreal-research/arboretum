pub struct ViewTransform {
    // View
    pub scale: f64,
    pub translate_x: f64,
    pub translate_y: f64,

    // Panning
    pub is_panning: bool,
    pub pan_start_x: f64,
    pub pan_start_y: f64,
    pub last_x: f64,
    pub last_y: f64,
}

impl ViewTransform {
    pub fn new() -> Self {
        Self {
            scale: 1.0,
            translate_x: 0.0,
            translate_y: 0.0,
            is_panning: false,
            pan_start_x: 0.0,
            pan_start_y: 0.0,
            last_x: 0.0,
            last_y: 0.0,
        }
    }
}
