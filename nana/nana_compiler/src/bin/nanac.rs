use std::io::Read;

use nana_compiler::{
    nana,
    // Lexical
};

fn main() -> anyhow::Result<()> {

    if std::env::args().len() > 1 {
        for (i, code) in code_base().iter().enumerate() {
            println!(">>>>>> Parsing: Case {} >>>>>>", i);
            parse_nana(code);
        }
        return Ok(())
    }

    // get the file[s] and combine them in simultaneity, and then...

    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;

    parse_nana(&buf);
    
    Ok(())
}


fn parse_nana(code: &String) {
    let res = nana::NanaParser::new().parse(code);
    match res {
        Ok(res) => {
            println!("{:#?}", res);
            // let nana = res.lexical();
            // println!("{:#?}", nana);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    println!("{}", "=".repeat(80));
}


fn code_base() -> Vec<String> {
    vec![ 
//     format!("{}", r#"
// pi := (
//     ~ (<pi; double; id; idd> = pi_m) = [
//         murmur := "...\"";
//         id := |x| (x);
//         ~ tuple x y := (x,y);
//         double := |x| (tuple x x);
//         ~ idd x' := [
//             ~ x' ;
//             ~ (x, y) := x';
//             x
//         ];
//         ~ dine := ~ <..> := (
//             divine := [|jail|];
//             divine := [1, 0x2f, 3.0, .4, 5e1];
//             3.1415926
//         );
//         pi := id dine;
//     ];
//     pi_m.pi
// );
//     "#),
    format!("{}", r#"
()
    "#),
    format!("{}", r#"
lam := || (
    || ()
);
    "#),
    format!("{}", r#"
curry := |a, b| (
    |c, d| ((((1))))
);
    "#),
    format!("{}", r#"
curry' := |a, b| (
    ((|| ( |c| (((1))))))
);
    "#),
    ]
} 
