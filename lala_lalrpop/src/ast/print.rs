use core::fmt;
use super::*;

impl fmt::Debug for Binder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let binder = match self {
            Binder::Identity(id) => id,
            Binder::Anonymous(an) => an,
            Binder::Arbitrary => "_",
        };
        write!(f, "{}", binder)
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

impl fmt::Debug for Parallel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.bindings.iter()).finish()
    }
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Block::Sequential(seq) => write!(f, "{:#?}", seq),
            Block::Parallel(par) => write!(f, "{:#?}", par),
        }
    }
}
