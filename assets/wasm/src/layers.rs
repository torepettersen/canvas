use crate::state::State;
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;
use web_sys::CanvasRenderingContext2d;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug)]
pub enum Object {
    Rect {
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    },
}

impl Object {
    pub fn draw(&self, context: &CanvasRenderingContext2d) {
        match self {
            Object::Rect { x, y, width, height } => {
                context.begin_path();
                context.rect(*x as f64, *y as f64, *width as f64, *height as f64);
                context.fill();
            }
        }
    }
}

pub fn new_rect(a: Point, b: Point) -> Object {
    let x_start = cmp::min(a.x, b.x);
    let y_start = cmp::min(a.y, b.y);
    let x_end = cmp::max(a.x, b.x);
    let y_end = cmp::max(a.y, b.y);

    Object::Rect { x: x_start, y: y_start, width: x_end - x_start, height: y_end - y_start }
}

pub struct Layer {
    pub object: Object,
}

pub fn render(state: &Rc<RefCell<State>>) {
    let state = state.borrow_mut();
    let canvas = state.canvas();
    let context = state.context();

    context.begin_path();
    context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

    for layer in &state.layers {
        layer.object.draw(&context);
    }
}
