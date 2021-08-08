use std::io::Read;

use nana_lalrpop::nana;

fn main() -> anyhow::Result<()> {

    if std::env::args().len() > 1 {
        fast_trial();
        return Ok(())
    }

    // get the file[s] and combine them in parallel, and then...

    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;

    let res = nana::NanaParser::new().parse(&buf);
    println!("{}", "=".repeat(80));
    match res {
        Ok(res) => {
            println!("{:#?}", res);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
    println!("{}", "=".repeat(80));

    Ok(())
}

fn fast_trial() {
    let file_seq = format!("{}", r#"
~ pi := ~ <dine> = [
    ~ murmur := "...\"";
    ~ id x := x;
    ~ double x := (x,x);
    ~ idd x' := [
        ~ (x, y) := x';
        x
    ];
    ~ dine := ~ <*> := [
        ~ divine := [|jail|];
        ~ divine := [1, 0x2f, 3.0, .4, 5e1];
        3.1415926
    ];
    id dine
];
pi
    "#);

    let res = nana::NanaParser::new().parse(&file_seq);
    match res {
        Ok(res) => {
            println!("{:#?}", res);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
