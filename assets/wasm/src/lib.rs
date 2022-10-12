mod events;
mod layers;
mod state;

use state::State;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Editor {
    _state: Rc<RefCell<State>>,
}

#[wasm_bindgen]
impl Editor {
    fn new(canvas_id: &str) -> Result<Editor, JsValue> {
        let state = State::new(canvas_id)?.to_ptr();
        events::init(&state);

        let editor = Editor { _state: state.clone() };

        let state = state.borrow();
        let canvas = state.canvas();

        canvas.set_width(800);
        canvas.set_height(400);
        canvas.style().set_property("background", "white")?;

        Ok(editor)
    }
}

#[wasm_bindgen]
pub fn start_editor(canvas_id: &str) -> Result<Editor, JsValue> {
    Editor::new(canvas_id)
}
