# Rust Coursework

A collection of Rust exercises exploring core systems programming concepts, including file I/O, parsing, concurrency, CLI design, and data structures.

This repository is organized as a set of small, independent programs grouped under a single workspace-style folder.

---

## Key Concepts Covered

Across all projects, this repository demonstrates:

### Core Rust Fundamentals

* Ownership and borrowing
* Structs and enums
* Pattern matching
* Error handling basics

### Data Structures

* Vectors as dynamic arrays
* Stack-based parsing
* Tape-like memory models

### File I/O

* Reading input files using `std::fs`
* Path handling with `std::path`

### Concurrency

* Threads (`std::thread`)
* Message passing (`std::sync::mpsc`)

### CLI Development

* Argument parsing with `clap`
* Flags, options, and defaults

### Unicode Handling

* Emoji parsing
* Multi-codepoint character handling

---

## How to Run Any Project

Navigate into a project folder:

```bash
cd rust-coursework/<project-name>
```

Then run:

```bash
cargo run -- <args-if-needed>
```

---

## Requirements

* Rust (latest stable recommended)
* Cargo package manager

Check installation:

```bash
rustc --version
cargo --version
```

---

## Purpose

This repository consists of Rust exercises from the coursework covering:

* Systems programming fundamentals
* Parsing and interpreters
* CLI tools
* Concurrency models
* Practical Rust patterns

---

## Notes

Each project is intentionally minimal and focuses on one concept at a time rather than production-level robustness.

---

## License

For educational use.
