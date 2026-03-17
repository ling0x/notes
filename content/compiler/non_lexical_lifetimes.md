---
title: Non-lexical lifetimes (NLL)
---

Non-Lexical Lifetimes (NLL) is a significant improvement to Rust's borrow
checker, introduced in Rust 2018 and fully stabilised by default in Rust 1.63.
It changed how the compiler reasons about _how long a reference lives_.[^8]

## The Problem: Lexical Lifetimes (Before NLL)

Before NLL, a reference's lifetime lasted until the **end of its enclosing
scope** (the closing `}`), regardless of whether it was actually used after a
certain point. This caused the borrow checker to reject perfectly safe code:[^1]

```rust
let mut data = vec![1, 2, 3];
let first = &data[^0];     // immutable borrow starts here
println!("{}", first);    // last actual use of `first`

// OLD borrow checker: `first` still "alive" until `}`, so this fails ❌
// Even though `first` is never used again!
data.push(4);
```

## The Fix: NLL

With NLL, the compiler tracks the **last point where a reference is actually
used**, and ends its lifetime there — not at the closing brace. This is based on
analysing the **control-flow graph** of your code rather than just the syntactic
block structure.[^6][^1]

```rust
let mut data = vec![1, 2, 3];
let first = &data[^0];     // immutable borrow starts
println!("{}", first);    // ✅ last use — borrow ENDS HERE (NLL)

data.push(4);             // ✅ now safe to mutate
```

## Lexical vs. Non-Lexical Lifetimes

|                    | Lexical Lifetimes (Old)    | Non-Lexical Lifetimes (NLL)      |
| :----------------- | :------------------------- | :------------------------------- |
| Lifetime ends at   | Closing `}` of scope       | Last actual use of the reference |
| Based on           | Syntax tree (scope blocks) | Control-flow graph               |
| Rejects safe code? | Yes, in many common cases  | Much less often                  |
| Introduced         | Original Rust              | Rust 2018, stable in Rust 1.63   |

## NLL and Control Flow

NLL is smart enough to handle branching logic too. A borrow is only considered
"live" at a point if its value **could be used in the future** from that point.
This means in conditional branches where a reference is only used in one path,
it won't block mutations on the other path.[^7]

```rust
let mut s = String::from("hello");

let r = &s;
if some_condition {
    println!("{r}"); // r used here in one branch
}
// NLL knows r MAY still be live here, so mutation below could still fail
// depending on the control flow — the compiler reasons through each path
```

In short, NLL makes Rust's borrow checker significantly more ergonomic without
compromising any of its safety guarantees — it simply became smarter about
_when_ a borrow truly ends.[^1]
<span style="display:none">[^10][^2][^3][^4][^5][^9]</span>

<div align="center">⁂</div>

[^1]: https://oneuptime.com/blog/post/2026-01-25-non-lexical-lifetimes-rust/view

[^2]: https://users.rust-lang.org/t/about-non-lexical-lifetimes/111614

[^3]: https://www.reddit.com/r/rust/comments/wgxr9q/nonlexical_lifetimes_nll_fully_stable_rust_blog/

[^4]: https://stackoverflow.com/questions/50251487/what-are-non-lexical-lifetimes

[^5]: https://www.youtube.com/watch?v=XD-nc28-8Fw

[^6]: https://rust-lang.github.io/rfcs/2094-nll.html

[^7]: https://www.reddit.com/r/rust/comments/6brtsu/eli5_nonlexical_lifetimes/

[^8]: https://blog.rust-lang.org/2022/08/05/nll-by-default.html

[^9]: https://smallcultfollowing.com/babysteps/blog/2017/02/21/non-lexical-lifetimes-using-liveness-and-location/

[^10]: https://smallcultfollowing.com/babysteps/blog/2016/04/27/non-lexical-lifetimes-introduction/
