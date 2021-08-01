use lala_lalrpop::lala;

fn main() -> anyhow::Result<()> {
    // get the file[s] and combine them in parallel, and then...

    let code = "a:[*]=[b=[];c:[d;e]=[d:=[];e:=[]]]";
    let binder = lala::BindingParser::new().parse(code)?;
    println!("{:#?}", binder);
    Ok(())
}
