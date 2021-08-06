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
    Block(Block),
    Literal(Literal),
}

#[derive(Debug, Clone)]
pub struct Block {
    pub binder_space: Vec<Binding>,
    pub value_space: Option<Box<Expr>>,
}

#[derive(Debug, Clone)]
pub struct Application {
    binder: Binder,
    arg: Option<Box<Atom>>,
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

    impl From<Block> for Atom {
        fn from(block: Block) -> Self { Self::Block(block) }
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

    impl From<Binder> for Application {
        fn from(binder: Binder) -> Self {
            let arg = None;
            Self { binder, arg }
        }
    }
    impl From<(Binder, Atom)> for Application {
        fn from((binder, arg): (Binder, Atom)) -> Self {
            let arg = Some(Box::new(arg));
            Self { binder, arg }
        }
    }
    impl From<(Binder, Option<Atom>)> for Application {
        fn from((binder, arg): (Binder, Option<Atom>)) -> Self {
            match arg {
                Some(arg) => (binder, arg).into(),
                None => binder.into(),
            }
        }
    }
    impl From<(Binder, Vec<Atom>)> for Application {
        fn from((binder, args): (Binder, Vec<Atom>)) -> Self {
            let arg = args.get(0).cloned(); // Todo: add sequence.
            (binder, arg).into()
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

    impl From<Vec<Binder>> for ExposurePattern {
        fn from(binders: Vec<Binder>) -> Self { Self::Binders(binders) }
    }
}

mod print {
    // use super::*;
    // use core::fmt;
}
