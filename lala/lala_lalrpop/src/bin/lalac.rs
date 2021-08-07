use std::io::Read;

use lala_lalrpop::lala;

fn main() -> anyhow::Result<()> {

    if std::env::args().len() > 1 {
        fast_trial();
        return Ok(())
    }

    // get the file[s] and combine them in parallel, and then...

    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;

    let res = lala::LaLaParser::new().parse(&buf).unwrap();
    println!("{}", "=".repeat(80));
    println!("{:#?}", res);
    println!("{}", "=".repeat(80));

    Ok(())
}

fn fast_trial() {
    let file_seq = format!("{}", r#"
a :[*]= [
    b = [];
    c :[d; e]= [
        d := [];
        e := [];
    ];
    x : y; z = [
        y := [];
        z := [];
    ];
    x' : * = {
        y' = [];
        z' = [];
    };
    f x y := [];
    g x y : a; b = [
        a := [];
        b := [];
    ];
    par := {
        d := [];
        e := [];
    };
    under_score := [];
    _anony := [];
    __anony := [];
    _A_nony := [];
    _ := [];
]
    "#);

    let res = lala::BindingParser::new().parse(&file_seq).unwrap();
    println!("{:#?}", res);
    
    let fa = r#"
a := []
    "#;
    let fb = r#"
b :*= []
    "#;

    let package_par = format!("{{{};{}}}", fa, fb);
    
    let res = lala::ParallelParser::new().parse(&package_par).unwrap();
    println!("{:#?}", res);
}
