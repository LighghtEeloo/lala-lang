pub enum Expr {
    // TypeConstructor(),
    Abstraction(Binding),
    Exposure(Atom),
    Atom(Atom),
}

pub enum Atom {
    Literal(Literal),
    // DataConstructor(Binder, Block),
    Obstruction(Block),
    Projection(Box<Atom>, Binder),
    Application(Binder, Block),
}

pub enum Pattern {
    Binder(Binder),
    Rest(),
    List(Vec<Pattern>),
    Append(Vec<Pattern>),
    Tuple(Vec<Pattern>),
    HashMap(Vec<Pattern>),
    Exposure(Vec<Binder>),
}

pub enum Literal {
    Str(String),
}

pub enum Binder {
    Identity(String),
    Anonymous(String),
    Arbitrary
}

pub struct Binding {
    pub binder: Binder,
    pub args: Option<Pattern>,
    pub mask: Mask,
    pub atom: Atom,
}

impl From<(Binder, Mask, Atom)> for Binding {
    fn from((binder, mask, atom): (Binder, Mask, Atom)) -> Self {
        Self { binder, args: None, mask, atom }
    }
}

impl From<(Binder, Pattern, Mask, Atom)> for Binding {
    fn from((binder, args, mask, atom): (Binder, Pattern, Mask, Atom)) -> Self {
        Self { binder, args: Some(args), mask, atom }
    }
}

pub enum Mask {
    Closed,
    Exposed {
        pattern: Option<Pattern>
    },
    Open
}

impl From<Option<Pattern>> for Mask {
    fn from(pattern: Option<Pattern>) -> Self {
        Mask::Exposed { pattern }
    }
}

pub struct Sequential {
    bindings: Vec<Binding>
}

impl From<Vec<Binding>> for Sequential {
    fn from(bindings: Vec<Binding>) -> Self {
        Self { bindings }
    }
}

pub struct Simultaneous {
    bindings: Vec<Binding>
}

impl From<Vec<Binding>> for Simultaneous {
    fn from(bindings: Vec<Binding>) -> Self {
        Self { bindings }
    }
}

pub struct Parallel {
    bindings: Vec<Binding>
}

impl From<Vec<Binding>> for Parallel {
    fn from(bindings: Vec<Binding>) -> Self {
        Self { bindings }
    }
}

pub enum Block {
    Sequential(Sequential),
    Simultaneous(Simultaneous),
    Parallel(Parallel),
}

impl From<Sequential> for Block {
    fn from(seq: Sequential) -> Self {
        Block::Sequential(seq)
    }
}

impl From<Parallel> for Block {
    fn from(par: Parallel) -> Self {
        Block::Parallel(par)
    }
}

pub struct ElAst {
    pub block: Block,
}

impl From<Block> for ElAst {
    fn from(block: Block) -> Self {
        Self { block }
    }
}

impl From<Sequential> for ElAst {
    fn from(seq: Sequential) -> Self {
        Block::from(seq).into()
    }
}

impl From<Parallel> for ElAst {
    fn from(par: Parallel) -> Self {
        Block::from(par).into()
    }
}

mod print {
    use core::fmt;
    use super::*;

    fn flatten_iter<I, T>(iter: I, sep: &str) -> String
    where I: Iterator<Item = T>, T: Into<String> {
        let vec: Vec<String> = iter.map(|x| x.into()).collect();
        vec.join(sep)
    }

    impl fmt::Debug for Atom {
        fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
            todo!()
        }
    }

    impl fmt::Debug for Pattern {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let s = match self {
                Pattern::Binder(binder) => format!("{:?}", binder),
                Pattern::Rest() => format!(".."),
                Pattern::List(_) => todo!(),
                Pattern::Append(_) => todo!(),
                Pattern::Tuple(_) => todo!(),
                Pattern::HashMap(_) => todo!(),
                Pattern::Exposure(binders) => {
                    format!("<{}>", flatten_iter(binders.into_iter(), ";"))
                }
            };
            write!(f, "{}", s)
        }
    }

    impl Into<String> for &Binder {
        fn into(self) -> String {
            match self {
                Binder::Identity(id) => format!("{}", id),
                Binder::Anonymous(an) => format!("_{}", an),
                Binder::Arbitrary => format!("_"),
            }
        }
    }

    impl fmt::Debug for Binder {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Binder::Identity(id) => write!(f, "{}", id),
                Binder::Anonymous(an) => write!(f, "_{}", an),
                Binder::Arbitrary => write!(f, "_"),
            }
        }
    }

    impl fmt::Debug for Binding {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let Self { binder, args, mask, atom } = self;
            let args = match args {
                Some(pat) => format!(" {:?}", pat),
                None => format!(""),
            };
            write!(f, "{:?}{} {:?} {:#?}", binder, args, mask, atom)
        }
    }

    impl fmt::Debug for Mask {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let s = match self {
                Mask::Closed  => "=".to_owned(),
                Mask::Exposed { pattern: None } => ":=".to_owned(),
                Mask::Exposed { pattern: Some(pat) } => format!(":{:?}=", pat),
                Mask::Open    => ":*=".to_owned(),
            };
            write!(f, "{}", s)
        }
    }

    impl fmt::Debug for Sequential {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.bindings.iter()).finish()
        }
    }

    impl fmt::Debug for Simultaneous {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.bindings.iter()).finish()
        }
    }

    impl fmt::Debug for Parallel {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_set().entries(self.bindings.iter()).finish()
        }
    }

    impl fmt::Debug for Block {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Block::Sequential(seq) => write!(f, "{:#?}", seq),
                Block::Simultaneous(sim) => write!(f, "{:#?}", sim),
                Block::Parallel(par) => write!(f, "{:#?}", par),
            }
        }
    }

    impl fmt::Debug for ElAst {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:#?}", self.block)
        }
    }
}
