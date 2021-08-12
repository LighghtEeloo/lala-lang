#[derive(Clone)]
pub struct Nana {
    pub body: Expr,
}

#[derive(Clone)]
pub enum Expr {
    Application(Application),
    ControlFlow(ControlFlow),
    Block(Block),
    Binder(Binder),
    Literal(Literal),
}

#[derive(Clone)]
pub struct Application {
    func: Box<Expr>,
    args: Vec<Expr>,
}

#[derive(Clone)]
pub enum ControlFlow {
    Matching(Box<Expr>, Vec<(Pattern, Expr)>),
    // Enumeration(Expr, Vec<Expr>)
}


#[derive(Clone)]
pub struct Block {
    pub structure: Sturcture,
    pub binder_space: Vec<Binding>,
    pub value_space: Vec<Molecule>,
}

#[derive(Clone)]
pub enum Sturcture {
    Sum,
    Product,
    Labeled
}

#[derive(Clone)]
pub enum Molecule {
    Expr(Expr),
    Paired(Paired),
}

#[derive(Debug, Clone)]
pub struct Paired {
    pub key: Literal,
    pub val: Expr,
}

#[derive(Clone)]
pub enum Literal {
    Int(u64),
    Float(f64),
    Str(String),
    Raw(String),
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
    Wild,
    Rest,
    Exposure(Vec<ExposurePattern>),
    Vector(Vec<Pattern>),
    Tuple(Vec<Pattern>),
    HashMap(Vec<Expr>),
}

#[derive(Clone)]
pub enum ExposurePattern {
    Binder(Binder),
    All
}

/// Constructing Ast with From trait
mod construct {
    use super::*;

    impl From<Expr> for Nana {
        fn from(body: Expr) -> Self { Self { body } }
    }

    impl From<(Vec<Binding>, Vec<Molecule>)> for Nana {
        fn from(bi: (Vec<Binding>, Vec<Molecule>)) -> Self { 
            Expr::from(Block::from((Sturcture::Product, bi))).into()
        }
    }

    impl From<Application> for Expr {
        fn from(app: Application) -> Self { Self::Application(app) }
    }
    impl From<ControlFlow> for Expr {
        fn from(flow: ControlFlow) -> Self { Self::ControlFlow(flow) }
    }
    impl From<Block> for Expr {
        fn from(block: Block) -> Self { Self::Block(block) }
    }
    impl From<Binder> for Expr {
        fn from(binder: Binder) -> Self { Self::Binder(binder) }
    }
    impl From<Literal> for Expr {
        fn from(lit: Literal) -> Self { Self::Literal(lit) }
    }

    impl From<Expr> for Application {
        fn from(func: Expr) -> Self {
            (func, Vec::new()).into()
        }
    }
    impl From<(Expr, Vec<Expr>)> for Application {
        fn from((func, args): (Expr, Vec<Expr>)) -> Self {
            let func = Box::new(func);
            Self { func, args }
        }
    }

    impl From<(Expr, Vec<(Pattern, Expr)>)> for ControlFlow {
        fn from((e, branches): (Expr, Vec<(Pattern, Expr)>)) -> Self {
            Self::Matching(Box::new(e), branches)
        }
    }

    impl From<(Sturcture, (Vec<Binding>, Vec<Molecule>))> for Block {
        fn from(
            (computation, (binder_space, value_space)): 
            (Sturcture, (Vec<Binding>, Vec<Molecule>))
        ) -> Self { 
            Self { structure: computation, binder_space, value_space } 
        }
    }

    impl From<Expr> for Molecule {
        fn from(e: Expr) -> Self { Self::Expr(e) }
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
}


/// Printing Ast.
mod print {
    use super::*;
    use std::fmt;

    struct DebugVec<T, Sep> (Vec<T>, Sep);
    impl<T, Sep> fmt::Debug for DebugVec<T, Sep> 
    where T: fmt::Debug, Sep: fmt::Display {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let DebugVec(ps, s) = self;
            let mut it = ps.iter();
            if let Some(p) = it.next() {
                write!(f, "{:#?}", p)?;
            }
            for p in it {
                write!(f, "{} {:#?}", s, p)?;
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
                Self::ControlFlow(c) => {
                    write!(f, "{:#?}", c)
                }
                Self::Application(app) => {
                    write!(f, "{:#?}", app)
                }
                Self::Block(e) => write!(f, "{:#?}", e),
                Self::Binder(e) => write!(f, "{:#?}", e),
                Self::Literal(e) => write!(f, "{:#?}", e),
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

    impl fmt::Debug for ControlFlow {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ControlFlow::Matching(e, bs) => {
                    write!(f, "? {:#?} ", e)?;
                    for (p, e) in bs {
                        write!(f, "| {:#?} -> {:#?} ", p, e)?;
                    }
                }
            }
            write!(f, "")
        }
    }

    impl fmt::Debug for Block {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.structure {
                Sturcture::Sum => {
                    let mut db = f.debug_list();
                    for b in self.binder_space.iter() {
                        db.entry(&b);
                    }
                    for m in self.value_space.iter() {
                        db.entry(&m);
                    }
                    db.finish()
                }
                Sturcture::Product => {
                    let mut db = f.debug_tuple("");
                    for b in self.binder_space.iter() {
                        db.field(&b);
                    }
                    for m in self.value_space.iter() {
                        db.field(&m);
                    }
                    db.finish()
                }
                Sturcture::Labeled => {
                    let mut db = f.debug_map();
                    for b in self.binder_space.iter() {
                        db.entry(&"", &b);
                    }
                    for m in self.value_space.iter() {
                        match m {
                            Molecule::Expr(e) => {
                                db.entry(&"", &e);
                            }
                            Molecule::Paired(Paired { key, val }) => {
                                db.entry(key, val);
                            }
                        }
                    }
                    db.finish()
                }
            }
        }
    }

    impl fmt::Debug for Molecule {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Molecule::Expr(e) => write!(f, "{:#?}", e),
                Molecule::Paired(_) => todo!(),
            }
            
        }
    }

    impl fmt::Debug for Literal {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Literal::Int(e) => write!(f, "Int({})", e),
                Literal::Float(e) => write!(f, "Flt({})", e),
                Literal::Str(e) => write!(f, "Str({})", e),
                Literal::Raw(e) => write!(f, "Raw({})", e),
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
                Pattern::Wild => write!(f, "_"),
                Pattern::Rest => write!(f, ".."),
                Pattern::Exposure(ex) => {
                    write!(f, "<")?;
                    write!(f, "{:#?}", DebugVec(
                        ex.iter().cloned().collect(),
                        ";"
                    ))?;
                    write!(f, ">")        
                }
                Pattern::Vector(ps) => {
                    write!(f, "[")?;
                    write!(f, "{:#?}", DebugVec(ps.clone(), ","))?;
                    write!(f, "]")
                }
                Pattern::Tuple(ps) => {
                    write!(f, "(")?;
                    write!(f, "{:#?}", DebugVec(ps.clone(), ","))?;
                    write!(f, ")")
                }
                Pattern::HashMap(ps) => {
                    write!(f, "{{")?;
                    write!(f, "{:#?}", DebugVec(ps.clone(), ","))?;
                    write!(f, "}}")
                }
            }
        }
    }

    impl fmt::Debug for ExposurePattern {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ExposurePattern::Binder(b) => write!(f, "{:#?}", b),
                ExposurePattern::All => write!(f, ".."),
            }
        }
    }
}