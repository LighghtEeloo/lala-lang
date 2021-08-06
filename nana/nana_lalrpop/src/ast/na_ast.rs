#[derive(Debug)]
pub struct Nana {
    pub body: Expr,
}

#[derive(Debug)]
pub enum Expr {
    Atom(Atom),
}

#[derive(Debug)]
pub enum Atom {
    Block(Block),
    Literal(Literal)
}

#[derive(Debug)]
pub struct Block {
    pub binder_space: Vec<Binding>,
    pub value_space: Option<Box<Expr>>,
}

#[derive(Debug)]
pub struct Literal(String);

#[derive(Debug)]
pub struct Binding {
    pub heads: Vec<Head>,
    pub expr: Expr,
}

#[derive(Debug)]
pub struct Binder(String);

#[derive(Debug)]
pub enum Head {
    Fun {
        binder: Binder,
        args: Pattern,
        mask: Mask,
    },
    Pat {
        pattern: Pattern,
        mask: Mask,
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

    impl From<Block> for Nana {
        fn from(block: Block) -> Self { 
            Expr::from(Atom::from(block)).into()
        }
    }

    impl From<Atom> for Expr {
        fn from(atom: Atom) -> Self { Self::Atom(atom) }
    }

    impl From<Literal> for Atom {
        fn from(lit: Literal) -> Self { Self::Literal(lit) }
    }
    impl From<Block> for Atom {
        fn from(block: Block) -> Self { Self::Block(block) }
    }

    impl From<(Vec<Binding>, Expr)> for Block {
        fn from((binding_space, value_space): (Vec<Binding>, Expr)) -> Self { 
            let value_space = Some(Box::new(value_space));
            Self { binder_space: binding_space, value_space } 
        }
    }
    impl From<Vec<Binding>> for Block {
        fn from(binding_space: Vec<Binding>) -> Self { 
            let value_space = None;
            Self { binder_space: binding_space, value_space } 
        }
    }

    impl From<String> for Literal {
        fn from(s: String) -> Self {
            Self (s)
        }
    }

    impl From<(Vec<Head>, Expr)> for Binding {
        fn from((heads, expr): (Vec<Head>, Expr)) -> Self {
            Self { heads, expr }
        }
    }

    impl From<String> for Binder {
        fn from(s: String) -> Self {
            Self (s)
        }
    }

    impl From<(Binder, Pattern, Mask)> for Head {
        /// Val form
        fn from((binder, args, mask): (Binder, Pattern, Mask)) -> Self {
            Self::Fun { binder, args, mask }
        }
    }
    impl From<(Pattern, Mask)> for Head {
        fn from((pattern, mask): (Pattern, Mask)) -> Self {
            Self::Pat { pattern, mask }
        }
    }
}

mod print {
    // use super::*;
    // use core::fmt;
}
