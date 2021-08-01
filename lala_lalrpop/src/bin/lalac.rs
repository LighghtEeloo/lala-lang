use lala_lalrpop::lala;

fn main() -> anyhow::Result<()> {

    // get the file[s] and combine them in parallel, and then...

    let code = r#"
{
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
        u : * = {
            v = [];
            w = [];
        };
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
}
    "#;
    let binder = lala::ParallelParser::new().parse(code)?;
    println!("{:#?}", binder);
    Ok(())
}
