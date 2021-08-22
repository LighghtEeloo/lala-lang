mod base;
mod external;
// mod resolve;
use external::ast as nana_ast;

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub nana);

// pub use external::Lexical;
