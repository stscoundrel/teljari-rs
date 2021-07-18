# Teljari

Print arrays with commas and conjunctions for human consumption.

`join(',')` might bring you far when printing contents for humans. Yet you often want the last entry not be preceded by comma, but by the word 'and' or maybe 'or'. Teljari does this for you.

Rust version of original JS/TS lib.

### Install

Add this to your `Cargo.toml`:

```toml
[dependencies]
teljari = "1.0.0"
```

### Usage


```rust
use teljari::join_with_conj;

let list: Vec<&str> = vec!("Me", "Myself", "I");
let conj: &str = "and";
let result = join_with_conj(&list, &conj);


println!!("{}", result); // Me, Myself and I

```

#### What's in the name?

"Teljari" comes from Old Norse verb for "telja" (to count, tell, tally). It has same origins as English verb "tell". Verb + ari structure, which makes it a noun, "someone who tells". Teljari tells what's in your array as a human would want to hear it.
