#[derive(Clone)]
pub struct Nana {
    pub body: Expr,
}

#[derive(Clone)]
pub enum Expr {
    Literal(Literal),
    Binder(Binder),
    Abstraction(Abstraction),
    Application(Application),
    Projection(Projection),
    GatedBlock(Closure),
    ControlFlow(ControlFlow),
}

#[derive(Clone)]
pub enum Literal {
    Int(u64),
    Float(f64),
    Str(String),
    Raw(String),
}

#[derive(Clone)]
pub struct Binder(String);
impl Binder {
    pub fn name(self) -> String {
        let Binder(s) = self;
        s
    }
}

// #[derive(Clone)]
// pub enum Head {
//     Fun {
//         binder: Binder,
//         args: Pattern,
//         mask: Mask,
//     },
//     Pat {
//         pattern: Pattern,
//         mask: Mask,
//     },
// }

#[derive(Clone)]
pub enum Mask {
    Closed,
    Exposed,
}

#[derive(Clone)]
pub struct Abstraction {
    pub pattern: Pattern,
    pub mask: Mask,
    pub expr: Box<Expr>,
}

#[derive(Clone)]
pub struct Application {
    func: Box<Expr>,
    arg: Box<Expr>,
}

#[derive(Clone)]
pub enum Block {
    Vector(BlockInner<Expr>),
    Tuple(BlockInner<Expr>),
    HashSet(BlockInner<Expr>),
    HashMap(BlockInner<Pair>),
}

#[derive(Clone)]
pub struct BlockInner<Val> {
    pub binder_space: Vec<Abstraction>,
    pub value_space: Vec<Val>,
}

#[derive(Debug, Clone)]
pub struct Pair {
    pub key: Expr,
    pub val: Expr,
}

#[derive(Clone)]
pub struct Projection {
    block: Box<Expr>,
    binder: Binder,
}

#[derive(Clone)]
pub struct Closure {
    paras: Vec<Pattern>,
    block: Block,
}

#[derive(Clone)]
pub enum Pattern {
    Alias(Box<Pattern>, Box<Pattern>),
    Wild,
    Rest,
    Literal(Literal),
    Binder(Binder),
    Exposure(Vec<ExposurePattern>),
    Vector(Vec<Pattern>),
    Tuple(Vec<Pattern>),
    HashMap(Vec<(Expr, Pattern)>),
}

#[derive(Clone)]
pub enum ExposurePattern {
    Binder(Binder),
    All
}

#[derive(Clone)]
pub enum ControlFlow {
    Matching(Box<Expr>, Vec<(Pattern, Expr)>),
    // Enumeration(Expr, Vec<Expr>)
}

/// Constructing Ast with From trait
mod construct {
    use super::*;

    impl From<Expr> for Nana {
        fn from(body: Expr) -> Self { Self { body } }
    }

    impl From<BlockInner<Expr>> for Nana {
        fn from(bi: BlockInner<Expr>) -> Self { 
            Expr::from(Block::from(bi)).into()
        }
    }

    impl From<Literal> for Expr {
        fn from(lit: Literal) -> Self { Self::Literal(lit) }
    }
    impl From<Binder> for Expr {
        fn from(binder: Binder) -> Self { Self::Binder(binder) }
    }
    impl From<Abstraction> for Expr {
        fn from(b: Abstraction) -> Self { Self::Abstraction(b) }
    }
    impl From<Application> for Expr {
        fn from(app: Application) -> Self { Self::Application(app) }
    }
    impl From<Block> for Expr {
        fn from(block: Block) -> Self { Self::GatedBlock(block.into()) }
    }
    impl From<Projection> for Expr {
        fn from(p: Projection) -> Self { Self::Projection(p) }
    }
    impl From<Closure> for Expr {
        fn from(c: Closure) -> Self { Self::GatedBlock(c) }
    }
    impl From<Pattern> for Expr {
        fn from(pat: Pattern) -> Self {
            match pat {
                Pattern::Binder(b) => Self::Binder(b),
                // Pattern::Exposure(_) => todo!(),
                // Pattern::Vector(v) => {
                //     Self::Block(Block::from())
                // }
                // Pattern::Tuple(_) => todo!(),
                // Pattern::HashMap(_) => todo!(),
                _ => panic!("Converting illegal pattern to expr.")
            }
        }
    }
    impl From<ControlFlow> for Expr {
        fn from(flow: ControlFlow) -> Self { Self::ControlFlow(flow) }
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

    impl From<String> for Binder {
        fn from(s: String) -> Self {
            Self (s)
        }
    }

    // impl From<(Head, Expr)> for Abstraction {
    //     fn from((head, expr): (Head, Expr)) -> Self {
    //         let expr = Box::new(expr);
    //         Self { head, expr }
    //     }
    // }
    // impl From<(Head, Abstraction)> for Abstraction {
    //     fn from((head, binding): (Head, Abstraction)) -> Self {
    //         (head, Expr::from(binding)).into()
    //     }
    // }
    impl From<(Pattern, Mask, Expr)> for Abstraction {
        fn from((pattern, mask, expr): (Pattern, Mask, Expr)) -> Self {
            let expr = Box::new(expr);
            Self { pattern, mask, expr }
        }
    }
    impl From<(Binder, Mask, Expr)> for Abstraction {
        fn from((binder, mask, expr): (Binder, Mask, Expr)) -> Self {
            (Pattern::Binder(binder), mask, expr).into()
        }
    }
    impl From<Pattern> for Abstraction {
        fn from(pattern: Pattern) -> Self {
            let expr = Box::new(Expr::from(pattern.clone()));
            Self {
                pattern,
                mask: Mask::Exposed,
                expr,
            }
        }
    }

    // impl From<(Pattern, Mask)> for Head {
    //     fn from((pattern, mask): (Pattern, Mask)) -> Self {
    //         Self::Pat { pattern, mask }
    //     }
    // }
    // impl From<Pattern> for Head {
    //     fn from(pattern: Pattern) -> Self {
    //         let mask = Mask::Exposed;
    //         Self::Pat { pattern, mask }
    //     }
    // }
    // impl From<(Binder, Pattern, Mask)> for Head {
    //     /// Val form
    //     fn from((binder, args, mask): (Binder, Pattern, Mask)) -> Self {
    //         Self::Fun { binder, args, mask }
    //     }
    // }
    // impl From<(Binder, Vec<Pattern>, Mask)> for Head {
    //     /// Val form
    //     fn from((binder, args, mask): (Binder, Vec<Pattern>, Mask)) -> Self {
    //         let args = Pattern::Vector(args);
    //         Self::Fun { binder, args, mask }
    //     }
    // }

    impl From<(Expr, Expr)> for Application {
        fn from((func, arg): (Expr, Expr)) -> Self {
            let func = Box::new(func);
            let arg = Box::new(arg);
            Self { func, arg }
        }
    }

    impl From<BlockInner<Expr>> for Block {
        fn from(bi: BlockInner<Expr>) -> Self { 
            Self::Tuple(bi)
        }
    }

    impl<Val> From<(Vec<Abstraction>, Vec<Val>)> for BlockInner<Val> {
        fn from(
            (binder_space, value_space): (Vec<Abstraction>, Vec<Val>)
        ) -> Self { 
            Self { binder_space, value_space } 
        }
    }

    impl From<(Expr, Expr)> for Pair {
        fn from((key, val): (Expr, Expr)) -> Self {
            Self { key, val }
        }
    }

    impl From<(Expr, Binder)> for Projection {
        fn from((e, binder): (Expr, Binder)) -> Self {
            let block = Box::new(e);
            Self { block, binder }
        }
    }

    impl From<Block> for Closure {
        fn from(b: Block) -> Self {
            let ps = Vec::new();
            (ps, b).into()
        }
    }
    impl From<(Vec<Pattern>, Block)> for Closure {
        fn from((paras, block): (Vec<Pattern>, Block)) -> Self { Self { paras, block } }
    }

    impl From<(Pattern, Pattern)> for Pattern {
        fn from((alias, pat): (Pattern, Pattern)) -> Self {
            Self::Alias(Box::new(alias), Box::new(pat))
        }
    }

    impl From<(Expr, Vec<(Pattern, Expr)>)> for ControlFlow {
        fn from((e, branches): (Expr, Vec<(Pattern, Expr)>)) -> Self {
            Self::Matching(Box::new(e), branches)
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
                Self::Literal(e) => write!(f, "{:#?}", e),
                Self::Binder(e) => write!(f, "{:#?}", e),
                Self::Abstraction(b) => {
                    write!(f, "{:#?}", b)
                }
                Self::Application(app) => {
                    write!(f, "{:#?}", app)
                }
                Self::Projection(p) => write!(f, "{:#?}", p),
                Self::GatedBlock(c) => write!(f, "{:#?}", c),
                Self::ControlFlow(c) => {
                    write!(f, "{:#?}", c)
                }
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

    impl fmt::Debug for Binder {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl fmt::Debug for Abstraction {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:#?} {:#?} {:#?}", self.pattern, self.mask, self.expr)
        }
    }

    // impl fmt::Debug for Head {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //         match self {
    //             Head::Fun { binder, args, mask } => 
    //                 write!(f, "{:#?} {:#?} {:#?}", binder, args, mask),
    //             Head::Pat { pattern, mask } =>
    //                 write!(f, "{:#?} {:#?}", pattern, mask),
    //         }
    //     }
    // }

    impl fmt::Debug for Mask {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Mask::Closed => write!(f, "="),
                Mask::Exposed => write!(f, ":="),
            }
        }
    }

    impl fmt::Debug for Application {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({:#?} {:#?})", self.func, self.arg)
        }
    }
    
    impl fmt::Debug for Block {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Block::Vector(blk) => {
                    let mut db = f.debug_list();
                    for b in blk.binder_space.iter() {
                        db.entry(&b);
                    }
                    for m in blk.value_space.iter() {
                        db.entry(&m);
                    }
                    db.finish()
                }
                Block::Tuple(blk) => {
                    let mut db = f.debug_tuple("");
                    for b in blk.binder_space.iter() {
                        db.field(&b);
                    }
                    for m in blk.value_space.iter() {
                        db.field(&m);
                    }
                    db.finish()
                }
                Block::HashSet(blk) => {
                    let mut db = f.debug_set();
                    for b in blk.binder_space.iter() {
                        db.entry(&b);
                    }
                    for m in blk.value_space.iter() {
                        db.entry(&m);
                    }
                    db.finish()
                }
                Block::HashMap(blk) => {
                    let mut db = f.debug_map();
                    for b in blk.binder_space.iter() {
                        db.entry(&"", &b);
                    }
                    for Pair { key, val } in blk.value_space.iter() {
                        db.entry(key, val);
                    }
                    db.finish()
                }
            }
        }
    }

    impl fmt::Debug for Projection {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let Projection { block, binder} = self;
            write!(f, "{:#?}.{:#?}", block, binder)
        }
    }

    impl fmt::Debug for Closure {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let Closure { paras: para, block } = self;
            write!(f, "| {:#?} | {:#?}", para, block)
        }
    }

    impl fmt::Debug for Pattern {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Alias(al,p) => write!(f, "({:#?} = {:#?})", al, p),
                Self::Wild => write!(f, "_"),
                Self::Rest => write!(f, ".."),
                Self::Literal(l) => write!(f, "{:#?}", l),
                Self::Binder(b) => write!(f, "{:#?}", b),
                Self::Exposure(ex) => {
                    write!(f, "<")?;
                    write!(f, "{:#?}", DebugVec(
                        ex.iter().cloned().collect(),
                        ";"
                    ))?;
                    write!(f, ">")        
                }
                Self::Vector(ps) => {
                    write!(f, "[")?;
                    write!(f, "{:#?}", DebugVec(ps.clone(), ","))?;
                    write!(f, "]")
                }
                Self::Tuple(ps) => {
                    write!(f, "(")?;
                    write!(f, "{:#?}", DebugVec(ps.clone(), ","))?;
                    write!(f, ")")
                }
                Self::HashMap(ps) => {
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
}