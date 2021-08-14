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

## Cheatsheet

Literal.

```nana
0      // int
0.0    // float
"na"   // str
[|la|] // raw
```


Block.

```nana
()       // unit
( a )    // expr
( a, b ) // tuple
[]       // nil
[ x, y ] // vector
{}       // null
{ s, t } // hash-set
{ 0: m, 1: n } 
         // hash-map
```


Binder Space & Value Space.

```nana
// Any block has both spaces.
(
    binded = 0; // binder space
    1           // value space
)

// Three blocks have different binding rules.

(
    one = 0;
    another = 1; // can't shadow; can't overlap
    // `nonono = another;` will be wrong.
)
// value: ()

[
    one = 0;
    one = 1;     // can shadow
    final = one; // can overlap
    final        // 1
                 // sequentially evaluated
]
// value: [1]

{
    two = one + 1;  // can overlap; can't shadow
    zero = 0;
    one = zero + 1; // but can figure out dependency
    // `one = 1` will be wrong; name collision
    "result": two, "null": zero
}
// value: {"result": 2, "null": 0}
```


Abstraction, and Encapsulation.

```nana
x = 1;      // `=` binds any expression to a "binder"
y = x + 1;  // binder `y` has value `2`
blk = [
    x = x;  // binder `x` has value `1`
    y := y; // binder `y` has value `2`
    x,x,x,y,y
]; // binder `blk` has value `[1,1,1,2,2]`
blk' = blk; // binder `blk'` is legal binder, and has:
            // 1. bindings of `x = 1; y := 2;`
            // 2. value of [1,1,1,2,2]
res = blk.y // binder `res` has value `2`
            // note that `blk.x` would be wrong,
            // because `x = x;` defines a local binding,
            // while `y := y;` defines an exposed binding.
```


Exposure.
```nana
blk = {
    x := 1;
    y := 2;
};
<x; y> = blk; // exposing `x` and `y`
// equals to
x = blk.x;
y = blk.y;
```


Gated Block. Or closure, if you prefer.
```nana
// Any block can be gated.
| | ()              // same as `()`
|x| ()              // different; `(|x| ()) ()` is `()`
never = |x| ();     // `never anything` is `()`
never = |_| ();     // `x` not used, name it `_` as arbitrary
gated_map = |x| {
    "ei": x,
};                  // just like a function
gated_map 0         // application
// value: { "ei": 0 }
```


Package Management:
1. A file is in fact a block of tuple. 
    1. Any file is treated as a block, guarded by `()`.
    2. An empty file is `()`.
    3. A standalone block in a file will be treated as itself: `([])` == `[]`.
    4. Generally, any standalone expression in a file is treated as itself.
2. Files can be managed by a top-level nana file, conventionally `Pac.da`.

