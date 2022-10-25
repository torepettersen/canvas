use crate::canvas::Canvas;
use crate::events::Event;
use crate::events::Point;
use crate::iter_ext::IterExt;
use crate::objects::Edge;
use crate::objects::Object;
use crate::objects::Rect;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Layer {
    pub object: Box<dyn Object>,
}

impl Layer {
    pub fn point_over_edge(&self, canvas: &Canvas, point: Point) -> Option<Edge> {
        let context = canvas.context();
        self.object
            .edges()
            .into_iter()
            .find(|edge| edge.is_point_over(&context, point))
    }
}

pub enum LayerState {
    ToCreateLayer { start: Point },
    CreatingLayer { layer: usize, start: Point },
    IdleLayer { layer: usize },
    ResizeLayer { layer: usize, edge: Edge },
    RelocateLayer { layer: usize, grab_point: Point },
}

pub struct Layers {
    layers: Vec<Layer>,
    active_layer: Option<LayerState>,
    outlined_layer: Option<usize>,
}

impl Layers {
    pub fn new() -> Self {
        Layers { layers: Vec::new(), active_layer: None, outlined_layer: None }
    }

    pub fn layers(&self) -> &Vec<Layer> {
        &self.layers
    }

    pub fn outlined_layer(&self) -> &Option<usize> {
        &self.outlined_layer
    }

    pub fn active_layer(&self) -> &Option<LayerState> {
        &self.active_layer
    }

    pub fn on_event(&mut self, event: Event, canvas: &Canvas) {
        match event {
            Event::MouseDown { point } => self.on_mouse_down(point, canvas),
            Event::MouseMove { point } => self.on_mouse_move(point, canvas),
            Event::MouseUp { point } => self.on_mouse_up(point, canvas),
        }

        match event {
            Event::MouseDown { point } | Event::MouseMove { point } | Event::MouseUp { point } => {
                self.set_cursor(point, canvas);
            }
        }
    }

    fn on_mouse_down(&mut self, point: Point, canvas: &Canvas) {
        match self.active_layer {
            None => {
                let maybe_active_layer = self.find_layer_from_point(point, canvas);
                if let Some(layer) = maybe_active_layer {
                    self.active_layer = Some(LayerState::IdleLayer { layer });
                } else {
                    self.active_layer = Some(LayerState::ToCreateLayer { start: point });
                }
                canvas.render(self);
            }
            Some(LayerState::IdleLayer { layer }) => {
                let active_layer = &self.layers[layer];
                if let Some(edge) = active_layer.point_over_edge(canvas, point) {
                    self.active_layer = Some(LayerState::ResizeLayer { layer, edge });
                } else if let Some(layer) = self.find_layer_from_point(point, canvas) {
                    let grab_point = self.layers[layer].object.grab_point(point);
                    self.active_layer = Some(LayerState::RelocateLayer { layer, grab_point });
                } else {
                    self.active_layer = None;
                }
                canvas.render(self);
            }
            _ => {}
        }
    }

    fn on_mouse_move(&mut self, point: Point, canvas: &Canvas) {
        match self.active_layer {
            Some(LayerState::ToCreateLayer { start }) => {
                self.layers
                    .push(Layer { object: Box::new(Rect::new(start, point)) });
                let layer = self.layers.len() - 1;
                self.active_layer = Some(LayerState::CreatingLayer { layer, start });
                canvas.render(self);
            }
            Some(LayerState::CreatingLayer { layer, start }) => {
                self.layers[layer] = Layer { object: Box::new(Rect::new(start, point)) };
                canvas.render(self);
            }
            Some(LayerState::ResizeLayer { layer, edge }) => {
                self.layers[layer].object.resize(point, edge);
                canvas.render(self);
            }
            Some(LayerState::RelocateLayer { layer, grab_point }) => {
                self.layers[layer].object.relocate(point, grab_point);
                canvas.render(self);
            }
            _ => {
                // Outlined layer
                let maybe_outlined_layer = self.find_layer_from_point(point, canvas);
                if let Some(outlined_layer) = maybe_outlined_layer {
                    self.outlined_layer = Some(outlined_layer);
                    canvas.render(self);
                } else if self.outlined_layer.is_some() {
                    self.outlined_layer = None;
                    canvas.render(self);
                }
            }
        }
    }

    fn on_mouse_up(&mut self, _point: Point, _canvas: &Canvas) {
        match self.active_layer {
            Some(LayerState::ToCreateLayer { .. }) => {
                self.active_layer = None;
            }
            Some(LayerState::CreatingLayer { layer, .. })
            | Some(LayerState::ResizeLayer { layer, .. })
            | Some(LayerState::RelocateLayer { layer, .. }) => {
                self.active_layer = Some(LayerState::IdleLayer { layer });
            }
            _ => {}
        }
    }

    fn set_cursor(&self, point: Point, canvas: &Canvas) {
        match self.active_layer {
            Some(LayerState::IdleLayer { layer }) => {
                let maybe_edge = self.layers[layer].point_over_edge(canvas, point);
                if let Some(edge) = maybe_edge {
                    edge.set_cursor(canvas.canvas())
                } else {
                    set_default_cursor(canvas)
                }
            }
            Some(LayerState::ResizeLayer { edge, .. }) => edge.set_cursor(canvas.canvas()),
            _ => set_default_cursor(canvas),
        }
    }

    fn last_item(&self) -> usize {
        self.layers.len() - 1
    }

    fn find_layer_from_point(&self, point: Point, canvas: &Canvas) -> Option<usize> {
        self.layers
            .iter()
            .rev()
            .position(|layer| layer.object.is_point_over(canvas.context(), point))
            .map(|idx| self.last_item() - idx)
    }
}

impl Into<Rc<RefCell<Layers>>> for Layers {
    fn into(self) -> Rc<RefCell<Layers>> {
        Rc::new(RefCell::new(self))
    }
}

fn set_default_cursor(canvas: &Canvas) {
    canvas
        .canvas()
        .style()
        .set_property("cursor", "auto")
        .unwrap();
}
