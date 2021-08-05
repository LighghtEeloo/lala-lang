# Lala

An expression oriented programming language / data notation, designed for elegant data processing and transference.

## Expression

All terms in `lala` will eventually converge to an expression, so it's fair to start from. Here is a list of all forms of expressions. The list will be revisited as new terms are introduced.

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

1. Literals are just the application of the type `primitive` defined by a type constructor in `core`.
2. An obstruction is a data block with certain computational order, a projection accesses the result of a previous block, and an exposure opens up a binded data block.
3. Abstraction can be analogized by variables and functions, and works through bindings.

## Basics 

Let's bootstrap with some quick examples.

### Literal

Literals contain int, float and string.

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

### Binders

Use binders to introduce new variables and functions.

```lala
ans := 42;
x   := ans;
x'  := ans - 1; /* only suffix `'`s are allowed */
add x y := x + y;
```

### Operators

Operators are special binders defined in `core`.

// Todo..

### Type

Use type by prefixing `'`.

```lala
truth' := bool'true; /* type'data */
truth := 'true; /* shortened, equal effect */
```

### Blocks

// Todo..

### Comments and Docs

Finally, perhaps key to maintainable data, let's introduce comment and document syntax.

```lala
/* Comment */

/* => tag
any markdown content
 */
```

within the markdown area, you may write anything that helps, from a speech to a note, which will be collected together with the binder directly below and renderred to a static doc page. The tags can be used to cluster the pages in a package; leave it blank if not useful.

If the doc is file-wise, or no binder is available, use `!!`.

```lala
/* => tag !!
any markdown content
 */
```



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

// Todo: Set treated as Map (?)

## Binder Space and Value Space

We use different separators for the two spaces. `;` acts upon the binder space, standing for a pulse of computation. an operation that don't show effect by returning value, and a reusable piece of code; while `,` acts upon the value space, standing for conjunction and product of values, and construction or deconstruction of things. 

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

```lala
data := [
    a := [];
    b := [];
];
<data>;
```

is equal to 

```lala
a := [];
b := [];
```

note that

```lala
data == [<data>];
```

// Note: Exposure with take out pattern?

```lala
<a; b; c> := data;
/* equals to */
_ := <a; b; c> = data;
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

### Projection

// Todo..


### Mask and Mask Exposure

// Todo..

```lala
<sin; sin'> := trigonometric = [
    math := [<lala>].math;
    <math> := [<lala>];
    <math> := lala;
    sin  := math.sin;
    sin' := math.cos;
];
```

Note that three `math`'s are the same.

```lala
<sin; cos> := trigonometric = [
    <lala.math>
];
```

// Note: see Exposure.


### Projection Visibility

Principles: 
1. If a block can be seen, it can be projected.
2. If a binder binds to a block, it is the block.
3. If a binder is masked by `=`, it can't be seen outside the containing block.
4. If a binder is masked by `:=`, it can be seen outside the containing block.

Thus `=` binders can't be projected to outside the containing block, while `:=` binders can.

Note that binders appear in the form of patterns.

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

Under the hood, the binders (for functions) only takes one actual argument, which is a lazy-evaluated list; you can read more in [pattern language section](#pattern-language).

### Modular Input

Suppose we want to pass a data block to a function and make use of the bindings immediately, considering it's similar to exposure, we could write

```lala
f <x;y;z> := x + y + z;
res = f [x := 3; y := 4; z := 5];
```

Note that it's unreasonable to write

```lala
/* error */
f <x;y;z> := <res> = [ 
    res := x + y + z;
];
```

because `res` would be ambiguous.


## Pattern Language

As we've in fact incountered many pattern usage, I think it's a good time now to formally introduce pattern language in lala:

```lala
/* deconstruct a list or array */
 a b c
[a,b,c]
[a]+[b]+[c]
ab+[c] /* favored for performance (?: dl-list) */
_ +[c]
[a]+bc
[a,_,_]
[a, ..]

/* deconstruct a tuple */
 a,b,c
(a,b,c)
(a,_,_)
(a, ..)

/* deconstruct a hashmap */
{0: a, 1: b, 2: c}
{"king": a, "queen": b, "eeloo": c}
{0: a, ..}

/* deconstruct a block */
<a;b;c>
<*>
```

Notice that there's little `;` in pattern language because `,` looks better (kidding). In fact, the design principle traces back to the difference between binder space and value space. Almost all patterns are dealing with values, so `,` appears everywhere, except `<a; b; c>` as it deals with binder space elimination.

A few other comments:
1. ` a b c` may seem a bit confusing; but actually we're using it all the time. It's in fact just function arguments, passed to the function in a sequence, one by one. The list itself is lazy-evaluated, so it's deconstructed, from right to left, eval one single right-most element in the list at a time, and take it to a function that receives one less argument, until the list is empty and the binding is reduce from function to a variable.
2. `ab+[c]` should be favored over `[a]+bc`, unlike most fp language's behavior. This is because lala prefers vector impl over linked lists. e.g., in the use case of `json` a vector is better most of the time. Say a history of operations are stored. Usually we append, not prepend the latest events.


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

But you may choose not to force the type, leaving it ducked:

```lala
tree := '{
    leaf '[
        :data,
    ],
    node '[
        :data,
        :lt,
        :rt,
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

// Todo: All data structures can be typed.

// Todo: Binders defined with type (member function / variable).

## Control Flow and Pattern Matching

// Todo...


## Lala File and Package

// Todo..


## Package Manager

It's called `dada`.

Defines how "global" data blocks within lala's domain maps to reality, i.e. file system or repositories.

Though with a special purpose, `dada` is just lala syntax, with no magic at all.
