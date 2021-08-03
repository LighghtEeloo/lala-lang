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
    pub args: Vec<Binder>,
    pub mask: Mask,
    pub block: Block
}

impl From<(Binder, Mask, Block)> for Binding {
    fn from((binder, mask, block): (Binder, Mask, Block)) -> Self {
        Self { binder, args: Vec::new(), mask, block }
    }
}

impl From<(Binder, Vec<Binder>, Mask, Block)> for Binding {
    fn from((binder, args, mask, block): (Binder, Vec<Binder>, Mask, Block)) -> Self {
        Self { binder, args, mask, block }
    }
}

pub enum Mask {
    Closed,
    Exposed,
    Exposing {
        binders: Vec<Binder>
    },
    Open
}

impl From<Vec<Binder>> for Mask {
    fn from(binders: Vec<Binder>) -> Self {
        Mask::Exposing { binders }
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
            let Self { binder, args, mask, block } = self;
            write!(f, "{:?} {:?} {:?} {:#?}", binder, args, mask, block)
        }
    }

    impl fmt::Debug for Mask {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let s = match self {
                Mask::Closed  => "=".to_owned(),
                Mask::Exposed => ":=".to_owned(),
                Mask::Exposing { binders } => format!(":{:?}=", binders),
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
