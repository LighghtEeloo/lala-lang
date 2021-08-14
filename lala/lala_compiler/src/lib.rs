mod external;
mod resolve;
use external::lala_ast;


#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub lala);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
