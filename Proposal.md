# Lala

An expression oriented programming language / data notation, designed for elegant data processing and transference.

## Expression

All terms in `lala` will eventually converge to an expression. Here a list of all expressions is given. The list will be revisited as new terms are introduced.

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
2. An obstruction is a data block with certain computational order, a projection accesses the result of a previous block, and an exposure opens up a binded data block.
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

Perhaps the most fundamental question in the short history of computer science is the question of how to collect, process and present data in a safe and flexible way. `lala` is yet another attempt of an elegant solution for data abstraction.

### Stance

// Note: !! Add Generalized Tuple `()` as true parallel; Tuple = Array made better?; `{}` for dependent, simultaneous computation.

It all started with the sequence of evaluation. We want to collect the result of our computation in three ways: either *sequential*, *simultaneous*, or *parallel*. If the latter computation depends on the result of the previous one, it's *sequential*; if the order of computation satisfies a resolvable dependency graph, it's *simultaneous*; if we insist that a set of computations must all complete regardless of the order of completion within the set, it's *parallel*. 

If you admit that all data processing is eventually a sequence of evaluation, it would natural to represent them with data blocks. To differentiate the three sequences of evaluation, `[]`, `{}` and `()` are introduced as primitives.

Nullary.

```lala
[];
{};
();
```

// Todo..

Since each expression is a computation, it's not abrupt to propagate the use of `;`.

### Usage as List and Map

// Todo: List and Map

```lala
list := [0,1,2,3];
map  := {
    0: 0,
    1: 1,
    2: 2,
    3: 3,
};
```

// Todo: Dynamically typed list and map

// Todo: List is more of a vector, for easier usage (?: doubly-linked list)

// Todo: Array treated as Tuple

// Todo: All can be typed

// Todo: Set treated as Map (?)

## Binder Space and Value Space

We use different separators for the two spaces. `;` acts upon the binder space, standing for a pulse of computation. an operation that don't show effect by returning value, and a reusable piece of code; while `,` acts upon the value space, standing for conjunction and product of values, and construction or destruction of things. 

// Todo..

### Effect Shift

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
    "alice"  : "kawaii",
    "ehaema" : "elegant",
    "carol"  : [
        "kawaii",
        "elegant"
    ],
};
```

## Exposure

// Todo..

// Note: Exposure with take out pattern?

```lala
<a; b; c> := data;
/* equals to */
_ : <a; b; c> = data;
```

## Abstraction

Abstraction is so common and signaficant in programming languages, but it's so hard to do it right. 

// Todo: example of function argument evaluation order.

// Todo..

### Binder and Binding

// Todo..

Note that a binder can be binded to any expression except an abstraction

```lala
f := g := []; /* invalid */
```

since it's not making any sense. If one insist, one should write the following instead.

```lala
f := [
    g := [];
];
```

### Mask and Mask Exposure

// Todo..

```lala
trigonometric : <sin; sin'> = [
    math = <lala>.math;
    sin  := math.sin;
    sin' := math.cos;
];
```

```lala
trigonometric : <sin; sin'> = [
    <lala.math>
];
```

// Note: see Exposure.

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
f <x;y;z> := x + y + z;
res = f [x := 3; y := 4; z := 5];
```

Note that it's unreasonable to write

```lala
/* error */
f <x;y;z> : <res> = [ 
    res := x + y + z;
];
```

because `res` would be ambiguous.


## Type Constructor

```lala
bool := '{
    true,
    false,
};
truth := bool'true;
falsehood := 'false;
```

To bind data and use polymorphism,

```lala
tree 'a := '{
    leaf '[
        :data 'a,
    ],
    node '[
        :data 'a,
        :lt tree 'a,
        :rt tree 'a,
    ],
};
```

But you may choose not to force the type:

```lala
tree := '{
    leaf '[
        :data,
    ],
    node '[
        :data,
        :lt tree,
        :rt tree,
    ],
};
```

To recursively define types,

```lala
http_response_status 'data := '{
    informational '{
        continue,
        switching,
        /* ... */
    },
    successful '{
        ok '[
            :data, /* equal to `:data 'data` */
        ],
        created,
        accepted,
        /* ... */
    },
    /* ... */
};
```


## Pattern Language

Introducing pattern language in lala:

```lala
/* destruct a list or array */
 a b c
[a,b,c]
[a]+[b]+[c]
ab+[c] /* favored for performance (?: dl-list) */
_ +[c]
[a]+bc
[a,_,_]
[a, ..]

/* destruct a tuple */
 a,b,c
(a,b,c)
(a,_,_)
(a, ..)

/* destruct a hashmap */
{a,b,c}
{a,_,_}
{a, ..}

/* destruct a block */
<a;b;c>
```

Notice that there's little `;` in pattern language because `,` looks better (kidding). In fact, the design principle traces back to the difference between binder space and value space. Almost all patterns are dealing with values, so `,` appears everywhere, except `<a; b; c>` as it deals with binder space elimination.

## Lala File

// Todo..

## Package Manager

It's called `dada`.

Defines how "global" data blocks within lala's domain maps to reality, i.e. file system or repositories.

Though with a special purpose, `dada` is just lala syntax, with no magic at all.
