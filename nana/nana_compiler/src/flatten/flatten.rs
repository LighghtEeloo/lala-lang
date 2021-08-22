//! Performs flatten.

use super::ast as fa;
use crate::external::ast as ea;

/// Flatten all tuples; add gate to all blocks.
pub trait Flatten<T> {
    fn flatten(self) -> T;
}

impl Flatten<fa::Nana> for ea::Nana {
    fn flatten(self) -> fa::Nana {
        fa::Nana { body: self.body.flatten() }
    }
}

impl Flatten<fa::GatedBlock> for ea::GatedBlock {
    fn flatten(self) -> fa::GatedBlock {
        fa::GatedBlock {
            traces: self.traces,
            block: self.block.flatten(),
        }
    }
}

impl Flatten<fa::Expr> for ea::GatedBlock {
        // println!("{:#?}", self);
    fn flatten(self) -> fa::Expr {
        let traces = self.traces;
        let block = self.block;
        match (traces.is_empty(), block) {
            (true, ea::Block::Tuple(bds, vls)) => {
                ea::Block::Tuple(bds, vls).flatten()
            }
            (_, block) => {
                fa::Expr::GatedBlock(fa::GatedBlock {
                    traces,
                    block: block.flatten(),
                })
            }
        }
    }
}

impl Flatten<fa::Block> for ea::Block {
    fn flatten(self) -> fa::Block {
        match self {
            ea::Block::Tuple(bds, vls) => {
                fa::Block::Tuple(
                    bds.into_iter().map(|x| x.flatten()).collect(),
                    vls.into_iter().map(|x| x.flatten()).collect()
                )
            }
            ea::Block::List(bds, vls) => {
                fa::Block::List(
                    bds.into_iter().map(|x| x.flatten()).collect(),
                    vls.into_iter().map(|x| x.flatten()).collect()
                )
            }
            ea::Block::Set(bds, vls) => {
                fa::Block::Set(
                    bds.into_iter().map(|x| x.flatten()).collect(),
                    vls.into_iter().map(|x| x.flatten()).collect()
                )
            }
            ea::Block::Map(bds, vls) => {
                fa::Block::Map(
                    bds.into_iter().map(|x| x.flatten()).collect(),
                    vls.into_iter().map(|x| x.flatten()).collect()
                )
            }
        }
    }
}

impl Flatten<fa::Abstraction> for ea::Abstraction {
    fn flatten(self) -> fa::Abstraction {
        fa::Abstraction {
            trace: self.trace,
            exposed: self.exposed,
            src: self.src.flatten(),
        }
    }
}

impl Flatten<fa::GatedBlock> for ea::Expr {
    fn flatten(self) -> fa::GatedBlock {
        match self {
            ea::Expr::Block(block) => {
                fa::GatedBlock {
                    traces: Vec::new(),
                    block: block.flatten(),
                }
            }
            ea::Expr::GatedBlock(g) => g.flatten(),
            ea::Expr::Literal(_) |
            ea::Expr::Binder(_) |
            ea::Expr::Application(_, _) |
            ea::Expr::Projection(_, _) => {
                fa::GatedBlock {
                    traces: Vec::new(),
                    block: fa::Block::Tuple(Vec::new(), vec![self.flatten()]),
                }
            }
        }
    }
}

impl Flatten<fa::Expr> for ea::Expr {
    fn flatten(self) -> fa::Expr {
        match self {
            ea::Expr::Literal(l) => fa::Expr::Literal(l),
            ea::Expr::Binder(b) => fa::Expr::Binder(b),
            ea::Expr::Block(block) => block.flatten(),
            ea::Expr::GatedBlock(g) => g.flatten(),
            ea::Expr::Application(e1, e2) => {
                fa::Expr::Application(
                    Box::new(e1.flatten()), 
                    Box::new(e2.flatten())
                )
            }
            ea::Expr::Projection(e, b) => {
                fa::Expr::Projection(Box::new(e.flatten()), b)
            }
        }
    }
}

impl Flatten<fa::Expr> for ea::Block {
    fn flatten(self) -> fa::Expr {
        match self {
            ea::Block::Tuple(bds, vls) => {
                match (bds.len(), vls.len()) {
                    (0, 1) => {
                        let e = unsafe { vls.get_unchecked(0).clone() };
                        e.flatten()
                    }
                    _ => {
                        fa::Expr::GatedBlock(fa::GatedBlock {
                            traces: Vec::new(),
                            block: ea::Block::Tuple(bds, vls).flatten(),
                        })
                    }
                }
            }
            ea::Block::List(_, _) |
            ea::Block::Set(_, _) |
            ea::Block::Map(_, _) => {
                fa::Expr::GatedBlock(fa::GatedBlock {
                    traces: Vec::new(),
                    block: self.flatten(),
                })
            }
        }
    }
}

impl Flatten<fa::Pair> for ea::Pair {
    fn flatten(self) -> fa::Pair {
        fa::Pair {
            key: self.key.flatten(),
            val: self.val.flatten(),
        }
    }
}
