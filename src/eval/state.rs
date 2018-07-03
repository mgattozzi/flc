use im::HashMap;
use prim::Primitive;

#[derive(Debug, Clone)]
pub struct State {
    root: HashMap<String, Primitive>,
}

impl State {
    pub fn instantiate() -> Self {
        let root = HashMap::new();
        State { root }
    }
}
