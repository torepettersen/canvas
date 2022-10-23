use crate::canvas::Canvas;
use crate::events::Point;
use crate::layers::Layers;
use crate::objects::Edge;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;
use web_sys::HtmlCanvasElement;
use web_sys::MouseEvent;

pub struct State {
    old_canvas: HtmlCanvasElement,
    pub canvas: Rc<RefCell<Canvas>>,
    pub layers: Rc<RefCell<Layers>>,
    pub active_layer: Option<usize>,
    pub active_edge: Option<Edge>,
    pub mouse_start: Option<Point>,
    pub outlined_layer: Option<usize>,
    pub _closuers: Vec<Closure<dyn FnMut(MouseEvent)>>,
}

impl State {
    pub fn new(canvas_id: &str) -> Result<State, JsValue> {
        let old_canvas = Self::init_canvas(canvas_id)?;
        let canvas = Canvas::new(canvas_id)?.into();
        let layers = Layers::new().into();

        Ok(State {
            canvas,
            old_canvas,
            layers,
            active_layer: None,
            active_edge: None,
            outlined_layer: None,
            mouse_start: None,
            _closuers: Vec::new(),
        })
    }

    pub fn canvas(&self) -> &HtmlCanvasElement {
        &self.old_canvas
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
}

impl Into<Rc<RefCell<State>>> for State {
    fn into(self) -> Rc<RefCell<State>> {
        Rc::new(RefCell::new(self))
    }
}
