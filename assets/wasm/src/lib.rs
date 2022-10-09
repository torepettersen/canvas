mod events;
mod state;

use events::Events;
use state::State;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Editor {
    _events: Events,
}

#[wasm_bindgen]
impl Editor {
    pub fn new(canvas_id: &str) -> Result<Editor, JsValue> {
        let state = State::new(canvas_id)?;

        let editor = Editor {
            _events: Events::new(&state),
        };

        let state = state.borrow();
        let canvas = state.canvas();

        canvas.set_width(800);
        canvas.set_height(400);
        canvas.style().set_property("background", "white")?;

        Ok(editor)
    }
}
