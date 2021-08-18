# Nana

An expression oriented programming language / data notation, designed for elegant data abstraction.

This is a proof-of-concept sub-language derived from Lala (or as a prelude).

## Core Concepts

All terms in `nana` will eventually converge to an expression.

1. Block
    - Encapsulation
    - Exposure
2. Binder Space
    - Abstraction
    - Gated Block
3. Value Space
    - Literal
    - Application
    - Projection

We will discuss them one by one.

## Block - Introduction

Block is the core concept in Nana. There are three blocks in total:

```nana
()
[]
{}
```

Each block corresponds to both a type of *data-structure* and a way of *computation*.

In Nana, one may use:

1. `(x, y, z)` as a tuple, which binds multiple values together. 
2. `[i, j, k]` as a list, which both grants indices to list elements and an order.
3. `{}` as hashable.
    1. `{a:x, b:y, c:z}` as a hash-map, which creates a table of key-value pairs.
    2. `{a, b, c}` as a hash-set, which is unordered and unique.

which is quite close to one's intuition. See [Value Space](#value-space) for further explanation.

> Observation: 
> 
> Tuple may serve as a function input and output syntax. When applying a function in most languages, one can just treat it as passing a tuple to the function, which seems to be syntactically same.
> 
> Both list and array serve as linear containers, but they are vastly different. To quickly index elements, one may use array; to easily insert and delete, one may use list; the problem is they are fighting for the same syntax. 
> 
> A hash-map can also be ordered by insertion, see Dictionary in python 3.6+.
> 

However, Nana is more than just representing data. It also cares about reusing them. That's where bindings comes in as a remedy. In any block you may write bindings:

```nana
(
    a := 1;
    b := 2;
    a, b
)
```

where `a` and `b` are binders. You may see them as "variables" or "functions". The block itself will be evaluated as

```nana
(1, 2)
```

But that's only half the story. The binding are still kept for reuse during the hole compilation, which means:

```nana
(
    tuple := (
        a := 1;
        b := 2;
        a, b
    );
    tuple.a, tuple.b, tuple
)
```

will be evaluated as

```nana
(1, 2, (1, 2))
```

Here in `tuple` we treat `a = 1; b = 2;` as binder space, and `a, b` as value space. The spaces will be treated differently according to the blocks it lies in. See [Binder Space](#binder-space) section for further explanation.

Also all blocks can be "gated" and form a gated block. This is similar to the concept of "closure", or "lambda function". It provides a way to eject binders at the time of function application. Another hello world example is

```nana
(
    add := |x, y| (x + y);
    add 18 24
)
```

Which leads to

```nana
42
```

Some interesting comments:
1. An empty gate is exactly the same as an ungated block (feel free to suprise).
```nana
(
    tuple := || (
        a := 1;
        b := 2;
        a, b
    );
    tuple.a, tuple.b, tuple
)
```
2. You may write functions in a more confortable way, which will also be translated to the gated block form:
```nana
(
    add x y := (x + y);
    add 18 24
)
```

More information can be found at [Gated Block](#gated-block) section.

## Binder Space

### Sequence of Computation


### Exposure

### Function Binding and Gated Block



## Value Space



## Gated Block
