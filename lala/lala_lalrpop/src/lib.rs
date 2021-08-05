pub mod ast; 
pub use ast::*;

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub lala);

#[cfg(test)]
mod tests {
    // use super::*;
    // #[test]
    // fn basic_block() {
    //     assert!(lala::BindingParser::new().parse("a=[]").is_ok());
    //     assert!(lala::BindingParser::new().parse("a:=[]").is_ok());
    //     assert!(lala::BindingParser::new().parse("a:=[]").is_ok());
    //     assert!(lala::BindingParser::new().parse("a:[]=[]").is_ok());
    //     assert!(lala::BindingParser::new().parse("a:[*]=[]").is_ok());
    //     assert!(lala::BindingParser::new().parse("a:[hi]=[hi=[]]").is_ok());
        
    //     assert!(lala::BlockParser::new().parse("[]").is_ok());
    //     assert!(lala::BlockParser::new().parse("[[[]]]").is_ok());
    //     assert!(lala::BlockParser::new().parse("[[ []]]").is_ok());
    //     assert!(lala::BlockParser::new().parse("[[ []]   ]").is_ok());
    //     assert!(lala::BlockParser::new().parse("[a=[]]").is_ok());
    //     assert!(lala::BlockParser::new().parse("[a = []]").is_ok());
    //     assert!(lala::BlockParser::new().parse("[[a=[];b=[]]]").is_ok());
    //     assert!(lala::BlockParser::new().parse("[[a = [] ;b =[]]]").is_ok());

    //     assert!(lala::BindingParser::new().parse("a:=[b=[];c:=[d:=[]]]").is_ok());
    //     assert!(lala::BindingParser::new().parse("a:[*]=[b=[];c:[d;e]=[d:=[];e:=[]]]").is_ok());
    // }
}