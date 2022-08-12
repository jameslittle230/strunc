# Strunc

A string truncating crate.

```rust
let output = "very long string".strunc_len(10);
assert_eq!("very lo...", output.to_string());
```

Different options:

```rust
// Default:
"very long string is very long".strunc(); // truncates to 25 characters

// Customize the suffix:
let output = "very long string".strunc_len_suf(15, "....");
assert_eq!("very long s....", output);
```
