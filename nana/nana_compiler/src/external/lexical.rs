use super::nana_ast::*;

pub trait Lexical<T> {
    fn lexical(self) -> T;
}

impl Lexical<Nana> for Nana {
    fn lexical(self) -> Nana {
        Self {
            body: self.body.lexical(),
        }
    }
}

impl Lexical<Expr> for Expr {
    fn lexical(self) -> Expr {
        match self {
            Expr::Abstraction(abs) => Expr::Abstraction(abs.lexical()),
            Expr::Application(_) => self,
            Expr::Projection(_) => self,
            Expr::GatedBlock(c) => c.lexical(),
            // Expr::ControlFlow(_) => self,
            _ => self
        }
    }
}

impl Lexical<Pair> for Pair {
    fn lexical(self) -> Pair {
        let Self { key, val } = self;
        Self {
            key: key.lexical(),
            val: val.lexical(),
        }
    }
}

impl Lexical<Abstraction> for Abstraction {
    fn lexical(self) -> Abstraction {
        let Self { pattern, mask, expr } = self;
        Self {
            pattern,
            mask,
            expr: Box::new(expr.lexical())
        }
    }
}

impl Lexical<Expr> for Closure {
    fn lexical(self) -> Expr {
        // println!("cls => {:?}", self);
        match self.para {
            Some(_) => Expr::GatedBlock(Self {
                para: self.para,
                block: self.block.lexical(),
            }),
            None => {
                self.block.lexical()
            }
        }
    }
}

impl Lexical<Block> for Block {
    fn lexical(self) -> Block {
        // println!("blk => {:?}", self);
        match self {
            Block::Vector(v)  => Block::Vector(v.lexical()),
            Block::Tuple(t) => Block::Tuple(t.lexical()),
            Block::HashSet(s) => Block::HashSet(s.lexical()),
            Block::HashMap(m) => Block::HashMap(m.lexical()),
        }
    }
}

impl Lexical<Expr> for Block {
    fn lexical(self) -> Expr {
        // println!("blk => {:?}", self);
        match self {
            Block::Vector(_)  => {
                let blk: Block = self.lexical();
                Expr::from(blk)
            }
            Block::Tuple(tup)   => {
                // let BlockInner { binder_space, value_space } = tup;
                match (tup.binder_space.len(), tup.value_space.len()) {
                    (0, 1) => {
                        tup.value_space.last().cloned().unwrap().lexical()
                    }
                    _ => {
                        Expr::from(Block::Tuple(tup.lexical()))
                    }
                }
            }
            Block::HashSet(_) => {
                let blk: Block = self.lexical();
                Expr::from(blk)
            }
            Block::HashMap(_) => {
                let blk: Block = self.lexical();
                Expr::from(blk)
            }
        }
    }
}

impl<Val> Lexical<BlockInner<Val>> for BlockInner<Val> 
where Val: Lexical<Val> + std::fmt::Debug {
    fn lexical(self) -> BlockInner<Val> {
        // println!("bin => {:?}", self.binder_space);
        // println!("val => {:?}", self.value_space);
        let Self { binder_space, value_space } = self;
        Self {
            binder_space: binder_space.into_iter().map(|a| a.lexical()).collect(),
            value_space: value_space.into_iter().map(|v| v.lexical()).collect(),
        }
    }
}
