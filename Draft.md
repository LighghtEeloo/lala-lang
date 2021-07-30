# Lala

A layout-insensitive, data & expr oriented programming language, designed for elegant data share and transfer.

## Data Block

Nullary.

```lala
[]
```

Binder (data field).

```lala
[
    field_within := []
]
```

Variables are binders scoped within the data block where it's defined:

```lala
data_name := [
    field_within := []
    field_another := field_within
]
```

A file named `data_name.la` above can be written as

```lala
field_within := []
field_another := field_within
```

But it's always a good habit to guard a lala data block with a binder.

We will formally introduce binders later.

## Projection and Mask

To access a field of a data block (projection):

```lala
data := [
    shallow := [
        deeper := []
    ]
    deep := shallow.deeper
]
```

To expose all in the data block:

```lala
data := [
    shallow : [*] = [
        deeper := []
    ]
    shade := shallow
    deep := deeper
]
```

or expose after definition:

```lala
data := [
    shallow := [
        deeper := []
    ]
    shade : [*] = shallow
    deep := deeper
]
```

or better, without noise:

```lala
data := [
    shallow := [
        deeper := []
    ]
    _ : [*] = shallow
    deep := deeper
]
```

but let's go further:

```lala
data := [
    [
        deeper := []
    ]
    deep := deeper
]
```

Inversely, a data block may define what to expose by wearing a mask:

```lala
data := [
    shallow : [ deeper ] = [
        deeper := []
        darker := []
    ]
    deep := deeper
]
```

```lala
data := [
    shallow : [ deeper; other ] = [
        deeper := []
        other  := []
        darker := []
    ]
    deep := deeper
    another := other
]
```

To strictly protect a field, one can refuse to "define" what's inside:

```lala
data := [
    shallow : [ deeper ] = [
        deeper := []
        darker := []
        closed = [
            fuckyou := []
        ]
    ]
]
```

So that inside scope `data`, `deeper` can be directly accessed, `darker` with projection, and `closed` can never be seen.

Principle: if a field can be accessed, it can be later exposed; vice versa.

## Context

Data blocks can be used to provide a hidden context. 

```lala
data := [
    ctx := []
] >> [
    name := ctx
]
```

which can be treated as

```lala
data := [
    ctx = []
    name := ctx
]
```


## Numbers and Calculation

...

## Expression

...

## Convertion and Null Safety

Convert data to latter type:

```lala
data := [
    x := 10 => 0.0
]
```

But sometimes a convertion may fail:

```lala
data := [
    x := 10.0 => ?0
]
```

`Some(0)` and `None` forms a non-null expression; in lala this is presented by

```lala
data := [
    x := ?0
]
```

which is called "optional 0".

To convert by force will probably panic the program:

```lala
data := [
    x := 10.2 => 0
]
```

To view optional data, use

```lala
data := [
    x  := ?0 : 1
    x' := ?0 => 0
]
```

The former provides a default value of 1, and the latter unwraps by force.

To convert with default:

```lala
data := [
    x  := 10.0 => ?0 : 10
    x' := 10.1 => ?0 : 10
]
```

Both will produce `10`.


## Primitive

All lala code can be stored as string. To represent primitive string:

```lala
[|field := []|]
```

and evaluate:

```lala
data := *[|field := []|]*
```

or with variable:

```lala
data_pri := [|field := []|]
data     := * data_pri *
data     := data_pri => []
```

To create primitive from lala data:

```lala
data     := [ field := [] ]
data_pri := | data |
data_pri := data => [||]
```

Primitives are used together with macros, which will be later introduced.

Todo: Current primitive implementation requires the data to be blocked.
While it's easier for language design and can let the user choose how to
use the marco product by binding and exposing at their own will, it's not
necessarily a limitation. Consider redesigning in the future.

## String and Comments

To introduce generalized string and comment:

```lala
/* Only block-like comments are allowed. */
str :=  "prpr := []"  /* string */
str := #"prpr := []"# /* raw string */
pri := [|prpr := []|] /* primitive */
```

while str is not effectively typed pri, but can be converted to:

```lala
pri := "prpr := []" => ?[||] 
```

or even:

```lala
data := "prpr := []" => ?[]
```


## Binders & Functions

...

Definition.

```lala
data := [
    func x := x + 1
]
```

Application.

```lala
data := [
    add x y := x + y
    a := add 1 2
    b := add (add 1 2) 0
]
```

Any binder can take a variable list and become a "function"; function is not special in lala. Therefore one may apply a variable list to any data block, including a file.

A function is only a partially evaluated data:

```lala
data name := [
    <prelude>
    name := name
    cat  := name + #"'s pet cat"#
    revd := rev name
    house := [
        name := name
        cat  := cat
        revd := revd
    ]
]

house := (data "Ehaema").house
```

...


## Macros

Marcos are functions that takes and returns a primitive.


## Seq and Par

Data fields are sequentially evaluated; can be shadowed:

```lala
data := [
    cat := [|darling := "blue"|]
    cat := [|darling := "black"|]
    cat := "meow"
    cat := "prpr"
]
```

...


But besides sequential evaluation, one may want parallel evalution.

```lala
data := [
    amer := {
        black := []
        white := []
    }
    asian := []
]
```

where black and white are evaluated without order; meanwhile asians are discriminated.

...




## Array and Map

The difference between sequential data block (Seq) and parallel data block (Par)
is effectively the difference between Array and Map. Therefore one may write

```lala
data := [
    arr := [
        [0]
        [1]
        [2]
    ]
    map := {
        0: [0]
        1: [1]
        2: [2]
        "hashable": [3]
    }
]
```

to represent them respectively.

Note that one may use anything hashable to hash the map, but it must be constant to provide better performance; one may still choose to use a binded value, of course.

No duplicated keys are allowed, the following:

```lala
data := [
    binder := "hey"
    map := {
        0: [0]
        1: [1]
        binder: [2]
        "hey": [3]
    }
]
```

will panic the program.

To access, use

```lala
data := [
    arr := [
        [0]
        [1]
        [2]
    ]
    map := {
        0: [0]
        1: [1]
        2: [2]
    }
    arr_0 := arr.0
    map_2 := map.2
]
```

Note that they are effectively data blocks, only indexed.

Inserting bindings within is still allowed:

```lala
data := [
    arr := [
        [0]
        hi := "hi"
        [1]
        [2]
    ]
    map := {
        0: [0]
        1: [1]
        hey := "naughty"
        2: [2]
    }
    arr_0 := arr.0
    map_2 := map.2
    hi    := arr.hi
    hey   := arr.hey
]
```

where binders will not be indexed but will be evaluated by the order the data block requires.

Bindings don't need to follow the same type as the indexed items.

## Open

You may open everything within a data block in the **upper** scope.

```lala
data := [
    cat := []
]
newland := [
    <data>
    catty  := cat
    data'  := [<data>]
    kitten := data'.cat
    data'' := data
    catta  := data''.cat
]
```

Note that file is a valid data block, so it can be opened as well.

In fact one may choose to use

```lala
data := [
    cat := []
]
newland := [
    _ : [*] = data
    catty := cat
]
```

But I think it's nice to have a shorthand.

Open syntax can also be used as file IO:

```lala
data := "Hi."
newland := [
    data'  := <"data">
    data'' := "Hi."
]
```


## Type & Data Constructor

A type constructor is defined by `'{}`.

```lala
data := [
    bool := '{
        true
        false
    }
]
```

Note that type constructor is also scoped.

To use a corresponding data constructor,

```lala
data := [
    bool := '{
        true
        false
    }
    elegant := bool'true
    elegant := 'true
]
```

The latter one will search for the usable data constructor within the field. If more than one exists, lala will panic.

Type constructors can take type variables:

```lala
data := [
    option 'a := '{
        some 'a
        none
    }
    head  := 'some 0
    brain := 'none
]
```

...

## Package Manager

Dada.

Defines how "global" data blocks within lala's domain maps to reality, i.e. file system or repositories.
