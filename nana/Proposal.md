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
> A hash-map can also be ordered by insertion, see Dictionary in Python 3.6+.
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

A kindly note: `:=` and `=` are introduced to control visibility, but they all means binding. Now you can just treat them as the same thing. We'll get to this later.

### Sequence of Computation

#### Overview

Learning from classical Monad idea, as well as some modifications, three forms of computation sequences are introduced.

1. Parallel <=> `()`.
2. Sequential <=> `[]`.
3. Dependent <=> `{}`.

All three have been carefully designed to address typical and practical issues. 


#### Parallel

As a mental model, a parallel block can run all the computations inside it parallelly, without any compile time optimization. The result of the parallel computaion will not show up until all computations are done (like `join` in multi-thread model). To realize that, the programmer must ensure:

1. No single computation will use the result of a sibling computation.

And yes, that's all, but that's actually quite strong a requirement.

e.g.

```nana
(
    a = 1;
    b = 2;
    c = a; // error, using sibling `a`
    a = 4; // error, binder `a` has been defined
)
```

The use of a sibling computation is also called *overlap*.

But note that one may safely write recursive functions:

```nana
(
    factorial x := (
        ? x                         // what is x?
        | 0 -> 1                    // if x is 0
        | _ -> x * factorial (x-1)  // else
    );
    factorial 10
)
```

This function is not safe, but let's take it for now, before things get too complicated.

#### Sequential

A sequential block deals things one after another. This means that one can shadow a binder: you can bind different things to a same binder. It also meaning that you can only use previously defined things, but not latter. This is in fact natural in most programming languages, like Python, Rust or Ocaml. As rules:

1. One may define new binders based on existing bindings.
2. One may shadow old binders with a new binding.

e.g.

```nana
[
    x = 4;
    x = y;      // error, binder `y` is not defined
    y = x;      // 4
    x := x + 1; // 5
    y := y + x; // 9
    x = "cow";
]
```

The new binding will always shadow the old one, and by the end of the evaluation, `x` is just `"cow"` and `y` is just `9`, end of story.

But shadowing doesn't come without a price. One can't write recursive functions in sequential blocks. Observe the following:

```nana
[
    x = 0;
    x = x + 1; // notice the `x` in `x + 1`
]
```

Should we treat that `x` as `0`, which is the value, or the `x` itself, which is being defined? The criterion can easily grow complicated. To avoid ambiguity, Nana closes the door of recursion in sequential blocks, and now `x` is just `0`.

But recursive functions are still possible:

```nana
[
    rec = (
        factorial x := (
           ? x                         // what is x?
           | 0 -> 1                    // if x is 0
           | _ -> x * factorial (x-1)  // else
       );
    );
    factorial = rec.factorial;

    // alternatively...

    <factorial> = (
        factorial x := (
           ? x                         // what is x?
           | 0 -> 1                    // if x is 0
           | _ -> x * factorial (x-1)  // else
       );
    );
]
```

#### Dependent

Sometimes we need to define things that are mutually recursive, or in arbitrary order. So here comes that dependent computation. One is now allowed to define things both recursively and in any order; however no shadowing is allowed.

e.g.

```nana
{
    a = 1;
    a = 2; // error, binder `a` has been defined

    // insertion sort
    insert x l = (
        ? l
        | [] -> [x]
        | [y] + ys -> (
            ? x < y
            | 1 -> [x, y] + ys
            | 0 -> [y] + insert x ys
        );
    );
    sort l := (
        ? l
        | [] -> []
        | [x] + xs -> insert x (sort xs)
    );

    // mutual recursion
    is_even n := (
        ? n
        | 0 -> 1             // true
        | _ -> is_odd (n-1)
    );
    is_odd n := (
        ? n
        | 0 -> 0             // false
        | _ -> is_even (n-1)
    );
}
```


#### Motavation

Well, why does this even matter? Does it sound over-reacted?

The sequence of computation has been a long debated question in all programming languages, imperative or functional. 

I would like to share some bad examples (jokes?) of unclear evaluation order - though mostly under the situation of function calls.

A headache in all C++ learner's head:

```c++
int a = 1;
func(a, a++, ++a);
```

What, then, will be sent into `func` eventually?

In the circle of FP, the opinions are also divergent. 

```ocaml
func a b c
```

OCaml will choose to evaluate `c` first. It was quite a shock for me, as I once thought it would be more natural to evaluate it as

```ocaml
(((func a) b) c)
```

where `a` should come first. 

Another negative example would be the naming issue in Elm, which has dragged down the language experience. One can hardly write any duplicate name inside the same scope - not even in the parameters of a function being defined! There's no need to dive into this topic, but if you like, you can try elm for a few hours and you'll get what I mean.

One may argue that the implementations have done enough dirty work, for example the laziness in evaluation, and one should never rely on such feature. I do agree, but I think we can do better. 


In Nana, one can always ensure the order of evaluation by passing a well-defined block, meaning:

```nana
(
   func (x,y,z) = (...);
   func (a,b,c) 
)
```

`a`, `b` and `c` will be evaluated in an arbitrary order, and will be passes into the function as (x,y,z), after all of them are computed.

```nana
(
    func [x,y,z] = (...);
    func [a,b,c]
)
```

`a`, `b` and `c` will be evaluated as their index order, meaning `a` first, `c` last, and will be passes into the function as `[x,y,z]`, one after another, meaning `func [x,y,z]` => `func_x [y,z]` => `func_xy [z]` => `func_xyz`.

If one directly passes several arguments instead, the compiler will try its best to optimize, and no evaluation order will be guaranteed, meaning

```nana
// passing a block
func (x,y,z) = [
    r = ...;
    s = ...;
    t = ...;
]
func (a,b,c)

// passing separated arguments
func x y z = [
    r = ...;
    s = ...;
    t = ...;
]
func a b c
```

One can ensure that in the first case the evaluation order will be `(a,b,c) => r => s => t`, but in the second case one may only ensure `r => s => t` obeys the order, while `a`, `b` and `c` may get evaluated at any time when `r`, `s` and `t` is been evaluated, without any given order.

#### Conclusion

| Computation | Symbol | Overlapping | Shadowing | Recursion |
| :---------: | :----: | :---------: | :-------: | :-------: |
| Parallel    | `()`   | No          | No        | Single    |
| Sequential  | `[]`   | Previous    | Yes       | None      |
| Dependent   | `{}`   | Two-ways    | No        | Mutual    |

### Exposure

### Function Binding and Gated Block



## Value Space



## Gated Block

## Pattern Language
