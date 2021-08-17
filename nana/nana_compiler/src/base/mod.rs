pub trait Inner {
    type Target;
    fn inner(self) -> Self::Target;
    fn inner_ref(&self) -> &Self::Target;
    fn inner_mut(&mut self) -> &mut Self::Target;
}

pub struct Node<T, Attr> (T, Attr);
impl<T, Attr> Inner for Node<T, Attr> {
    type Target = T;
    fn inner(self) -> Self::Target {
        self.0
    }
    fn inner_ref(&self) -> &Self::Target {
        &self.0
    }
    fn inner_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
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

/// Constructing Ast with From trait
mod construct {
    use super::*;

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

    impl<S> From<S> for Binder
    where S: Into<String> {
        fn from(s: S) -> Self {
            Self (s.into())
        }
    }
}

/// Printing Ast.
mod print {
    use super::*;
    use std::fmt;

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
}
