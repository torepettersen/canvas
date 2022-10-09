
use crate::events::Point;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;
use web_sys::MouseEvent;
use web_sys::window;

pub struct State {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    closures: Vec<Closure<dyn FnMut(MouseEvent)>>,
    pub mouse_start: Option<Point>,
}

impl State {
    pub fn new(canvas_id: &str) -> Result<Rc<RefCell<State>>, JsValue> {
        let canvas = Self::init_canvas(canvas_id)?;
        let context = Self::init_context(&canvas)?;

        Ok(Rc::new(RefCell::new(State {
            canvas: canvas,
            context: context,
            closures: Vec::new(),
            mouse_start: None,
        })))
    }
    
    pub fn canvas(&self) -> &HtmlCanvasElement {
        &self.canvas
    }

    pub fn context(&self) -> &CanvasRenderingContext2d {
        &self.context
    }

    pub fn add_closure(&mut self, closure: Closure<dyn FnMut(MouseEvent)>) {
        self.closures.push(closure);
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
