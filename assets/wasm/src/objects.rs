use crate::events::Point;
use web_sys::CanvasRenderingContext2d;

#[derive(Clone, Copy, Debug)]
pub enum Object {
    Rect {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },
}

impl Object {
    pub fn draw(&self, context: &CanvasRenderingContext2d) {
        match *self {
            Object::Rect { x, y, width, height } => {
                context.begin_path();
                context.rect(x, y, width, height);
                context.fill();
            }
        }
    }

    pub fn draw_outline(&self, context: &CanvasRenderingContext2d) {
        match self {
            Object::Rect { x, y, width, height } => {
                context.begin_path();
                context.rect(x - 1.5, y - 1.5, width + 3.0, height + 3.0);
                context.set_stroke_style(&"red".into());
                context.set_line_width(3.0);
                context.stroke();
            }
        }
    }

    pub fn is_mouse_over(&self, context: &CanvasRenderingContext2d, point: Point) -> bool {
        match *self {
            Object::Rect { x, y, width, height } => {
                context.begin_path();
                context.rect(x, y, width, height);
                context.is_point_in_path_with_f64(point.x.into(), point.y.into())
            }
        }
    }
}

pub mod rect {
    use super::Object;
    use crate::events::Point;
    use std::cmp;

    pub fn new(a: Point, b: Point) -> Object {
        let x_start = cmp::min(a.x, b.x) as f64;
        let y_start = cmp::min(a.y, b.y) as f64;
        let x_end = cmp::max(a.x, b.x) as f64;
        let y_end = cmp::max(a.y, b.y) as f64;

        Object::Rect { x: x_start, y: y_start, width: x_end - x_start, height: y_end - y_start }
    }
}
