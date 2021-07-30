pub type Binder = String;

pub struct Binding {
    pub name: Binder,
    pub expose: Option<Vec<Binder>>,
    pub block: Block
}

pub struct Block {
    pub bindings: Vec<Binding>
}