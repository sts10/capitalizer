# Capitalizer (Double Mount Bug Demo)

This repo's sole purpose is to demonstrate a small issue with Rust's compiler warnings. 

## The warning we get

If you clone down this repo and run `cargo build` (using rustc 1.33.0 stable), you should get this warning:

```text
Compiling capitalizer v0.1.0 (/home/sschlinkert/code/capitalizer)
warning: function is never used: `downcase_this`
--> src/downcase.rs:1:1
|
1 | pub fn downcase_this(s: String) -> String {
| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
|
= note: #[warn(dead_code)] on by default

 Finished dev [unoptimized + debuginfo] target(s) in 0.37s
```

What's strange about this is that that function, `downcase_this`, _is_ used, both in `src/lib.rs` and in `src/main.rs`. 

## A guess at why we get this warning

The `downcase` module is "mounted" in both `main.rs` and in `lib.rs`. Strangely, only one _version_ on `downcase` is used in both `main.rs` and `lib.rs`, and the other version is unused.

Calling `mod downcase` and `use capitalizer::downcase::downcase_this` in `main.rs`, while calling `use crate::downcase::downcase_this` in `lib.rs` seems to cause the issue. 

As [Graydon explained to me](https://octodon.social/@graydon/102069460491920758) when I showed him a different project experiencing a similar issue: 

> What's going on here is cargo is seeing lib.rs and main.rs and compiling _two_ crates (one library, one executable that uses the library) but you're _also_ mounting entries.rs as a sub module of main.rs. One you don't use directly.

> It's only giving you the warning when compiling the second (executable == main.rs) crate. It just does them both back to back and you get the warnings from both.

Thus I'm, at least informally, going to refer to this as "double mounting."

## Ways to avoid this warning

1. If we have `use crate::downcase::downcase_this` in both main.rs and lib.rs, we don't get the warning.

2. If you remove `mod downcase` from `main.rs`, the warning also goes away. So:

```rust
// main.rs
use capitalizer::downcase::downcase_this;
```

```rust
// lib.rs
pub mod downcase;
use crate::downcase::downcase_this;
```

## A guess at what the warning should say

Ideally the compiler would give a more accurate warning here, explaining to the developer they've "double mounted" a module or function from a module. 

However I'm not sure if it's a bad thing to "double mount". If it's not a bad thing, maybe no warning should be given at all. 

Conversely, there may be real issues with double-mounting that I'm unable to understand.

## More about my system

```text
> rustc --version
rustc 1.33.0 (2aa4c46cf 2019-02-28)

> cargo --version
cargo 1.33.0 (f099fe94b 2019-02-12)

> uname -a
Linux sschlinkert 4.18.0-18-generic #19~18.04.1-Ubuntu SMP Fri Apr 5 10:22:13 UTC 2019 x86_64 x86_64 x86_64 GNU/Linux
```
