# Big `S` &mdash; Rust's missing `String` literal

Hey. Sometimes we need a `String` but all we have is a string literal (an
`&'static str`). I mean, I think we've all seen eye-searing code filled with
`"this".to_string()`, `"that".to_owned()`, or `String::from("theother")`.

OMG somebody just do something about it.

Introducing `S`, the three-char solution to Rust's stupidest papercut.

Check it out:

```rust
do_something_lame("this".to_string(), "that".to_string(), "theother".to_string());
```

🙄

```rust
use big_s::S;

do_something_rad(S("this"), S("that"), S("theother"));
```

👍

## License

MIT/Apache-2.0/CC0-1.0
