# Lala
An expression oriented data notation, aimed at transpiling itself to any cascaded data notation.

## Quick Examples

Some examples will be provided here as an introduction to lala.

- lala4html
- lala4css
- lala4json
- lala4toml
- lala4yaml

Also some "just because I can" examples:

### Functional Programming in Nana

```lala
~ qsort xs := (
    ? xs
    | [] -> []
    | [x] + xs -> (
        ~ (s, l) = partition ((>) x) xs;
        (qsort s) + [x] + (qsort l)
    )
);
qsort [1,3,4,2,5]
```

### Object-Oriented Programming in Nana

```lala
~ student <name; sleep; ability; gpa> := [
    ~ study := (
        ?? sleep 
        | name + " doesn't want to study." 
        | name + " is diligent!"
    );
    ~ exam hardcore := (
        ~ base = (
            ?? gpa >= 4.0
            | ability
            | (
                ?? sleep
                | ability - hardcore
                | ability - 2 * hardcore
            )
        );
        ?? base > 0
        | base
        | 0
    );
];

~ hcz := student [
    ~ name := "hcz";
    ~ sleep := 0;
    ~ ability := 100;
    ~ gpa := 4.3;
];

{
    "study_status": hcz.study,
    "exam_result": hcz.exam 100,
}
```

the result is
```json
{
    "study_status": "hcz doesn't want to study.",
    "exam_result": 100
}
```

## Roadmap

### Nana

Nana, a proof-of-concept language, will first be implemented, along with the following key concepts:

1. Block
    - Obstruction
    - Projection
    - Exposure
2. Value Space
    - Literal
    - Application
3. Binder Space
    - Abstraction
    - Pattern Language
4. Control Flow 
    - Pattern Matching

### Lala

Nana++. 

1. the order of computation embedded in data structure
2. more data structures as primitives
3. type system
4. a standard library

### Dada

A (fake) package manager in pure lala, designed more as a structural convention than a heavy building system.


## Language Proposal
- Nana: [nana-lang proposal](nana/Proposal.md).
- Lala: [lala-lang proposal](lala/Proposal.md).
- Dada: tbd.

## Current Progress
Using [lalrpop](https://github.com/lalrpop/lalrpop) as nana/lala's parser to Rust.
