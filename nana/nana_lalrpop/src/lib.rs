pub mod ast;
pub use ast::*;

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub nana);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}