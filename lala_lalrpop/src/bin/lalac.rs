use lala_lalrpop::lala;

fn main() -> anyhow::Result<()> {

    // get the file[s] and combine them in parallel, and then...

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

    Ok(())
}
