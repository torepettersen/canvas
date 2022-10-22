use crate::layers::Layer;
use crate::objects::Rect;
use crate::renderer;
use crate::state::State;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use web_sys::MouseEvent;

macro_rules! closure {
    ( { $($x:ident),* }, $y:expr ) => {
        Closure::new({
            $(let $x = $x.clone();)*
            $y
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub fn init(state_ref: &Rc<RefCell<State>>) {
    let on_mousedown = on_mousedown(&state_ref);
    let on_mouseup = on_mouseup(&state_ref);
    let on_mousemove = on_mousemove(&state_ref);

    let mut state = state_ref.borrow_mut();

    let canvas = state.canvas();
    canvas.set_onmousedown(Some(on_mousedown.as_ref().unchecked_ref()));
    canvas.set_onmouseup(Some(on_mouseup.as_ref().unchecked_ref()));
    canvas.set_onmousemove(Some(on_mousemove.as_ref().unchecked_ref()));

    state._closuers = vec![on_mousedown, on_mouseup, on_mousemove];
}

fn on_mousedown(state_ref: &Rc<RefCell<State>>) -> Closure<dyn FnMut(MouseEvent)> {
    closure!({ state_ref }, move |event: MouseEvent| {
        let mut state = state_ref.borrow_mut();
        let point = get_mouse_position(state.canvas(), &event);
        match &*state {
            State { active_layer: Some(active_layer), .. } => {
                let maybe_edge =
                    state.layers[*active_layer].point_over_edge(state.context(), point);
                if let Some(edge) = maybe_edge {
                    crate::log!("{:?}", edge.rect);
                    state.active_edge = Some(edge)
                }
            }
            State { outlined_layer: Some(outlined_layer), .. } => {
                state.active_layer = Some(*outlined_layer);
                state.outlined_layer = None;
            }
            _ => {
                state.mouse_start = Some(get_mouse_position(state.canvas(), &event));
                state.active_layer = None;
            }
        }
        drop(state);
        renderer::render(&state_ref);
    })
}

fn on_mousemove(state_ref: &Rc<RefCell<State>>) -> Closure<dyn FnMut(MouseEvent)> {
    closure!({ state_ref }, move |event: MouseEvent| {
        let mut state = state_ref.borrow_mut();
        let point = get_mouse_position(state.canvas(), &event);
        match &mut *state {
            State { active_edge: Some(edge), active_layer: Some(active_layer), layers, .. } => {
                layers[*active_layer].object.resize(point, *edge);
                drop(state);
                renderer::render(&state_ref);
            }
            State { mouse_start: Some(mouse_start), active_layer: Some(active_layer), layers, .. } => {
                layers[*active_layer] =
                    Layer { object: Box::new(Rect::new(*mouse_start, point)) };
                drop(state);
                renderer::render(&state_ref);
            }
            State { mouse_start: Some(mouse_start), layers, active_layer, .. } => {
                let layer = Layer { object: Box::new(Rect::new(*mouse_start, point)) };
                layers.push(layer);
                *active_layer = Some(layers.len() - 1);
                drop(state);
                renderer::render(&state_ref);
            }
            _ => {
                let context = state.context().clone();
                if let Some(active_layer) = state.active_layer {
                    let maybe_edge = state.layers[active_layer].point_over_edge(&context, point);
                    if let Some(edge) = maybe_edge {
                        edge.set_cursor(state.canvas())
                    } else {
                        set_default_cursor(state.canvas())
                    }
                } else {
                    set_default_cursor(state.canvas())
                }

                let maybe_outlined_layer = state
                    .layers
                    .iter()
                    .rev()
                    .position(|layer| layer.object.is_point_over(&context, point))
                    .map(|idx| state.layers.len() - 1 - idx);
                if let Some(outlined_layer) = maybe_outlined_layer {
                    state.outlined_layer = Some(outlined_layer);
                    drop(state);
                    renderer::render(&state_ref);
                } else if state.outlined_layer.is_some() {
                    state.outlined_layer = None;
                    drop(state);
                    renderer::render(&state_ref);
                }
            }
        };
    })
}

fn on_mouseup(state_ref: &Rc<RefCell<State>>) -> Closure<dyn FnMut(MouseEvent)> {
    closure!({ state_ref }, move |_event: MouseEvent| {
        state_ref.borrow_mut().mouse_start = None;
        state_ref.borrow_mut().active_edge = None;
    })
}

fn get_mouse_position(canvas: &HtmlCanvasElement, event: &MouseEvent) -> Point {
    let rect = canvas.get_bounding_client_rect();
    Point {
        x: event.client_x() as f64 - rect.left(),
        y: event.client_y() as f64 - rect.top(),
    }
}

fn set_default_cursor(canvas: &HtmlCanvasElement) {
    canvas.style().set_property("cursor", "auto").unwrap();
}
