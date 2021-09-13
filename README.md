# Lala
An expression-oriented data notation, aimed at transpiling itself to any cascaded data notation.

Lala is separated into three components: `Nana`, `Lala`, and `Dada`.


## Motivation

Checkout [Motivation](docs/Motivation.md).

<!-- ## Quick Examples -->

<!-- ### Direct Tranpilation

Some examples will be provided here as an introduction to Lala.

- lala4html
- lala4css
- lala4json
- lala4toml
- lala4yaml -->

<!-- ### Functional Programming in Lala

```lala
/* `~` means definition, like `let` or `var` */
~ qsort xs := (
    /* pattern matching */
    ? xs
    | [] -> []
    | [x] + xs -> (
        ~ (s, l) = list.partition ((>) x) xs;
        (qsort s) + [x] + (qsort l)
    )
);
qsort [1,3,4,2,5]
```

### Object-Oriented Programming in Lala

```lala
/* definition of a "class", `student` */
~ student <name; sleep; ability; gpa> := [
    ~ name := name;
    ~ study := (
        ?? sleep 
        | name + " doesn't want to study." 
        | name + " is diligent!"
    );
    ~ exam difficulty := (
        ~ base = (
            ?? gpa >= 4.0
            | ability
            | (
                ?? sleep
                | ability - difficulty
                | ability - 2 * difficulty
            )
        );
        ?? base > 0
        | base
        | 0
    );
];

/* application, creating two instances */
~ alice := student [
    ~ name := "Alice";
    ~ sleep := 0;
    ~ ability := 100;
    ~ gpa := 4.3;
];
~ bob := student [
    ~ name := "Bob";
    ~ sleep := 1;
    ~ ability := 90;
    ~ gpa := 3.7;
];

/* get value; of course, it could be better abstracted */
{
    alice.name: {
        "study_status": alice.study,
        "exam_result": alice.exam 20,
    },
    bob.name: {
        "study_status": bob.study,
        "exam_result": bob.exam 20,
    }
}
```

the result is
```json
{
    "alice": {
        "study_status": "Alice doesn't want to study.",
        "exam_result": 100
    },
    "bob": {
        "study_status": "Bob is diligent!",
        "exam_result": 70
    }
}
``` -->


## First Glance

[Nana CheatSheet](nana/CheatSheet.md)


## Language Proposals
- Nana: [nana-lang proposal](nana/Proposal.md).
- Lala: [lala-lang proposal](lala/Proposal.md).
- Dada: tbd.


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
2. type system
3. a standard library

### Dada

A (fake) package manager in pure Lala, designed more as a structural convention than a heavy building system.


## Current Progress
Using [lalrpop](https://github.com/lalrpop/lalrpop) as nana/lala's parser to Rust.

Currently dealing with symbol resolution.

Also, here is a [todo list](docs/Todo.md).
