use crate::state::State;
use std::cell::RefCell;
use std::rc::Rc;
use std::cmp;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
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

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug)]
struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Rect {
    fn new(a: Point, b: Point) -> Rect {
        let x_start = cmp::min(a.x, b.x);
        let y_start = cmp::min(a.y, b.y);
        let x_end = cmp::max(a.x, b.x);
        let y_end = cmp::max(a.y, b.y);

        Rect {
            x: x_start,
            y: y_start,
            width: x_end - x_start,
            height: y_end - y_start,
        }
    }

    fn draw(&self, context: &CanvasRenderingContext2d) {
        context.rect(self.x.into(), self.y.into(), self.width.into(), self.height.into());
        context.stroke();
    }
}

pub struct Events {
    state: Rc<RefCell<State>>,
}

impl Events {
    pub fn new(state: &Rc<RefCell<State>>) -> Events {
        let events = Events {
            state: state.clone(),
        };

        events.on_mousedown();
        events.on_mouseup();

        events
    }

    fn on_mousedown(&self) {
        let state = &self.state;
        let on_mousedown: Closure<dyn FnMut(MouseEvent)> =
            closure!({ state }, move |event: MouseEvent| {
                let mut state = state.borrow_mut();
                state.mouse_start = Some(Self::get_mouse_position(state.canvas(), &event));
                log!("Point:{:?}", state.mouse_start);
            });
        let mut state = self.state.borrow_mut();
        state.canvas().set_onmousedown(Some(on_mousedown.as_ref().unchecked_ref()));
        state.add_closure(on_mousedown);
    }

    fn on_mouseup(&self) {
        let state = &self.state;
        let on_mouseup: Closure<dyn FnMut(MouseEvent)> =
            closure!({ state }, move |event: MouseEvent| {
                let mut state = state.borrow_mut();
                if let Some(mouse_start) = state.mouse_start {
                    state.mouse_start = None;
                    let mouse_end = Self::get_mouse_position(state.canvas(), &event);
                    Rect::new(mouse_start, mouse_end).draw(&state.context());
                    log!("From:{:?}, To:{:?}", mouse_start, mouse_end);
                }
            });
        let mut state = state.borrow_mut();
        state.canvas().set_onmouseup(Some(on_mouseup.as_ref().unchecked_ref()));
        state.add_closure(on_mouseup);
    }


    fn get_mouse_position(canvas: &HtmlCanvasElement, event: &MouseEvent) -> Point {
        let rect = canvas.get_bounding_client_rect();
        Point {
            x: event.client_x() - rect.left() as i32,
            y: event.client_y() - rect.top() as i32,
        }
    }
}
