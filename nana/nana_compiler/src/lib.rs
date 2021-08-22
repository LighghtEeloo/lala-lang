mod base;
mod external;
mod flatten;
// mod resolve;
use external::ast as nana_ast;

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub nana);

// pub use external::Lexical;

pub use flatten::Flatten;
