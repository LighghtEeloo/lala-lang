#[derive(Debug)]
pub struct Nana {
    pub body: Expr,
}

#[derive(Debug)]
pub enum Expr {
    Atom(Atom),
    Binding(Box<Binding>),
}

#[derive(Debug)]
pub enum Atom {
    Block(Block),
    Literal(Literal)
}

#[derive(Debug)]
pub struct Block {
    pub exprs: Vec<Expr>,
}

#[derive(Debug)]
pub struct Literal(String);

#[derive(Debug)]
pub struct Binding {
    pub head: Head,
    pub mask: Mask,
    pub expr: Expr,
}

#[derive(Debug)]
pub struct Binder(String);

#[derive(Debug)]
pub enum Head {
    Val {
        binder: Binder,
        args: Pattern,
    },
    Pat {
        pattern: Pattern
    },
}

#[derive(Debug)]
pub enum Mask {
    Closed,
    Exposed,
}

#[derive(Debug)]
pub enum Pattern {
    Binder(Binder),
    Exposure(Vec<Binder>),
    Sequence(Vec<Pattern>),
}

mod construct {
    use super::*;

    impl From<Expr> for Nana {
        fn from(body: Expr) -> Self { Self { body } }
    }

    impl From<Vec<Expr>> for Nana {
        fn from(exprs: Vec<Expr>) -> Self { 
            Expr::from(Atom::from(Block::from(exprs))).into()
        }
    }

    impl From<Atom> for Expr {
        fn from(atom: Atom) -> Self { Self::Atom(atom) }
    }
    impl From<Binding> for Expr {
        fn from(binding: Binding) -> Self {
            Self::Binding(Box::new(binding))
        }
    }

    impl From<Literal> for Atom {
        fn from(lit: Literal) -> Self { Self::Literal(lit) }
    }
    impl From<Block> for Atom {
        fn from(block: Block) -> Self { Self::Block(block) }
    }

    impl From<Vec<Expr>> for Block {
        fn from(exprs: Vec<Expr>) -> Self { Self { exprs } }
    }

    impl From<String> for Literal {
        fn from(s: String) -> Self {
            Self (s)
        }
    }

    impl From<(Head, Mask, Expr)> for Binding {
        fn from((head, mask, expr): (Head, Mask, Expr)) -> Self {
            Self { head, mask, expr }
        }
    }

    impl From<String> for Binder {
        fn from(s: String) -> Self {
            Self (s)
        }
    }

    impl From<(Binder, Pattern)> for Head {
        /// Val form
        fn from((binder, args): (Binder, Pattern)) -> Self {
            Self::Val { binder, args }
        }
    }
    impl From<Pattern> for Head {
        fn from(pattern: Pattern) -> Self {
            Self::Pat { pattern }
        }
    }
}

mod print {
    // use super::*;
    // use core::fmt;
}
