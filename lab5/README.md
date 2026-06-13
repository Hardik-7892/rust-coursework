# Bracket Validator (Rust)

A simple command-line tool that checks whether a file contains properly balanced brackets using a stack-based approach.

It validates:

* Parentheses `()`
* Curly braces `{}`
* Square brackets `[]`

---

## Idea

The program treats the input as a stream of characters and uses a **stack** to ensure that every opening bracket has a matching closing bracket in the correct order.

This is a classic stack problem often used to demonstrate parsing and syntax validation.

---

## How to Run

```bash id="run1"
cargo run -- brackets.txt
```

The program expects a file path as a command-line argument.

---

## Valid Input Example

```text id="ex1"
({[]})
```

Output:

```text id="out1"
VALID
```

---

## Invalid Input Example

```text id="ex2"
({[}])
```

Output:

```text id="out2"
INVALID
```

---

## How It Works

### 1. Stack-Based Validation

A vector is used as a stack:

```rust id="stack1"
let mut stack: Vec<char> = Vec::new();
```

* Opening brackets are pushed onto the stack
* Closing brackets attempt to pop and match the most recent opening bracket

---

### 2. Character Processing

Each character is processed using:

```rust id="match1"
check_brackets(&mut stack, ch)
```

The function handles:

| Character   | Action                 |
| ----------- | ---------------------- |
| `(` `{` `[` | Push to stack          |
| `)` `}` `]` | Pop and validate match |
| `\n` `\r`   | Ignored                |
| Others      | Immediately invalid    |

---

### 3. Validation Function

Core logic:

```rust id="func1"
fn check_brackets(stack: &mut Vec<char>, ch: char) -> &'static str
```

It returns:

* `"VALID"` → if the current state is still correct
* `"INVALID"` → if a mismatch is detected

---

### 4. Early Exit Optimization

If an invalid state is detected:

```rust id="exit1"
if res == "INVALID" {
    break;
}
```

The program stops processing immediately instead of scanning the entire file.

---

### 5. Final Check

Even if no mismatch is found during iteration, the stack must be empty at the end:

```rust id="final1"
if stack.is_empty() {
    println!("{}", res);
} else {
    println!("INVALID");
}
```

This ensures all opening brackets were properly closed.

---

## Edge Cases Handled

* Extra closing brackets → INVALID
* Missing closing brackets → INVALID
* Wrong nesting order → INVALID
* Empty input → VALID
* Newlines ignored

---

## Limitations

* Returns only `"VALID"` or `"INVALID"` (no error details)
* Stops at first detected error
* Ignores all non-bracket characters (even if unexpected)
* Uses string literals instead of enum-based state handling

---

## Possible Improvements

* Replace `"VALID"/"INVALID"` with `enum Result`
* Track and report exact error position
* Support detailed error messages (expected vs found)
* Add CLI validation (missing file argument handling)
* Extend to HTML/XML tag validation
* Improve architecture by separating parser and validator logic

---

## Summary

This project demonstrates:

* Stack-based parsing
* Character-by-character file processing
* Basic compiler-style validation logic
* Early-exit optimization for efficiency
* Safe file handling in Rust
