# Emoji Mood Analyzer

A simple Rust CLI program that reads a text file, counts happy (`😀`) and sad (`🙁`) emojis, and prints:

* `HAPPY` if happy emojis are more frequent
* `SAD` if sad emojis are more frequent
* `OK` if the counts are equal

## Run

```bash
cargo run -- smiles.txt
```

## Example

Given `smiles.txt`:

```text
😀😀🙁
```

Output:

```text
HAPPY
```
