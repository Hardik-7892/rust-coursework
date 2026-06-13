# CLI Argument Parser (Rust + Clap)

A simple command-line application demonstrating structured argument parsing in Rust using the `clap` crate.

It showcases how to define flags, options, default values, and typed CLI inputs with minimal boilerplate.

---

## Features

* Typed CLI arguments using `clap::Parser`
* Default values for optional flags
* Short and long flag support
* Boolean switches
* String and numeric input handling

---

## Dependencies

Add `clap` to your project:

```bash id="dep1"
cargo add clap --features derive
```

---

## Run

```bash id="run1"
cargo run -- --help
```

Example execution:

```bash id="run2"
cargo run -- -s 200 -F myfile.txt -Q
```

---

## Command-Line Arguments

| Flag | Long Form | Type     | Default               | Description             |
| ---- | --------- | -------- | --------------------- | ----------------------- |
| `-s` | `--size`  | `u32`    | `100`                 | Sets memory/vector size |
| `-F` | `--file`  | `String` | `"No file attached!"` | Input file name         |
| `-Q` | `--quiet` | `bool`   | `false`               | Enables quiet mode      |

---

## Example Usage

### 1. Default Run

```bash id="ex1"
cargo run
```

Output:

```text id="out1"
memory vector usage: 100
AAAAAHHHHH!!!
No file attached!
```

---

### 2. Custom Size and File

```bash id="ex2"
cargo run -- -s 250 -F data.txt
```

Output:

```text id="out2"
memory vector usage: 250
AAAAAHHHHH!!!
File name is data.txt
```

---

### 3. Quiet Mode

```bash id="ex3"
cargo run -- -Q
```

Output:

```text id="out3"
memory vector usage: 100
shush
No file attached!
```

---

## Code Structure

### 1. Argument Definition

Arguments are defined using a struct with derive macros:

```rust id="struct1"
#[derive(Parser, Debug)]
struct Args {
```

This enables automatic CLI parsing.

---

### 2. Field Annotations

Each field uses `#[arg(...)]` to define CLI behavior:

* `short`: single-letter flag
* `long`: full flag name
* `default_value_t`: typed default value

Example:

```rust id="field1"
#[arg(short, long, default_value_t = 100)]
size: u32,
```

---

### 3. Parsing CLI Input

Arguments are parsed automatically:

```rust id="parse1"
let args = Args::parse();
```

---

### 4. Runtime Behavior

The program prints based on parsed values:

* Always prints `size`
* Prints either `"shush"` or `"AAAAAHHHHH!!!"` based on `quiet`
* Prints file name only if it differs from default

---

## Behavior Notes

* If `--file` is not provided, a placeholder string is used:

  ```text
  No file attached!
  ```
  
* `--quiet` toggles output verbosity

* `--size` controls a numeric configuration value

---

## Limitations

* Uses a sentinel string instead of `Option<String>` for file handling
* No validation of file existence
* Default file logic relies on string comparison (not idiomatic Rust)
* Output is purely demonstrative (no real file processing)

---

## Improvements

* Replace `file: String` with `Option<String>`
* Add file existence checking
* Improve output formatting (structured logging or verbosity levels)
* Add subcommands using `clap::Subcommand`
* Integrate actual file processing logic
* Support environment variables for defaults

---

## What This Demonstrates

This project highlights:

* Rust CLI design using `clap`
* Declarative argument parsing
* Default values and type safety
* Handling flags vs options
* Basic CLI application structure

---

## Summary

A minimal but expressive example of building a CLI tool in Rust using modern derive-based argument parsing.

It serves as a foundation for more advanced command-line utilities.
