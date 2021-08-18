mod base;
mod external;
mod resolve;
use external::nana_ast;

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub nana);

pub use external::Lexical;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
