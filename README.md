# Next Prime CLI

## Usage

Build the program and run it like a normal CLI:

```
cargo build
./target/debug/next-prime-rs -h
./target/debug/next-prime-rs -V
./target/debug/next-prime-rs 1
```

## Crates You Should Know

-   [Clap](https://crates.io/crates/clap) - Command Line Argument Parser for Rust

-   [Primes](https://crates.io/crates/primes) - A prime generator for Rust.

## Learnings

### Return Values

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
