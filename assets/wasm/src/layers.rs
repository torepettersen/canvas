use crate::events::Point;
use crate::objects::Edge;
use crate::objects::Object;
use web_sys::CanvasRenderingContext2d;

pub struct Layer {
    pub object: Box<dyn Object>,
}

impl Layer {
    pub fn point_over_edge(
        &self,
        context: &CanvasRenderingContext2d,
        point: Point,
    ) -> Option<Edge> {
        self.object
            .edges()
            .into_iter()
            .find(|edge| edge.is_point_over(&context, point))
    }
}
