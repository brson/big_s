//! Big `S` &mdash; Rust's missing `String` literal
//!
//! Hey. Sometimes we need a `String` but all we have is a string literal (an
//! `&'static str`). I mean, I think we've all seen eye-searing code filled with
//! `"this".to_string()`, `"that".to_owned()`, or `String::from("theother")`.
//!
//! OMG somebody just do something about it.
//!
//! Introducing `S`, the three-char solution to Rust's stupidest papercut.
//!
//! Check it out:
//!
//! ```rust
//! # fn do_something_lame(x: String, y: String, z: String) { }
//! do_something_lame("this".to_string(), "that".to_string(), "theother".to_string());
//! ```
//!
//! 🙄
//!
//! ```rust
//! # fn do_something_rad(x: String, y: String, z: String) { }
//! use big_s::S;
//!
//! // RADICAL!
//! do_something_rad(S("this"), S("that"), S("theother"));
//! ```
//!
//! 👍
//!

#![allow(non_snake_case)]

/// Turn an `&'static str` into a `String`
#[inline]
pub fn S(s: &'static str) -> String {
    s.to_string()
}

#[cfg(test)]
mod tests {
    use super::S;

    #[test]
    fn it_works() {
        assert!(S("foo") == "foo".to_string());
    }
}
