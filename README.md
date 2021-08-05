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
- lala as fp
- lala with oo

## Roadmap

### Nana

Nana, a proof-of-concept language, will first be implemented, along with the following key concepts:
1. Block
    1. binder space and value space
    2. encapsulation by mask
    3. exposure
2. Abstraction
    1. currying by arg sequence
    2. application into value space
    3. binder as pattern
3. Control Flow
    1. pattern matching

### Lala

Nana++. 

1. the order of computation embedded in data structure
2. more data structures as primitives
3. type system
4. a standard library

### Dada

A (fake) package manager in pure lala, designed more as a structural convention than a heavy building system.


## Language Proposal
- Nana: tbd.
- Lala: [lala-lang proposal](lala/Proposal.md).
- Dada: tbd.

## Current Progress
Using [lalrpop](https://github.com/lalrpop/lalrpop) as nana/lala's parser to Rust.
