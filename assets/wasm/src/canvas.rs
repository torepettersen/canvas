use crate::alignments::Alignment;
use crate::events::Point;
use crate::layers::LayerState;
use crate::layers::Layers;
use crate::layers::Layer;
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
                .draw_outline(context);
        }
        match layers.active_layer() {
            Some(LayerState::Creating { layer, .. })
            | Some(LayerState::Idle { layer, .. })
            | Some(LayerState::Resize { layer, .. })
            | Some(LayerState::Relocate { layer, .. }) => {
                let active_layer = &layers.layers()[*layer];
                active_layer.object.draw_active(context);
                draw_alignments(active_layer, layers.alignments(), context);
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

fn draw_alignments(layer: &Layer, alignments: &[Alignment], context: &CanvasRenderingContext2d) {
    let top = layer.object.top();
    let left = &layer.object.left();
    let right = &layer.object.right();
    // crate::log!("{:?}", alignments);
    let iter = alignments.iter().flat_map(|alignment| {
        if let Alignment::Y { y, left: x1, right: x2 } = *alignment {
            if y == top {
                vec![x1, x2]
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    })
        .chain(vec![*left, *right]);

    let max = iter.clone().reduce(f64::max).unwrap();
    let min = iter.reduce(f64::min).unwrap();

    context.begin_path();
    context.move_to(min, top);
    context.line_to(max, top);
    context.stroke();
}

impl From<Canvas> for Rc<RefCell<Canvas>> {
    fn from(canvas: Canvas) -> Self {
        Rc::new(RefCell::new(canvas))
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
