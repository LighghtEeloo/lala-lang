pub use crate::base::*;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Context {
    pub binded: Vec<Binder>,
    pub intro: Vec<(Binder, Arc<Ctx<GatedBlock>>)>,
}

#[derive(Clone, Debug)]
pub struct Ctx<T> (Context, T);

impl<T> Ctx<T> {
    pub fn new(c: Context, t: T) -> Self {
        Ctx(c, t)
    }
}
impl<T> Inner for Ctx<T> {
    type Target = T;
    fn inner(self) -> Self::Target {
        self.1
    }
    fn inner_ref(&self) -> &Self::Target {
        &self.1
    }
    fn inner_mut(&mut self) -> &mut Self::Target {
        &mut self.1
    }
}

#[derive(Clone, Debug)]
pub struct Nana {
    pub body: Ctx<GatedBlock>,
}

#[derive(Clone, Debug)]
pub struct GatedBlock {
    pub block: Ctx<Block>
}

#[derive(Clone, Debug)]
pub enum Block {
    Tuple(Vec<Expr>),
    List(Vec<Expr>),
    Set(Vec<Expr>),
    Map(Vec<Pair>),
}

#[derive(Clone, Debug)]
pub struct Abstraction {
    pub trace: Binder,
    pub exposed: bool,
    pub src: Ctx<GatedBlock>,
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
    GatedBlock(Ctx<GatedBlock>),
    Application(Box<Expr>, Box<Expr>),
    Projection(Box<Expr>, Binder),
}
