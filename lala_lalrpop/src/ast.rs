#[derive(Debug)]
pub enum Binder {
    Identity(String),
    Anonymous(String),
    Arbitrary
}

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

impl From<Vec<Binder>> for Mask {
    fn from(binders: Vec<Binder>) -> Self {
        Mask::Exposing { binders }
    }
}

#[derive(Debug)]
pub struct Sequential {
    bindings: Vec<Binding>
}

impl From<Vec<Binding>> for Sequential {
    fn from(bindings: Vec<Binding>) -> Self {
        Sequential { bindings }
    }
}

#[derive(Debug)]
pub struct Parallel {
    bindings: Vec<Binding>
}

impl From<Vec<Binding>> for Parallel {
    fn from(bindings: Vec<Binding>) -> Self {
        Parallel { bindings }
    }
}

#[derive(Debug)]
pub enum Block {
    Sequential(Sequential),
    Parallel(Parallel),
}

impl From<Sequential> for Block {
    fn from(seq: Sequential) -> Self {
        Block::Sequential(seq)
    }
}

impl From<Parallel> for Block {
    fn from(par: Parallel) -> Self {
        Block::Parallel(par)
    }
}
