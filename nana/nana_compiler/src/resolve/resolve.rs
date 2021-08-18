use crate::nana_ast::*;

#[derive(Clone)]
pub struct Ctx {
    inheritted: Option<Box<Ctx>>,
    introduced: Pattern,
    evaluated: Vec<(Pattern, Expr)>,
}

#[derive(Clone)]
pub struct Contexted<T> (Ctx, T);

impl<T> Inner for Contexted<T> {
    type Target = T;
    fn inner(self) -> Self::Target {
        self.1
    }
    fn inner_ref(&self) -> &Self::Target {
        &self.1
    }
    fn inner_mut(&mut self) -> &mut Self::Target {
        &mut self.1
    }
}

// use std::ops::{Deref, DerefMut};
// impl<T> Deref for Contexted<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.1
//     }
// }
// impl<T> DerefMut for Contexted<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.1
//     }
// }


