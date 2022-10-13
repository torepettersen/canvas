use crate::layers::Layer;
use crate::objects;
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
    pub x: i32,
    pub y: i32,
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

fn on_mousedown(state: &Rc<RefCell<State>>) -> Closure<dyn FnMut(MouseEvent)> {
    closure!({ state }, move |event: MouseEvent| {
        let mut state = state.borrow_mut();
        state.mouse_start = Some(get_mouse_position(state.canvas(), &event));
        state.active_layer = None;
    })
}

fn on_mousemove(state_ref: &Rc<RefCell<State>>) -> Closure<dyn FnMut(MouseEvent)> {
    closure!({ state_ref }, move |event: MouseEvent| {
        let mut state = state_ref.borrow_mut();
        match *state {
            State { mouse_start: Some(mouse_start), active_layer: Some(active_layer), .. } => {
                let mouse_end = get_mouse_position(state.canvas(), &event);
                state.layers[active_layer] =
                    Layer { object: objects::rect::new(mouse_start, mouse_end) };
                drop(state);
                renderer::render(&state_ref);
            }
            State { mouse_start: Some(mouse_start), .. } => {
                let mouse_end = get_mouse_position(state.canvas(), &event);
                let layer = Layer { object: objects::rect::new(mouse_start, mouse_end) };
                state.layers.push(layer);
                state.active_layer = Some(state.layers.len() - 1);
                drop(state);
                renderer::render(&state_ref);
            }
            _ => {
                let point = get_mouse_position(state.canvas(), &event);
                let context = state.context();
                let layer = state
                    .layers
                    .iter()
                    .rev()
                    .position(|layer| layer.object.is_mouse_over(context, point))
                    .map(|idx| state.layers.len() - 1 - idx);
                if let Some(layer) = layer {
                    state.outlined_layer = Some(layer);
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
    })
}

fn get_mouse_position(canvas: &HtmlCanvasElement, event: &MouseEvent) -> Point {
    let rect = canvas.get_bounding_client_rect();
    Point {
        x: event.client_x() - rect.left() as i32,
        y: event.client_y() - rect.top() as i32,
    }
}
