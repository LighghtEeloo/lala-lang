# Nana

An expression oriented programming language / data notation, designed for elegant data abstraction.

This is a proof-of-concept sub-language derived from Lala (or as a prelude).

## Expression

All terms in `nana` will eventually converge to an expression, so it's fair to start from. Here is a list of all forms of expressions. The list will be revisited as new terms are introduced.

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


// Todo...

## Cheatsheet




## Quick Samples

### Functional Programming in Nana

Quick Sort.

```nana
~ partition f xs := (
    ~ _part xs a b = (
        ? xs
        | [] -> (a, b)
        | [x] + xs -> (
            ~ (a, b) = _part xs a b;
            ?? (f x) 
            | (a + [x], b) 
            | (a, b + [x])
        )
    );
    _part xs [] []
);

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

Pi.

```nana
~ range n := (
    ? n
    | x ?? x > 0 -> (range (n - 1)) + [n - 1]
    | _ -> []
);

~ foldl f acc xs := (
    ? xs 
    | [x] + xs -> foldl f (f x acc) xs
    | [] -> acc
);

~ sum := foldl (+) 0;

~ map f xs := (
    ~ f' x acc = acc + (f x);
    foldl f' [] xs
);

~ (@) := map;

~ <pi'> := pi := [
    /* 1-1/3+1/5-... */
    ~ f x = 1 / (2 * x + 1) * ((x % 2) * 2 - 1);
    ~ x = sum (f @ (range 40));
    ~ rough := 4 * x;
    ~ lookup := 3.14159;
    ~ pi := rough;
];

~ all_pi = [pi', pi.rough, pi.lookup];
all_pi
```

### Object-Oriented Programming in Nana

```nana
~ student <name; sleep; ability; gpa> := [
    ~ study := (
        ?? sleep 
        | name + " don't want to study." 
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
    "study_status": "hcz don't want to study.",
    "exam_result": 100
}
```
