use crate::flatten::ast as fa;
use crate::resolve::ast as ra;
use ra::{Context, Ctx};

pub trait Resolve<T> {
    fn resolve(self) -> T;
}

impl Resolve<ra::Nana> for fa::Nana {
    fn resolve(self) -> ra::Nana {
        ra::Nana {
            body: self.body.resolve(),
        }
    }
}

impl Resolve<Ctx<ra::GatedBlock>> for fa::GatedBlock {
    fn resolve(self) -> Ctx<ra::GatedBlock> {
        let c = Context {
            binded: self.traces,
            intro: Vec::new(),
        };
        Ctx::new(c, ra::GatedBlock {
            block: self.block.resolve()
        })
    }
}

impl Resolve<Ctx<ra::Block>> for fa::Block {
    fn resolve(self) -> Ctx<ra::Block> {
        match self {
            fa::Block::Tuple(_abs, _) => {
                todo!()
            }
            fa::Block::List(_abs, _) => {
                todo!()
            }
            fa::Block::Set(_abs, _) => {
                todo!()
            }
            fa::Block::Map(_abs, _) => {
                todo!()
            }
        }
    }
}

impl Resolve<ra::Expr> for fa::Expr {
    fn resolve(self) -> ra::Expr {
        todo!()
    }
}
