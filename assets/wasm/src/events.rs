use crate::state::State;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
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

pub enum Event {
    MouseDown { point: Point },
    MouseMove { point: Point },
    MouseUp { point: Point },
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
        let state = state.borrow();
        let canvas = state.canvas.borrow();
        let point = canvas.get_mouse_position(&event);
        state
            .layers
            .borrow_mut()
            .on_event(Event::MouseDown { point }, &canvas);
    })
}

fn on_mousemove(state: &Rc<RefCell<State>>) -> Closure<dyn FnMut(MouseEvent)> {
    closure!({ state }, move |event: MouseEvent| {
        let state = state.borrow();
        let canvas = state.canvas.borrow();
        let point = canvas.get_mouse_position(&event);
        state
            .layers
            .borrow_mut()
            .on_event(Event::MouseMove { point }, &canvas);
    })
}

fn on_mouseup(state: &Rc<RefCell<State>>) -> Closure<dyn FnMut(MouseEvent)> {
    closure!({ state }, move |event: MouseEvent| {
        let state = state.borrow();
        let canvas = state.canvas.borrow();
        let point = canvas.get_mouse_position(&event);
        state
            .layers
            .borrow_mut()
            .on_event(Event::MouseUp { point }, &canvas);
    })
}
