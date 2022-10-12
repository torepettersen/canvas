
use web_sys::CanvasRenderingContext2d;

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
                crate::log!("hei");
            }
        }
    }
}

pub mod rect {
    use super::Object;
    use crate::events::Point;
    use std::cmp;

    pub fn new(a: Point, b: Point) -> Object {
        let x_start = cmp::min(a.x, b.x);
        let y_start = cmp::min(a.y, b.y);
        let x_end = cmp::max(a.x, b.x);
        let y_end = cmp::max(a.y, b.y);

        Object::Rect { x: x_start, y: y_start, width: x_end - x_start, height: y_end - y_start }
    }
}
