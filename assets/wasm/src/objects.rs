use crate::events::Point;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

pub trait Object {
    fn draw(&self, context: &CanvasRenderingContext2d);
    fn draw_outline(&self, context: &CanvasRenderingContext2d);
    fn draw_active(&self, context: &CanvasRenderingContext2d);
    fn is_point_over(&self, context: &CanvasRenderingContext2d, point: Point) -> bool;
    fn edges(&self) -> Vec<Edge>;
    fn resize(&mut self, point: Point, edge: Edge);
    fn top(&self) -> f64;
    fn set_top(&mut self, y: f64);
    fn left(&self) -> f64;
    fn set_left(&mut self, x: f64);
    fn bottom(&self) -> f64;
    fn set_bottom(&mut self, y: f64);
    fn right(&self) -> f64;
    fn set_right(&mut self, x: f64);
    fn top_left(&self) -> Point {
        Point { x: self.left(), y: self.top() }
    }
    fn set_top_left(&mut self, point: Point) {
        self.set_top(point.y);
        self.set_left(point.x);
    }
    fn top_right(&self) -> Point {
        Point { x: self.right(), y: self.top() }
    }
    fn set_top_right(&mut self, point: Point) {
        self.set_top(point.y);
        self.set_right(point.x);
    }
    fn bottom_right(&self) -> Point {
        Point { x: self.right(), y: self.bottom() }
    }
    fn set_bottom_right(&mut self, point: Point) {
        self.set_bottom(point.y);
        self.set_right(point.x);
    }
    fn bottom_left(&self) -> Point {
        Point { x: self.left(), y: self.bottom() }
    }
    fn set_bottom_left(&mut self, point: Point) {
        self.set_bottom(point.y);
        self.set_left(point.x);
    }
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

        for edge in self.edges() {
            edge.draw(context);
        }
    }

    fn is_point_over(&self, context: &CanvasRenderingContext2d, point: Point) -> bool {
        context.begin_path();
        context.rect(self.x, self.y, self.width, self.height);
        context.is_point_in_path_with_f64(point.x.into(), point.y.into())
    }

    fn edges(&self) -> Vec<Edge> {
        vec![
            Edge::new(self.top_left(), EdgeKind::TopLeft),
            Edge::new(self.top_right(), EdgeKind::TopRight),
            Edge::new(self.bottom_right(), EdgeKind::BottomRight),
            Edge::new(self.bottom_left(), EdgeKind::BottomLeft),
        ]
    }

    fn top(&self) -> f64 {
        self.y
    }

    fn set_top(&mut self, y: f64) {
        self.height = self.bottom() - y;
        self.y = y;
    }

    fn left(&self) -> f64 {
        self.x
    }

    fn set_left(&mut self, x: f64) {
        self.width = self.right() - x;
        self.x = x;
    }

    fn bottom(&self) -> f64 {
        self.y + self.height
    }

    fn set_bottom(&mut self, y: f64) {
        self.height = y - self.y;
    }

    fn right(&self) -> f64 {
        self.x + self.width
    }

    fn set_right(&mut self, x: f64) {
        self.width = x - self.x;
    }

    fn resize(&mut self, point: Point, edge: Edge) {
        match edge.kind {
            EdgeKind::TopLeft => self.set_top_left(point),
            // EdgeKind::Top => "auto",
            EdgeKind::TopRight => self.set_top_right(point),
            // EdgeKind::Right => "auto",
            EdgeKind::BottomRight => self.set_bottom_right(point),
            // EdgeKind::Bottom => "auto",
            EdgeKind::BottomLeft => self.set_bottom_left(point),
            // EdgeKind::Left => "auto",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum EdgeKind {
    TopLeft,
    // Top,
    TopRight,
    // Right,
    BottomRight,
    // Bottom,
    BottomLeft,
    // Left,
}

#[derive(Clone, Copy, Debug)]
pub struct Edge {
    pub rect: Rect,
    kind: EdgeKind,
}

impl Edge {
    pub fn new(point: Point, kind: EdgeKind) -> Edge {
        let size = 8.0;
        Edge { rect: Rect::from_center(point, size, size), kind: kind }
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d) {
        self.rect.draw(context);
    }

    pub fn is_point_over(&self, context: &CanvasRenderingContext2d, point: Point) -> bool {
        self.rect.is_point_over(context, point)
    }

    pub fn set_cursor(&self, canvas: &HtmlCanvasElement) {
        canvas
            .style()
            .set_property("cursor", &self.cursor())
            .unwrap();
    }

    fn cursor(&self) -> &str {
        match self.kind {
            EdgeKind::TopLeft => "nwse-resize",
            // EdgeKind::Top => "auto",
            EdgeKind::TopRight => "nesw-resize",
            // EdgeKind::Right => "auto",
            EdgeKind::BottomRight => "nwse-resize",
            // EdgeKind::Bottom => "auto",
            EdgeKind::BottomLeft => "nesw-resize",
            // EdgeKind::Left => "auto",
        }
    }
}
