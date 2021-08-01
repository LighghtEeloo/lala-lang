# Lala

An expression oriented programming language / data notation, designed for elegant data processing and transference.

## Expression

All terms in `lala` will eventually converge to an expression. Here a list of all expressions is given. The list will be revisited again and again as new terms are introduced.

List of Expression

1. Type System
    - Literal
    - Type Constructor
2. Modular System
    - Obstruction
    - Projection
    - Exposure
3. Context System
    - Abstraction
    - Application

where

1. Literals are just the application of the type `primitive`.
2. An obstruction is a data block with certain computational order, a projection accesses the result of a previous block, and an exposure opens up a data block.
3. Abstraction can be analogized by variables and functions, and works through bindings.

## Basic Literal

Literals are int, float and string.

```lala
42;       /* int */
-1;       /* int */

0.0;      /* float */
0.;       /* float */

"";       /* string */
"Ehaema"; /* string */
```

where `/* ... */` represents comment.

A more sophisticated literal will come later.

## Data Block

Perhaps the most fundamental question in short history of computer science is the question of how to collect, process and present data in a safe and flexible way. `lala` is yet another attempt of an elegant solution for data abstraction.

### Stance

It all started with the sequence of evaluation. We want to collect the result of our computation in two ways: either *sequential*, or *parallel*. If the latter computation depends on the result of the previous one, it's *sequential*; if we insist that a set of computations must all complete regardless of the order of completion within the set, it's *parallel*. 

If you admit that all data processing is eventually a sequence of evaluation, then this natural to represent them with data blocks. To differentiate the two sequences of evaluation, `[]` and `{}` are introduced as primitives.

Nullary.

```lala
[];
{};
```

// Todo..

Since each expression is a computation, it's not abrupt to propagate the use of `;`.

### Usage as List and Map

// Todo: List and Map

```lala
list := [0;1;2;3];
map  := {
    0: 0;
    1: 1;
    2: 2;
    3: 3;
};
```

// Todo: Weak typed list and map

// Todo: Array treated as Map (?: use typed)

// Todo: Set treated as Map (?)

## Exposure

// Todo..

## Abstraction

Abstraction is so common and signaficant in programming languages, but it's so hard to do it right. 

// Todo: example of function argument evaluation order.

// Todo..

### Binder and Binding

// Todo..

### Mask and Mask Exposure

// Todo..

```lala
trigonometric : <sin, sin'> = [
    math = [<lala>].math;
    sin  := math.sin;
    sin' := math.cos;
];
```

```lala
trigonometric : <sin, sin'> = [
    <lala.math>
];
```

### Currying

// Todo: Sequential is currying

```lala
add [x;y] := x + y; /* function definition `add` */
res := add [1;2];   /* 3 */
```

Sugaring to make user happy:

```lala
add x y := x + y;
res := add 1 2;     /* 3 */
```

Note that both notations of function abstractions and applications above are legal and equal to each other.

As for currying,

```lala
add_1 := add 1;
```

partially apply a function will produce any other function with fewer arguments to be fed.

### Modular Input

Suppose we want to pass a data block to a function and make use of the bindings immediately, considering it's similar to exposure, we could write

```lala
f <x y z> := x + y + z;
res = f [x := 3; y := 4; z := 5];
```

Note that it's unreasonable to write

```lala
f <x y z> : <res> = [ /* error */
    res := x + y + z;
];
```

because `res` would be ambiguous.

## Type Constructor

// Todo..

## Pattern Language

As an extended topic, let's introduce pattern language in `lala` by first concluding the pattern that already appeared:

// Todo..

```lala
 a b c
[a; b; c]
<a, b, c>
```

## Effect Shift

// Todo..

```lala
data := ![
    alice  := "kawaii";
    ehaema := "elegant";
    carol  := [ alice; ehaema ];
];
```

is effectively

```lala
data := {
    "alice"  : "kawaii";
    "ehaema" : "elegant";
    "carol"  : [ "kawaii"; "elegant" ];
};
```

## Lala File

...

## Package Manager

It's called `dada`.

Defines how "global" data blocks within lala's domain maps to reality, i.e. file system or repositories.

Though with a special purpose, `dada` is just lala syntax, with no magic at all.
