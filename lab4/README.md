# Emoji Tape Interpreter (Rust)

A minimalist emoji-based tape interpreter written in Rust.
It reads a file containing emoji instructions and simulates a dynamic integer tape similar to a simplified Brainf*ck-style model.

---

## Features

* Dynamic tape that grows as needed
* Pointer-based navigation across cells
* Increment/decrement operations on cells
* UTF-8 emoji parsing with lookahead handling
* Automatic expansion when moving right

---

## Instructions Set

| Emoji                | Meaning                |
| -------------------- | ---------------------- |
| 😀 (`U+1F600`)       | Increment current cell |
| 🙁 (`U+1F641`)       | Decrement current cell |
| ➡️ (`U+27A1 U+FE0F`) | Move pointer right     |
| ⬅️ (`U+2B05 U+FE0F`) | Move pointer left      |

---

## How It Works

* A vector `v: Vec<i32>` represents the tape, starting with one cell initialized to `0`.
* A pointer `i` tracks the current cell index.
* The program reads the input file as a stream of Unicode characters.
* It uses a `peekable()` iterator to properly handle multi-codepoint emojis (like ➡️ and ⬅️ which include variation selectors).

### Core Behavior

* 😀 increases the current cell value
* 🙁 decreases the current cell value
* ➡️ moves right and expands the tape if needed
* ⬅️ moves left (no bounds protection included)
* All other characters are ignored

---

## Input

The program expects a file path as a command-line argument:

```bash id="q7xk2a"
cargo run -- moresmiles.txt
```

Example input file:

```text id="p1m8zr"
😀😀➡️😀🙁➡️😀
```

---

## Output

The program prints the tape state up to the last non-zero cell:

```text id="v3n8sd"
0 : 2
1 : 1
2 : 1
```

Each line shows:

```bash
index : value
```

---

## Implementation Notes

### 1. Tape Initialization

```rust
let mut v: Vec<i32> = Vec::new();
v.push(0);
```

A single-cell tape is created at startup.

---

### 2. Pointer Movement

Right movement:

* Checks if next cell exists
* Expands vector if needed

Left movement:

* Simply decrements index (no underflow protection)

---

### 3. Emoji Handling

Because some emojis are composed of multiple Unicode scalars (e.g. `➡️`), the program uses:

```rust
iter.peekable()
```

to ensure correct parsing of variation selectors like `U+FE0F`.

---

### 4. Tape Output Logic

The program determines the highest non-zero index:

```rust
let mut highest_non_zero_index = 0;
```

Then prints only meaningful portion of the tape.

---

## Limitations

* No bounds checking on left movement (can panic if used incorrectly)
* No input validation for missing arguments
* Tape only stores `i32` values
* No error recovery for malformed emoji sequences

---

## Possible Improvements

* Add safe bounds handling for pointer movement
* Support loops / control flow (Brainf*ck-style)
* Add command validation errors
* Replace vector with a growable deque for safer left/right movement
* Add debug mode to visualize tape evolution step-by-step

---

## Summary

This project demonstrates:

* Unicode-aware parsing in Rust
* Manual state machine design
* Tape-based computation model
* Handling multi-codepoint emoji sequences safely using iterators
