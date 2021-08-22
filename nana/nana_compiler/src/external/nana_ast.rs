pub use crate::base::*;

#[derive(Clone, Debug)]
pub struct Nana {
    pub body: GatedBlock,
}

#[derive(Clone, Debug)]
pub struct GatedBlock {
    pub trace: Option<Binder>,
    pub block: Block
}

#[derive(Clone, Debug)]
pub enum Block {
    Tuple(Vec<Binding>, Vec<Expr>),
    List(Vec<Binding>, Vec<Expr>),
    Set(Vec<Binding>, Vec<Expr>),
    Map(Vec<Binding>, Vec<Pair>),
}

pub type BlockInner = (Vec<Binding>, Vec<Expr>);

#[derive(Clone, Debug)]
pub struct Binding {
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
            Self { trace: None, block }
        }
    }
    /// Insert from last to first.
    impl From<(Vec<Binder>, Block)> for GatedBlock {
        fn from((mut traces, block): (Vec<Binder>, Block)) -> Self {
            let head = traces.pop();
            match head {
                Some(t) => Self {
                    trace: Some(t),
                    block: GatedBlock::from((traces, block)).into(),
                },
                None => block.into(),
            }
        }
    }

    impl From<BlockInner> for Block {
        fn from((bds, vls): BlockInner) -> Self {
            Block::Tuple(bds, vls)
        }
    }
    impl From<Expr> for Block {
        fn from(e: Expr) -> Self {
            Block::Tuple(Vec::new(), vec![e])
        }
    }
    impl From<GatedBlock> for Block {
        fn from(e: GatedBlock) -> Self {
            Expr::GatedBlock(e).into()
        }
    }

    impl From<(Expr, Expr)> for Pair {
        fn from((key, val): (Expr, Expr)) -> Self {
            Self { key, val }
        }
    }
}
