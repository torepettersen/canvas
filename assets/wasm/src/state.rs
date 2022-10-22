use crate::events::Point;
use crate::layers::Layer;
use crate::objects::Edge;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;
use web_sys::MouseEvent;

pub struct State {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    pub layers: Vec<Layer>,
    pub active_layer: Option<usize>,
    pub active_edge: Option<Edge>,
    pub outlined_layer: Option<usize>,
    pub _closuers: Vec<Closure<dyn FnMut(MouseEvent)>>,
    pub mouse_start: Option<Point>,
}

impl State {
    pub fn new(canvas_id: &str) -> Result<State, JsValue> {
        let canvas = Self::init_canvas(canvas_id)?;
        let context = Self::init_context(&canvas)?;

        Ok(State {
            canvas: canvas,
            context: context,
            layers: Vec::new(),
            active_layer: None,
            active_edge: None,
            outlined_layer: None,
            mouse_start: None,
            _closuers: Vec::new(),
        })
    }

    pub fn to_ptr(self) -> Rc<RefCell<State>> {
        Rc::new(RefCell::new(self))
    }

    pub fn canvas(&self) -> &HtmlCanvasElement {
        &self.canvas
    }

    pub fn context(&self) -> &CanvasRenderingContext2d {
        &self.context
    }

    fn init_canvas(canvas_id: &str) -> Result<HtmlCanvasElement, JsValue> {
        let canvas = window()
            .ok_or("Window not found")?
            .document()
            .ok_or("Document not found")?
            .get_element_by_id(canvas_id)
            .ok_or(format!("Canvas with id '{}' not found", canvas_id))?
            .dyn_into::<HtmlCanvasElement>()?;

        Ok(canvas)
    }

    fn init_context(canvas: &HtmlCanvasElement) -> Result<CanvasRenderingContext2d, JsValue> {
        let context = canvas
            .get_context("2d")?
            .ok_or("Canvas 2d context not found")?
            .dyn_into::<CanvasRenderingContext2d>()?;

        Ok(context)
    }
}
