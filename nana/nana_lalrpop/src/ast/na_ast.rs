#[derive(Debug, Clone)]
pub struct Nana {
    pub body: Expr,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Atom(Atom),
    Application(Application),
}

#[derive(Debug, Clone)]
pub enum Atom {
    Expr(Box<Expr>),
    Block(Block),
    Struct(Struct),
    Binder(Binder),
    Literal(Literal),
}

#[derive(Debug, Clone)]
pub struct Application {
    func: Atom,
    args: Vec<Atom>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub binder_space: Vec<Binding>,
    pub value_space: Option<Box<Expr>>,
}

#[derive(Debug, Clone)]
pub enum Struct {
    Sequence(Vec<Expr>),
    Tuple(Vec<Expr>),
    Hashmap(Vec<(Literal, Expr)>),
}

#[derive(Debug, Clone)]
pub struct Literal(String);

#[derive(Debug, Clone)]
pub struct Binding {
    pub heads: Vec<Head>,
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub struct Binder(String);

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Mask {
    Closed,
    Exposed,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Binder(Binder),
    Arbitrary,
    Exposure(ExposurePattern),
    Sequence(Vec<Pattern>),
    Tuple(Vec<Pattern>),
}

#[derive(Debug, Clone)]
pub enum ExposurePattern {
    Binders(Vec<Binder>),
    All
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
    impl From<Application> for Expr {
        fn from(app: Application) -> Self { Self::Application(app) }
    }

    impl From<Expr> for Atom {
        fn from(e: Expr) -> Self {
            Self::Expr(Box::new(e))
        }
    }
    impl From<Block> for Atom {
        fn from(block: Block) -> Self { Self::Block(block) }
    }
    impl From<Struct> for Atom {
        fn from(stct: Struct) -> Self { Self::Struct(stct) }
    }
    impl From<Binder> for Atom {
        fn from(binder: Binder) -> Self { Self::Binder(binder) }
    }
    impl From<Literal> for Atom {
        fn from(lit: Literal) -> Self { Self::Literal(lit) }
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

    impl From<Atom> for Application {
        fn from(func: Atom) -> Self {
            (func, Vec::new()).into()
        }
    }
    impl From<(Atom, Vec<Atom>)> for Application {
        fn from((func, args): (Atom, Vec<Atom>)) -> Self {
            Self { func, args }
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

    impl From<(Pattern, Mask)> for Head {
        fn from((pattern, mask): (Pattern, Mask)) -> Self {
            Self::Pat { pattern, mask }
        }
    }
    impl From<(Binder, Pattern, Mask)> for Head {
        /// Val form
        fn from((binder, args, mask): (Binder, Pattern, Mask)) -> Self {
            Self::Fun { binder, args, mask }
        }
    }
    impl From<(Binder, Vec<Pattern>, Mask)> for Head {
        /// Val form
        fn from((binder, args, mask): (Binder, Vec<Pattern>, Mask)) -> Self {
            let args = Pattern::Sequence(args);
            Self::Fun { binder, args, mask }
        }
    }

    impl From<Vec<Binder>> for ExposurePattern {
        fn from(binders: Vec<Binder>) -> Self { Self::Binders(binders) }
    }
}

mod print {
    // use super::*;
    // use core::fmt;
}
