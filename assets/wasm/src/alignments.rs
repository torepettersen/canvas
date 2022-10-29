use crate::iter_ext::IterExt;
use crate::layers::Layer;
use array_tool::vec::Intersect;

#[derive(Copy, Clone, Debug)]
pub enum Alignment {
    X { x: f64, top: f64, bottom: f64 },
    Y { y: f64, left: f64, right: f64 },
}

impl Alignment {
    pub fn from_layers(layers: &[Layer], active_layer_idx: usize) -> Vec<Alignment> {
        let active_layer = &layers[active_layer_idx];

        layers
            .iter()
            .skip_nth(active_layer_idx)
            .flat_map(|layer| Alignment::get_alignments(active_layer, layer))
            .collect()
    }

    fn get_alignments(active_layer: &Layer, layer: &Layer) -> Vec<Alignment> {
        let active_object = &active_layer.object;
        let object = &layer.object;

        let object_y = vec![object.top(), object.bottom()];
        let active_y = vec![active_object.top(), active_object.bottom()];
        let aligments_y = object_y
            .intersect(active_y)
            .into_iter()
            .map(|y| Alignment::Y { y, left: object.left(), right: object.right() });

        let object_x = vec![object.left(), object.right()];
        let active_x = vec![active_object.left(), active_object.right()];
        let aligments_x = object_x
            .intersect(active_x)
            .into_iter()
            .map(|x| Alignment::X { x, top: object.top(), bottom: object.bottom() });

        aligments_y.chain(aligments_x).collect()
    }
}
