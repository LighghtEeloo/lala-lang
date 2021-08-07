#[derive(Clone)]
pub struct Nana {
    pub body: Expr,
}

#[derive(Clone)]
pub enum Expr {
    Atom(Atom),
    Application(Application),
}

/// A semantically minimal expr, 
/// meaning that Atom must be associated within itself.
///
/// Note that `()` wrapped expr is now in Struct(Tuple).
#[derive(Clone)]
pub enum Atom {
    Block(Block),
    Struct(Struct),
    Binder(Binder),
    Literal(Literal),
}

#[derive(Clone)]
pub struct Application {
    func: Atom,
    args: Vec<Atom>,
}

#[derive(Clone)]
pub struct Block {
    pub binder_space: Vec<Binding>,
    pub value_space: Option<Box<Expr>>,
}

#[derive(Clone)]
pub enum Struct {
    Vector(Vec<Expr>),
    Tuple(Vec<Expr>),
    Hashmap(Vec<(Literal, Expr)>),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(u64),
    Float(f64),
    Str(String),
}

#[derive(Clone)]
pub struct Binding {
    pub heads: Vec<Head>,
    pub expr: Expr,
}

#[derive(Clone)]
pub struct Binder(String);

#[derive(Clone)]
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

#[derive(Clone)]
pub enum Mask {
    Closed,
    Exposed,
}

#[derive(Clone)]
pub enum Pattern {
    Binder(Binder),
    Arbitrary,
    Exposure(ExposurePattern),
    Vector(Vec<Pattern>),
    Tuple(Vec<Pattern>),
}

#[derive(Clone)]
pub enum ExposurePattern {
    Binders(Vec<Binder>),
    All
}

/// Constructing Ast with From trait
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

    impl From<u64> for Literal {
        fn from(i: u64) -> Self {
            Self::Int (i)
        }
    }
    impl From<f64> for Literal {
        fn from(f: f64) -> Self {
            Self::Float (f)
        }
    }
    impl From<String> for Literal {
        fn from(s: String) -> Self {
            Self::Str (s)
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
            let args = Pattern::Vector(args);
            Self::Fun { binder, args, mask }
        }
    }

    impl From<Binder> for Pattern {
        fn from(b: Binder) -> Self { Self::Binder(b) }
    }

    impl From<Vec<Binder>> for ExposurePattern {
        fn from(binders: Vec<Binder>) -> Self { Self::Binders(binders) }
    }
}


/// Printing Ast.
mod print {
    use super::*;
    use std::fmt;

    struct DebugVec<T> (Vec<T>);
    impl<T> fmt::Debug for DebugVec<T> 
    where T: fmt::Debug {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let DebugVec(ps) = self;
            let mut it = ps.iter();
            if let Some(p) = it.next() {
                write!(f, "{:#?}", p)?;
            }
            for p in it {
                write!(f, ", {:#?}", p)?;
            }
            write!(f, "")

        }
    }

    impl fmt::Debug for Nana {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Nana ")?;
            write!(f, "{:#?}", self.body)
        }
    }

    impl fmt::Debug for Expr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Expr::Atom(a) => {
                    write!(f, "{:#?}", a)
                }
                Expr::Application(app) => {
                    write!(f, "{:#?}", app)
                }
            }
        }
    }

    impl fmt::Debug for Atom {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Atom::Block(e) => write!(f, "{:#?}", e),
                Atom::Struct(e) => write!(f, "{:#?}", e),
                Atom::Binder(e) => write!(f, "{:#?}", e),
                Atom::Literal(e) => write!(f, "{:#?}", e),
            }
        }
    }

    impl fmt::Debug for Application {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({:#?}", self.func)?;
            for a in &self.args {
                write!(f, " {:#?}", a)?;
            }
            write!(f, ")")
        }
    }

    impl fmt::Debug for Block {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut db = f.debug_list();
            db.entries(self.binder_space.iter());
            match &self.value_space {
                Some(e) => db.entry(e).finish(),
                None => db.finish()
            }
        }
    }

    impl fmt::Debug for Struct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Struct::Vector(v) => {
                    write!(f, "[")?;
                    write!(f, "{:#?}", DebugVec(v.clone()))?;
                    write!(f, "]")
                }
                Struct::Tuple(v) => {
                    write!(f, "(")?;
                    write!(f, "{:#?}", DebugVec(v.clone()))?;
                    write!(f, ")")
                },
                Struct::Hashmap(v) => {
                    write!(f, "{{")?;
                    write!(f, "{:#?}", DebugVec(v.clone()))?;
                    write!(f, "}}")

                }
            }
        }
    }

    impl fmt::Debug for Binding {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for h in &self.heads {
                write!(f, "{:#?} ", h)?;
            }
            write!(f, "{:#?}", self.expr)
        }
    }

    impl fmt::Debug for Binder {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl fmt::Debug for Head {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Head::Fun { binder, args, mask } => 
                    write!(f, "{:#?} {:#?} {:#?}", binder, args, mask),
                Head::Pat { pattern, mask } =>
                    write!(f, "{:#?} {:#?}", pattern, mask),
            }
        }
    }

    impl fmt::Debug for Mask {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Mask::Closed => write!(f, "="),
                Mask::Exposed => write!(f, ":="),
            }
        }
    }

    impl fmt::Debug for Pattern {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Pattern::Binder(b) => write!(f, "{:#?}", b),
                Pattern::Arbitrary => write!(f, "_"),
                Pattern::Exposure(ex) => write!(f, "{:#?}", ex),
                Pattern::Vector(ps) => {
                    write!(f, "[")?;
                    write!(f, "{:#?}", DebugVec(ps.clone()))?;
                    write!(f, "]")
                }
                Pattern::Tuple(ps) => {
                    write!(f, "(")?;
                    write!(f, "{:#?}", DebugVec(ps.clone()))?;
                    write!(f, ")")
                }
            }
        }
    }


    impl fmt::Debug for ExposurePattern {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ExposurePattern::Binders(bs) => {
                    write!(f, "<")?;
                    write!(f, "{:#?}", DebugVec(
                        bs.iter().cloned().map(Pattern::from).collect()
                    ))?;
                    write!(f, ">")
                }
                ExposurePattern::All => write!(f, "<*>"),
            }
        }
    }
}