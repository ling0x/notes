---
title: Async Futures
---

These two signatures are functionally equivalent — `async fn` is just syntactic
sugar that the compiler desugars into the explicit `impl Future` form.

```rust
trait T {
  async fn f(&self) -> u32;
}
```

```rust
trait T {
  fn f(&self) -> impl Future<Output = u32>;
}
```

### Question: whats the different between:

```rust
fn create_user(user: NewUser, pool: &Pool<Postgres>) -> impl std::future::Future<Output = sqlx::Result<User>> + Send;
```

and

```rust
async fn create_user(user: NewUser, pool: &Pool<Postgres>) -> sqlx::Result<User>;
```

## What the compiler does

When you write:

```rust
async fn create_user(user: NewUser, pool: &Pool<Postgres>) -> sqlx::Result<User>
```

The compiler automatically rewrites it to something like:
(rust-lang)[https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html]

```rust
fn create_user(user: NewUser, pool: &Pool<Postgres>) -> impl Future<Output = sqlx::Result<User>> + '_
```

So the first signature in your example is just the explicit version of what
async fn does implicitly.

The + Send detail The most important practical difference in your specific
example is the explicit `+ Send` bound on the first signature. This guarantees
the returned future is safe to send across threads, which is required when
spawning tasks with `tokio::spawn`. With `async fn`, whether the future is
`Send` is inferred from the function body — if any non-`Send` type is held
across an .await` point, the compiler will reject it without giving you an
explicit contract.
[rust-lang](https://rust-lang.github.io/async-book/part-guide/more-async-await.html)
​

So the explicit form is useful when you're writing trait objects, function
pointers, or want to enforce `Send` as part of a public API contract, while
`async
fn` is preferred for everyday use.
