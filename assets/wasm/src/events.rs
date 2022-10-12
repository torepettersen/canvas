use crate::layers;
use crate::layers::Layer;
use crate::layers::Point;
use crate::state::State;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use web_sys::MouseEvent;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

macro_rules! closure {
    ( { $($x:ident),* }, $y:expr ) => {
        Closure::new({
            $(let $x = $x.clone();)*
            $y
        })
    }
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
    })
}

fn on_mousemove(state_ref: &Rc<RefCell<State>>) -> Closure<dyn FnMut(MouseEvent)> {
    closure!({ state_ref }, move |event: MouseEvent| {
        let mut state = state_ref.borrow_mut();
        match *state {
            State { mouse_start: Some(mouse_start), active_layer: Some(active_layer), .. } => {
                let mouse_end = get_mouse_position(state.canvas(), &event);
                state.layers[active_layer] =
                    Layer { object: layers::new_rect(mouse_start, mouse_end) };
                drop(state);
                layers::render(&state_ref);
            }
            State { mouse_start: Some(mouse_start), .. } => {
                let mouse_end = get_mouse_position(state.canvas(), &event);
                let layer = Layer { object: layers::new_rect(mouse_start, mouse_end) };
                state.layers.push(layer);
                state.active_layer = Some(state.layers.len() - 1);
                drop(state);
                layers::render(&state_ref);
            }
            _ => (),
        };
    })
}

fn on_mouseup(state_ref: &Rc<RefCell<State>>) -> Closure<dyn FnMut(MouseEvent)> {
    closure!({ state_ref }, move |event: MouseEvent| {
        let mut state = state_ref.borrow_mut();
        if let Some(mouse_start) = state.mouse_start {
            state.mouse_start = None;
            let mouse_end = get_mouse_position(state.canvas(), &event);
            let layer = Layer { object: layers::new_rect(mouse_start, mouse_end) };
            state.layers.push(layer);
            drop(state);
            layers::render(&state_ref);
        }
    })
}

fn get_mouse_position(canvas: &HtmlCanvasElement, event: &MouseEvent) -> Point {
    let rect = canvas.get_bounding_client_rect();
    Point {
        x: event.client_x() - rect.left() as i32,
        y: event.client_y() - rect.top() as i32,
    }
}
