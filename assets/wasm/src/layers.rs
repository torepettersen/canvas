use crate::objects::Object;

pub struct Layer {
    pub object: Box<dyn Object>,
}
