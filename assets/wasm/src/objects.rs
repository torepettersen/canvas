use crate::events::Point;
use web_sys::CanvasRenderingContext2d;

pub trait Object {
    fn draw(&self, context: &CanvasRenderingContext2d);
    fn draw_outline(&self, context: &CanvasRenderingContext2d);
    fn draw_active(&self, context: &CanvasRenderingContext2d);
    fn is_mouse_over(&self, context: &CanvasRenderingContext2d, point: Point) -> bool;
    fn corners(&self) -> Vec<Corner>;
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
        let x_start = f64::min(a.x, b.x);
        let y_start = f64::min(a.y, b.y);
        let x_end = f64::max(a.x, b.x);
        let y_end = f64::max(a.y, b.y);

        Rect { x: x_start, y: y_start, width: x_end - x_start, height: y_end - y_start }
    }

    pub fn from_center(center: Point, width: f64, height: f64) -> Rect {
        let x = center.x - width / 2.0;
        let y = center.y - height / 2.0;
        Rect { x: x, y: y, width: width, height: height }
    }

    pub fn top_left(&self) -> Point {
        Point { x: self.x, y: self.y }
    }

    pub fn top_right(&self) -> Point {
        Point { x: self.x + self.width, y: self.y }
    }

    pub fn bottom_right(&self) -> Point {
        Point { x: self.x + self.width, y: self.y + self.height }
    }

    pub fn bottom_left(&self) -> Point {
        Point { x: self.x, y: self.y + self.height }
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
        context.rect(
            x - offset,
            y - offset,
            width + stroke_width,
            height + stroke_width,
        );
        context.set_stroke_style(&"#3782F7".into());
        context.set_line_width(stroke_width);
        context.stroke();
    }

    fn draw_active(&self, context: &CanvasRenderingContext2d) {
        self.draw_outline(context);

        for corner in self.corners() {
            corner.draw(context);
        }
    }

    fn is_mouse_over(&self, context: &CanvasRenderingContext2d, point: Point) -> bool {
        context.begin_path();
        context.rect(self.x, self.y, self.width, self.height);
        context.is_point_in_path_with_f64(point.x.into(), point.y.into())
    }

    fn corners(&self) -> Vec<Corner> {
        vec![
            Corner::new(self.top_left(), "nwse-resize"),
            Corner::new(self.top_right(), "nesw-resize"),
            Corner::new(self.bottom_right(), "nwse-resize"),
            Corner::new(self.bottom_left(), "nesw-resize"),
        ]
    }
}

#[derive(Clone, Debug)]
pub struct Corner {
    rect: Rect,
    cursor: String,
}

impl Corner {
    pub fn new(point: Point, cursor: &str) -> Corner {
        let size = 8.0;
        Corner { rect: Rect::from_center(point, size, size), cursor: cursor.into() }
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d) {
        self.rect.draw(context);
    }

    pub fn cursor(&self) -> &String {
        &self.cursor
    }

    pub fn is_mouse_over(&self, context: &CanvasRenderingContext2d, point: Point) -> bool {
        self.rect.is_mouse_over(context, point)
    }
}
