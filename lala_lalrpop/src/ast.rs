pub type Binder = String;

#[derive(Debug)]
pub struct Binding {
    pub binder: Binder,
    pub args: Vec<Binder>,
    pub mask: Mask,
    pub block: Block
}

impl From<(Binder, Mask, Block)> for Binding {
    fn from((binder, mask, block): (Binder, Mask, Block)) -> Self {
        Binding { binder, args: Vec::new(), mask, block }
    }
}

impl From<(Binder, Vec<Binder>, Mask, Block)> for Binding {
    fn from((binder, args, mask, block): (Binder, Vec<Binder>, Mask, Block)) -> Self {
        Binding { binder, args, mask, block }
    }
}

#[derive(Debug)]
pub enum Mask {
    Closed,
    Exposed,
    Exposing {
        binders: Vec<Binder>
    },
    Open
}

#[derive(Debug)]
pub struct Block {
    pub bindings: Vec<Binding>
}

impl From<Vec<Binding>> for Block {
    fn from(bindings: Vec<Binding>) -> Self {
        Block {
            bindings
        }
    }
}