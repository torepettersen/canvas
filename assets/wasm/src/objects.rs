use crate::events::Point;
use std::cmp;
use web_sys::CanvasRenderingContext2d;

pub trait Object {
    fn draw(&self, context: &CanvasRenderingContext2d);
    fn draw_outline(&self, context: &CanvasRenderingContext2d);
    fn is_mouse_over(&self, context: &CanvasRenderingContext2d, point: Point) -> bool;
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl Rect {
    pub fn new(a: Point, b: Point) -> Rect {
        let x_start = cmp::min(a.x, b.x) as f64;
        let y_start = cmp::min(a.y, b.y) as f64;
        let x_end = cmp::max(a.x, b.x) as f64;
        let y_end = cmp::max(a.y, b.y) as f64;

        Rect { x: x_start, y: y_start, width: x_end - x_start, height: y_end - y_start }
    }
}

impl Object for Rect {
    fn draw(&self, context: &CanvasRenderingContext2d) {
        context.begin_path();
        context.rect(self.x, self.y, self.width, self.height);
        context.fill();
    }

    fn draw_outline(&self, context: &CanvasRenderingContext2d) {
        let stroke_width = 3.0;
        let offset = stroke_width / 2.0;
        let Rect { x, y, width, height } = self;
        context.begin_path();
        context.rect(x - offset, y - offset, width + stroke_width, height + stroke_width);
        context.set_stroke_style(&"#3782F7".into());
        context.set_line_width(stroke_width);
        context.stroke();
    }

    fn is_mouse_over(&self, context: &CanvasRenderingContext2d, point: Point) -> bool {
        context.begin_path();
        context.rect(self.x, self.y, self.width, self.height);
        context.is_point_in_path_with_f64(point.x.into(), point.y.into())
    }
}
