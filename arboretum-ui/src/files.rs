use std::{cell::RefCell, rc::Rc};

use arboretum_core::{merge_u64, split_u64, Prefix, Value};
use arboretum_query::{http_reqwasm::HttpGraphQueryExecutor, GraphQueryResponse};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, Event, HtmlInputElement, Window};

use crate::{State, ViewTransform};

pub struct FilesViewMode {
    pub view_transform: ViewTransform,

    pub show_system_files: bool,
    pub system_files: Option<Vec<(u64, String)>>,

    pub show_header_files: bool,
    pub header_files: Option<Vec<(u64, String)>>,

    pub show_translation_units: bool,
    pub translation_units: Option<Vec<(u64, String)>>,
}

impl FilesViewMode {
    pub fn new() -> Self {
        Self {
            view_transform: ViewTransform::new(),

            show_header_files: true,
            header_files: None,

            show_system_files: true,
            system_files: None,

            show_translation_units: true,
            translation_units: None,
        }
    }

    pub fn setup(&mut self, slf: Rc<RefCell<State>>, window: &Window) {
        use web_sys::console;

        console::log_1(&"files setup start".into());

        let document = window.document().unwrap();

        macro_rules! bind_checkbox {
            ($id:tt, $var:ident) => {
                let checkbox = document.get_element_by_id($id).unwrap();
                let checkbox: HtmlInputElement = checkbox
                    .dyn_into::<web_sys::HtmlInputElement>()
                    .expect(&format!("Expected {} to be an input tag.", $id));
                checkbox.set_checked(self.$var);
                let callback_state = slf.clone();
                let closure = Closure::wrap(Box::new(move |event: Event| {
                    let target = event.target().unwrap();
                    let checkbox = target.dyn_into::<HtmlInputElement>().unwrap();
                    callback_state.borrow_mut().files.$var = checkbox.checked();

                    console::log_1(&format!("{} toggle", $id).into());
                }) as Box<dyn FnMut(_)>);
                checkbox.set_onchange(Some(closure.as_ref().unchecked_ref()));
                closure.forget();
            };
        }

        bind_checkbox!("chkShowSystem", show_system_files);
        bind_checkbox!("chkShowHeader", show_header_files);
        bind_checkbox!("chkShowTU", show_translation_units);

        let query_executor =
            HttpGraphQueryExecutor::new("http://127.0.0.1:8080/api/api/query/".into());
        wasm_bindgen_futures::spawn_local(async move {
            let response = query_executor
                .run(&arboretum_query::GraphQuery::GlobalPOS(Prefix::Two(
                    clang_id(reify_rs::META_CLASS),
                    clang_id(reify_rs::FILE_CLASS),
                )))
                .await
                .expect("Failed to execute file lookup");

            if let GraphQueryResponse::Edges(edges) = response {
                let mut system_files = Vec::new();
                let mut header_files = Vec::new();
                let mut translation_units = Vec::new();
                for (file_id, _, _, _) in edges {
                    let response = query_executor
                        .run(&arboretum_query::GraphQuery::SPO(Prefix::Two(
                            file_id,
                            clang_id(reify_rs::FILE_NAME),
                        )))
                        .await
                        .expect("Failed to get file name");
                    if let GraphQueryResponse::Edges(name_edges) = response {
                        if let Some(name_id) = name_edges.iter().next() {
                            let response = query_executor
                                .run(&arboretum_query::GraphQuery::NodeProps(name_id.2))
                                .await
                                .expect("Failed to get file name string");

                            if let GraphQueryResponse::NodeProps(Some(Value::String(filename))) =
                                response
                            {
                                if filename.starts_with("/usr/include/") {
                                    system_files.push((file_id, filename));
                                } else if filename.ends_with(".h") {
                                    header_files.push((file_id, filename));
                                } else {
                                    translation_units.push((file_id, filename));
                                }
                            }
                        }
                    }
                }
                let mut slf = slf.borrow_mut();
                slf.files.system_files = Some(system_files);
                slf.files.header_files = Some(header_files);
                slf.files.translation_units = Some(translation_units);
            }
        });
    }

    pub fn reset(&mut self) {}

    pub fn render(
        &mut self,
        _dt: f64,
        _mouse_position: Option<(f64, f64)>,
        _last_click: Option<(f64, f64)>,
        ctx: &CanvasRenderingContext2d,
    ) -> Result<(), JsValue> {
        let initial_y = 300.0;
        let initial_x = 300.0;

        let mut x_pos = initial_x;
        let mut y_pos = initial_y;

        let box_padding = 5.0;
        let _box_height = 16.0 * 3.0;
        let y_gap = 16.0;

        let loading = self.system_files.is_none()
            || self.header_files.is_none()
            || self.translation_units.is_none();

        let black = "rgb(0,0,0)";
        let red = "rgb(255,192,192)";
        let green = "rgb(192,255,192)";
        let blue = "rgb(192,192,255)";
        let _yellow = "rgb(255, 255, 192)";

        ctx.set_fill_style_str(black);

        if loading {
            ctx.set_font("bold 16pt Courier");
            ctx.fill_text("Loading...", x_pos, y_pos)?;
        } else {
            let mut max_width = 0.0;

            if self.show_system_files {
                ctx.set_fill_style_str(black);
                ctx.set_font("bold 16pt Courier");
                ctx.fill_text("System Headers", x_pos, y_pos)?;

                y_pos += 16.0;
                for (_, name) in self.system_files.as_ref().unwrap() {
                    let text_width = ctx.measure_text(&name)?.width();

                    if text_width > max_width {
                        max_width = text_width;
                    }

                    ctx.set_fill_style_str(red);
                    ctx.fill_rect(
                        x_pos,
                        y_pos - box_padding,
                        2.0 * box_padding + text_width,
                        2.0 * box_padding + 16.0,
                    );

                    ctx.set_fill_style_str(black);
                    ctx.set_font("16pt Courier");
                    ctx.fill_text(&name, x_pos + box_padding, y_pos + box_padding + 8.0)?;
                    y_pos += 16.0 + y_gap;
                }

                x_pos += max_width + box_padding * 2.0;
                max_width = 0.0;

                y_pos = initial_y;
            }

            if self.show_header_files {
                ctx.set_font("bold 16pt Courier");
                ctx.fill_text("Headers", x_pos, y_pos)?;

                y_pos += 16.0;
                for (_, name) in self.header_files.as_ref().unwrap() {
                    let text_width = ctx.measure_text(&name)?.width();

                    if text_width > max_width {
                        max_width = text_width;
                    }

                    ctx.set_fill_style_str(green);
                    ctx.fill_rect(
                        x_pos,
                        y_pos - box_padding,
                        2.0 * box_padding + text_width,
                        2.0 * box_padding + 16.0,
                    );

                    ctx.set_fill_style_str(black);
                    ctx.set_font("16pt Courier");
                    ctx.fill_text(&name, x_pos + box_padding, y_pos + box_padding + 8.0)?;
                    y_pos += 16.0 + y_gap;
                }

                x_pos += max_width + box_padding * 2.0;
                max_width = 0.0;

                y_pos = initial_y;
            }

            if self.show_translation_units {
                ctx.set_font("bold 16pt Courier");
                ctx.fill_text("Translation Units", x_pos, y_pos)?;

                y_pos += 16.0;
                for (_, name) in self.translation_units.as_ref().unwrap() {
                    let text_width = ctx.measure_text(&name)?.width();

                    if text_width > max_width {
                        max_width = text_width;
                    }

                    ctx.set_fill_style_str(blue);
                    ctx.fill_rect(
                        x_pos,
                        y_pos - box_padding,
                        2.0 * box_padding + text_width,
                        2.0 * box_padding + 16.0,
                    );

                    ctx.set_fill_style_str(black);
                    ctx.set_font("16pt Courier");
                    ctx.fill_text(&name, x_pos + box_padding, y_pos + box_padding + 8.0)?;
                    y_pos += 16.0 + y_gap;
                }
            }
        }
        Ok(())
    }
}

fn clang_id(id: u64) -> u64 {
    merge_u64(1, split_u64(id).1)
}
