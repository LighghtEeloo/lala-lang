use std::io::Read;

use nana_compiler::nana;

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
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    println!("{}", "=".repeat(80));
}


fn code_base() -> Vec<String> {
    vec![ format!("{}", r#"
~ pi := (
    ~ (<pi; double; id; idd> = pi_m) = [
        ~ murmur := "...\"";
        ~ id x := x;
        ~ tuple x y := (x,y);
        ~ double x := tuple x x;
        ~ idd x' := [
            ~ x' ;
            ~ (x, y) := x';
            x
        ];
        ~ dine := ~ <..> := (
            ~ divine := [|jail|];
            ~ divine := [1, 0x2f, 3.0, .4, 5e1];
            3.1415926
        );
        ~ pi := id dine;
    ];
    pi_m.pi
);
    "#),
    format!("{}", r#"
~ pattern xs := (
    ? xs
    | [] -> []
    | [x, y] -> (
        ~ _ = (
            ? { "x": 1 }
            | { "x": x } -> x
        );
        ? x
        | 0 -> [y, x]
        | _ -> [y, y, x]
    )
    | [_, _, (xs=..)] -> xs
    | _ -> xs
);
    "#),
    format!("{}", r#"
~ partition f xs := (
    ~ _part xs a b = (
        ? xs
        | [] -> (a, b)
        | [x] + xs -> (
            ~ (a, b) = _part xs a b;
            ? (f x) 
            | 1 -> (a + [x], b) 
            | 0 -> (a, b + [x])
        )
    );
    _part xs [] []
);
    "#),
    format!("{}", r#"
~ qsort xs := (
    ? xs
    | [] -> []
    | [x] + xs -> (
        ~ (s, l) = partition ((>) x) xs;
        (qsort s) + [x] + (qsort l)
    )
);
qsort [1,3,4,2,5]
    "#),
    format!("{}", r#"
~ range n := (
    ? n
    | x -> (
        ? x > 0 
        | 1 -> (range (n - 1)) + [n - 1]
        | 0 -> []
    )
    | _ -> []
);
    "#),
    ]
} 
