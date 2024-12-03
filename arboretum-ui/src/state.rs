use std::cell::RefCell;
use std::rc::Rc;

use web_sys::{CanvasRenderingContext2d, Window};

use crate::control_flow::ControlFlowViewMode;
use crate::data_flow::DataFlowViewMode;
use crate::files::FilesViewMode;
use crate::force_directed::ForceDirectedViewMode;
use crate::invocations::InvocationsViewMode;
use crate::ViewTransform;

#[derive(Clone)]
pub enum ViewMode {
    Files,
    Invocations,
    ControlFlow,
    DataFlow,
    ForceDirected,
}

pub struct State {
    pub active_mode: ViewMode,

    pub mouse_position: Option<(f64, f64)>,
    pub last_click: Option<(f64, f64)>,

    pub force_directed: ForceDirectedViewMode,
    pub files: FilesViewMode,
    pub invocations: InvocationsViewMode,
    pub control_flow: ControlFlowViewMode,
    pub data_flow: DataFlowViewMode,
}

impl State {
    pub fn new() -> Self {
        Self {
            active_mode: ViewMode::Files,

            mouse_position: None,
            last_click: None,

            files: FilesViewMode::new(),
            invocations: InvocationsViewMode::new(),
            control_flow: ControlFlowViewMode::new(),
            data_flow: DataFlowViewMode::new(),
            force_directed: ForceDirectedViewMode::new(),
        }
    }

    pub fn setup(slf: Rc<RefCell<State>>, window: &Window) {
        let callback_slf = slf.clone();

        let active_mode;
        {
            active_mode = slf.borrow().active_mode.clone()
        }

        match active_mode {
            ViewMode::Files => slf.borrow_mut().files.setup(callback_slf, window),
            ViewMode::Invocations => todo!(),
            ViewMode::ControlFlow => todo!(),
            ViewMode::DataFlow => todo!(),
            ViewMode::ForceDirected => todo!(),
        }
    }

    pub fn reset(&mut self) {
        {
            let view_transform = self.view_transform_mut();
            view_transform.scale = 1.0;
            view_transform.translate_x = 0.0;
            view_transform.translate_y = 0.0;
        }

        self.last_click = None;

        self.files.reset();
        self.invocations.reset();
        self.control_flow.reset();
        self.data_flow.reset();
        self.force_directed.reset();
    }

    pub fn view_transform(&self) -> &ViewTransform {
        match self.active_mode {
            ViewMode::Files => &self.files.view_transform,
            ViewMode::Invocations => &self.invocations.view_transform,
            ViewMode::ControlFlow => &self.control_flow.view_transform,
            ViewMode::DataFlow => &self.data_flow.view_transform,
            ViewMode::ForceDirected => &self.force_directed.view_transform,
        }
    }

    pub fn view_transform_mut(&mut self) -> &mut ViewTransform {
        match self.active_mode {
            ViewMode::Files => &mut self.files.view_transform,
            ViewMode::Invocations => &mut self.invocations.view_transform,
            ViewMode::ControlFlow => &mut self.control_flow.view_transform,
            ViewMode::DataFlow => &mut self.data_flow.view_transform,
            ViewMode::ForceDirected => &mut self.force_directed.view_transform,
        }
    }

    pub fn render(&mut self, dt: f64, ctx: &CanvasRenderingContext2d) {
        match self.active_mode {
            ViewMode::Files => self
                .files
                .render(dt, self.mouse_position, self.last_click.take(), ctx)
                .expect("ok"),
            ViewMode::Invocations => self
                .invocations
                .render(dt, self.mouse_position, self.last_click.take(), ctx)
                .expect("ok"),
            ViewMode::ControlFlow => self
                .control_flow
                .render(dt, self.mouse_position, self.last_click.take(), ctx)
                .expect("ok"),
            ViewMode::DataFlow => self
                .data_flow
                .render(dt, self.mouse_position, self.last_click.take(), ctx)
                .expect("ok"),
            ViewMode::ForceDirected => self
                .force_directed
                .render(dt, self.mouse_position, self.last_click.take(), ctx)
                .expect("ok"),
        }
    }
}
