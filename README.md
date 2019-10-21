# Introduction

# Links

-   [Clap.rs](https://clap.rs)

# Learnings

## Return Values

-   Forgot the return type (`-> u64`)

-   I was trying to return using a statement: `n;` vs `n`

> Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value.

```
error[E0308]: mismatched types
 --> src/main.rs:4:26
  |
4 | fn next_prime(x: u64) -> u64 {
  |    ----------            ^^^ expected u64, found ()
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
...
7 |     n;
  |      - help: consider removing this semicolon
  |
  = note: expected type `u64`
             found type `()`
```