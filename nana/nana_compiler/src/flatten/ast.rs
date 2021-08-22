//! In flattened Ast all `Block`s are gated.
//! All unwanted single tuples will be reduced to `Expr` form.

pub use crate::base::*;

#[derive(Clone, Debug)]
pub struct Nana {
    pub body: GatedBlock,
}

#[derive(Clone, Debug)]
pub struct GatedBlock {
    pub traces: Vec<Binder>,
    pub block: Block
}

#[derive(Clone, Debug)]
pub enum Block {
    Tuple(Vec<Abstraction>, Vec<Expr>),
    List(Vec<Abstraction>, Vec<Expr>),
    Set(Vec<Abstraction>, Vec<Expr>),
    // Map(Vec<Abstraction>, Vec<Pair>),
}

#[derive(Clone, Debug)]
pub struct Abstraction {
    pub trace: Binder,
    pub exposed: bool,
    pub src: GatedBlock,
}

#[derive(Clone, Debug)]
pub struct Pair {
    pub key: Expr,
    pub val: Expr,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Literal(Literal),
    Binder(Binder),
    GatedBlock(GatedBlock),
    Application(Box<Expr>, Box<Expr>),
    Projection(Box<Expr>, Binder),
}
