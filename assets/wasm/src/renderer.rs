
use crate::state::State;

struct Renderer {
    state: Rc<RefCell<State>>,
}
    
impl Renderer {
    pub fn new(state: &Rc<RefCell<State>>) {
        Renderer {
            state: state.clone(),
        }
    }

}
