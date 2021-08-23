//! External Language Ast. The language ast for nana.

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
    Map(Vec<Abstraction>, Vec<Pair>),
}

pub type BlockInner = (Vec<Abstraction>, Vec<Expr>);

#[derive(Clone, Debug)]
pub struct Abstraction {
    pub trace: Binder,
    pub exposed: bool,
    pub src: Expr,
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
    Block(Block),
    GatedBlock(GatedBlock),
    Application(Box<Expr>, Box<Expr>),
    Projection(Box<Expr>, Binder),
}

mod construct {
    use super::*;

    impl From<BlockInner> for Nana {
        fn from(bi: BlockInner) -> Self {
            Self { body: bi.into() }
        }
    }

    impl From<BlockInner> for GatedBlock {
        fn from(bi: BlockInner) -> Self {
            Block::from(bi).into()
        }
    }
    impl From<Expr> for GatedBlock {
        fn from(e: Expr) -> Self {
            Block::from(e).into()
        }
    }
    impl From<Block> for GatedBlock {
        fn from(block: Block) -> Self {
            (Vec::new(), block).into()
        }
    }
    impl From<(Vec<Binder>, Block)> for GatedBlock {
        fn from((traces, block): (Vec<Binder>, Block)) -> Self {
            Self { traces, block }
        }
    }
    impl From<(Vec<Binder>, Expr)> for GatedBlock {
        fn from((traces, expr): (Vec<Binder>, Expr)) -> Self {
            (traces, Block::from(expr)).into()
        }
    }

    impl From<BlockInner> for Block {
        fn from((bds, vls): BlockInner) -> Self {
            Block::Tuple(bds, vls)
        }
    }
    impl From<Expr> for Block {
        fn from(e: Expr) -> Self {
            match e {
                Expr::Block(blk) => blk,
                _ => {
                    Block::Tuple(Vec::new(), vec![e])
                }
            }
        }
    }
    impl From<GatedBlock> for Block {
        fn from(e: GatedBlock) -> Self {
            Expr::GatedBlock(e).into()
        }
    }

    impl From<(Binder, bool, Expr)> for Abstraction {
        fn from((trace, exposed, src): (Binder, bool, Expr)) -> Self {
            Self { trace, exposed, src }
        }
    }

    impl From<(Expr, Expr)> for Pair {
        fn from((key, val): (Expr, Expr)) -> Self {
            Self { key, val }
        }
    }
}
