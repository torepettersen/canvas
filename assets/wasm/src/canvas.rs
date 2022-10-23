use crate::events::Point;
use crate::layers::LayerState;
use crate::layers::Layers;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::window;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;
use web_sys::MouseEvent;

pub struct Canvas {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
}

impl Canvas {
    pub fn new(canvas_id: &str) -> Result<Self, JsValue> {
        let canvas = init_canvas(canvas_id)?;
        let context = init_context(&canvas)?;
        Ok(Canvas { canvas, context })
    }

    pub fn render(&self, layers: &Layers) {
        let canvas = self.canvas();
        let context = self.context();

        context.begin_path();
        context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

        for layer in layers.layers() {
            layer.object.draw(context);
        }
        if let Some(outlined_layer) = layers.outlined_layer() {
            layers.layers()[*outlined_layer]
                .object
                .draw_outline(&context);
        }
        match layers.active_layer() {
            Some(LayerState::CreatingLayer { layer, .. })
            | Some(LayerState::IdleLayer { layer, .. })
            | Some(LayerState::ResizeLayer { layer, .. }) => {
                layers.layers()[*layer].object.draw_active(&context);
            }
            _ => {}
        }
    }

    pub fn canvas(&self) -> &HtmlCanvasElement {
        &self.canvas
    }

    pub fn context(&self) -> &CanvasRenderingContext2d {
        &self.context
    }

    pub fn get_mouse_position(&self, event: &MouseEvent) -> Point {
        let rect = self.canvas.get_bounding_client_rect();
        Point {
            x: event.client_x() as f64 - rect.left(),
            y: event.client_y() as f64 - rect.top(),
        }
    }
}

impl Into<Rc<RefCell<Canvas>>> for Canvas {
    fn into(self) -> Rc<RefCell<Canvas>> {
        Rc::new(RefCell::new(self))
    }
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
