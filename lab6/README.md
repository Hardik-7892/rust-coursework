# Emoji Tape Interpreter (Multithreaded Version)

A Rust-based emoji interpreter that simulates a tape of integers, upgraded to use **message passing between threads** for processing instructions.

This version builds on a previous single-threaded interpreter by introducing concurrency via channels.

---

## Overview

The program:

* Reads emoji instructions from a file
* Spawns a worker thread to parse and interpret input
* Sends structured messages to the main thread via an `mpsc` channel
* Updates a shared tape (`Vec<i32>`) safely in the main thread

---

## Run

```bash id="run1"
cargo run -- moresmiles.txt
```

---

## Emoji Instruction Set

| Emoji | Meaning                |
| ----- | ---------------------- |
| 😀    | Increment current cell |
| 🙁    | Decrement current cell |
| ➡️    | Move pointer right     |
| ⬅️    | Move pointer left      |

---

## Architecture

### Threaded Design

This version separates concerns into two parts:

### 1. Producer Thread (Parser)

* Reads the input file character-by-character
* Tracks:

  * current pointer index `i`
  * tape length
* Sends structured messages to the main thread

### 2. Main Thread (Executor)

* Receives `UpdateMsg` messages
* Mutates the tape accordingly
* Prints final state

---

## Message Passing System

Communication is done using:

```rust id="mpsc1"
let (tx, rx) = mpsc::channel();
```

### Message Types

```rust id="enum1"
enum UpdateMsg {
    IncrementCell { i: usize },
    DecrementCell { i: usize },
    NewCell,
    Error,
}
```

| Message         | Effect                        |
| --------------- | ----------------------------- |
| `IncrementCell` | Increase value at index `i`   |
| `DecrementCell` | Decrease value at index `i`   |
| `NewCell`       | Append a new cell to the tape |
| `Error`         | Flag an error state           |

---

## How It Works

### 1. Input Processing (Threaded)

Inside the spawned thread:

```rust id="thread1"
let handle = thread::spawn(move || {
```

The thread:

* Iterates over characters using `peekable()`
* Detects multi-codepoint emojis (➡️, ⬅️)
* Tracks pointer position (`i`)
* Sends messages via `tx.send(...)`

---

### 2. Tape Updates (Main Thread)

The main thread listens:

```rust id="recv1"
for msg in rx {
```

Each message updates the tape:

* Increment/decrement modifies `v[i]`
* `NewCell` expands the vector
* `Error` sets a failure flag

---

### 3. Concurrency Model

This is a **producer–consumer architecture**:

```bash
[ Parser Thread ]  --->  (mpsc channel)  --->  [ Main Thread ]
```

* Producer: parses emoji instructions
* Consumer: mutates shared state safely

No shared mutable state between threads → avoids data races.

---

## Tape Representation

```rust id="tape1"
let mut v: Vec<i32> = Vec::new();
v.push(0);
```

* Each index is a memory cell
* Pointer `i` moves across cells
* Vector grows dynamically when needed

---

## Output Behavior

After processing:

* The program prints all cells up to the last non-zero value
* Format:

```text id="out1"
index : value
```

Example:

```text id="out2"
0 : 3
1 : 1
2 : 0
```

---

## Error Handling

* An `Error` message can be triggered by future extensions
* If triggered, the program prints:

```text id="err1"
ERROR
```

Currently, errors are defined but not actively emitted in logic.

---

## Limitations

* No bounds checking on left movement (`i -= 1`)
* `Error` message is unused in current parsing logic
* Thread does not terminate early on invalid input
* Pointer and tape logic still partially duplicated across threads
* No synchronization primitives beyond channel messaging

---

## Possible Improvements

* Add safe bounds checks for pointer movement
* Emit `Error` for invalid operations (e.g., underflow)
* Replace `Vec<i32>` with a safer tape abstraction
* Add instruction enums instead of raw emojis in parsing layer
* Introduce unit tests for interpreter correctness
* Add visualization of tape state over time
* Support multiple worker threads for parallel parsing (advanced)

---

## What This Demonstrates

This project showcases:

* Rust threading (`std::thread`)
* Message passing concurrency (`std::sync::mpsc`)
* Safe shared-state design without locks
* Parsing Unicode emoji streams
* Building interpreter-like systems in Rust
* Separation of parsing and execution stages

---

## Summary

This is a step toward a more structured interpreter design:

* Single-threaded → multithreaded architecture
* Direct mutation → message-based updates
* Linear processing → staged pipeline model
