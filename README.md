# churchill

A simple untyped lambda calculus interpreter written in Rust. 
Supports parsing lambda expressions, normal-order reduction, named definitions and an interactive REPL or batch processing of definition files.

> **Version:** 0.1.0
> **Edition:** Rust 2021

## Features

- Parsing of lambda expressions with **λ** or `\\` notation
- Alpha-conversion to avoid variable capture
- Substitution and normal-order reduction (normal-order, with a high step limit)
- Named definitions, loaded from a file or defined interactively
- Batch mode: process `.lam` files, skip comments (`#`), and evaluate expressions
- Interactive REPL with comment support and `exit`/`quit` commands
- Built-in examples in `examples.churchill`: Church numerals, booleans, arithmetic, recursion, lists

## Prerequisites

- Rust and Cargo (edition 2021 or later)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Building

From the project root, build the release binary:

```bash
cargo build --release
```

This produces the executable at `target/release/churchill`.

## Usage

### One-shot evaluation

Evaluate a lambda expression directly:

```bash
cargo run -- '(\x.x) y'
# or, if installed:
churchill '(\x.x) y'
```

Output:
```text
y
```

### Batch mode

Process a file with definitions and expressions:

```bash
cargo run -- --file examples.churchill
# or, if installed:
churchill --file examples.churchill
```

This will load definitions, skip comments (lines starting with `#`), and print the normal form of each expression.

### Interactive REPL

Start an interactive session:

```bash
cargo run
# or installed:
churchill
```

Type lambda expressions or definitions:

```text
> id = \x.x
Defined id = (λx.x)
> id y
y
> exit
```

Comments can be added with `#`:

```text
> \x.x  # identity
(λx.x)
```

## Examples

See `examples.churchill` for predefined Church numerals, arithmetic, booleans, recursion (factorial), lists, and sample expressions.

## Testing

Run the unit tests:

```bash
cargo test
```
