use crate::state::State;
use std::cell::RefCell;
use std::rc::Rc;

pub fn render(state: &Rc<RefCell<State>>) {
    let state = state.borrow_mut();
    let canvas = state.canvas();
    let context = state.context();

    context.begin_path();
    context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

    for layer in &state.layers {
        layer.object.draw(&context);
    }
    if let Some(outlined_layer) = state.outlined_layer {
        state.layers[outlined_layer].object.draw_outline(&context);
    }
    if let Some(active_layer) = state.active_layer {
        state.layers[active_layer].object.draw_active(&context);
    }
}
