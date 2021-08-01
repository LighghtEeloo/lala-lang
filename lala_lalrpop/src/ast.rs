pub type Binder = String;

#[derive(Debug)]
pub struct Binding {
    pub name: Binder,
    pub expose: Option<Vec<Binder>>,
    pub block: Block
}

#[derive(Debug)]
pub struct Block {
    pub bindings: Vec<Binding>
}