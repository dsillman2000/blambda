# blambda

Boolean lambda calculus engine, written in Rust using [pest](https://pest.rs/) and [clap](https://github.com/clap-rs/clap)

Provides a simple CLI, `blambda`, which can be used to parse a simple boolean lambda calculus specified below:

| Token Type         | Alternatives                                                                                                    |
| ------------------ | --------------------------------------------------------------------------------------------------------------- |
| Value              | "t" or "f" (case-insensitive)                                                                                   |
| Prefix operator(s) | "~ expr" (logical not)                                                                                          |
| Infix operator(s)  | "expr \| expr" (logical or)<br>"expr & expr" (logical and)<br>"expr ? (expr : expr)" (logical ternary operator) |

The AST of a set of expressions can be determined using

```sh
blambda parse -s "t | f"

# exprs:
# - op: or
#   arg1: true
#   arg2: false
```

Likewise, the truth values of a set of expressions can be encoded as (little-endian) bits and be returned as an unsigned integer using

```sh
blambda eval -s "(f | f | t) (t ? t : t) f"

# 6
```

Moreover, a blambda program can be formatted using the `format` comand, which will return a formatted representation of the blambda program.

```sh
blambda format -s "t ? f : t | t & f f | t"

# (t ? (f : ((t | t) & f))) (f | t)
```

All of these commands can be used without the `-s` flag to read from a filepath instead.

### High-quality parse error handling

Thanks to pest, `blambda` is able to provide explanatory error messages whenever a parsing error is encountered, e.g.

```sh
blambda parse -s "t inv"

# BlambdaError:  --> 1:3
#   |
# 1 | t inv
#   |   ^---
#   |
#   = expected EOI, expr, or, and, condition, or branch
```

### Evaluation halting

Of course, some `blambda` expressions cannot be evaluated according to the logic of the lambda calculus, even though it may be parseable. In this case,
we don't take advantage  of pest's fancy error reporting, so we simply throw an error when the evaluator fails to pattern-match an expression.

As an example, consider the parseable expression:

```sh
blambda parse -s "t : t : f"

# exprs:
# - op: branch
#   arg1:
#     op: branch
#     arg1: true
#     arg2: true
#   arg2: false
```

Although its syntax is parseable in the (somewhat loose) AST for `blambda`, trying to evaluate it under the evaluation rules for the boolean lambda calculus
will not succeed:

```sh
blambda eval -s "t : t : f"

# Error: program could not be evaluated
```
