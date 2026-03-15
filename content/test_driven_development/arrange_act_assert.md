---
title: Arrange-Act-Assert
---

## The Pattern

Arrange-Act-Assert is a great way to structure test cases. It prescribes an
order of operations:

1. **Arrange** inputs and targets. Arrange steps should set up the test case.
   Does the test require any objects or special settings? Does it need to prep a
   database? Does it need to log into a web app? Handle all of these operations
   at the start of the test.

2. **Act** on the target behavior. Act steps should cover the main thing to be
   tested. This could be calling a function or method, calling a REST API, or
   interacting with a web page. Keep actions focused on the target behavior.

3. **Assert** expected outcomes. Act steps should elicit some sort of response.
   Assert steps verify the goodness or badness of that response. Sometimes,
   assertions are as simple as checking numeric or string values. Other times,
   they may require checking multiple facets of a system. Assertions will
   ultimately determine if the test passes or fails.

**Behavior-Driven Development** follows the **Arrange-Act-Assert** pattern by
another name: **Given-When-Then**. The Gherkin language uses Given-When-Then
steps to specify behaviors in scenarios. Given-When-Then is essentially the same
formula as Arrange-Act-Assert.

### References

[automationpanda](https://automationpanda.com/2020/07/07/arrange-act-assert-a-pattern-for-writing-good-tests/)
[semaphore](https://semaphore.io/blog/aaa-pattern-test-automation)

## Example

Here is a simple Rust example demonstrating the Arrange-Act-Assert pattern using
a small `ShoppingCart` struct. Rust's built-in `#[cfg(test)]` module makes the
three phases very natural to express.

The code:

```rust
// src/lib.rs

pub struct ShoppingCart {
    items: Vec<(String, f64)>, // (name, price)
}

impl ShoppingCart {
    pub fn new() -> Self {
        ShoppingCart { items: vec![] }
    }

    pub fn add_item(&mut self, name: &str, price: f64) {
        self.items.push((name.to_string(), price));
    }

    pub fn total(&self) -> f64 {
        self.items.iter().map(|(_, price)| price).sum()
    }

    pub fn item_count(&self) -> usize {
        self.items.len()
    }
}
```

The test:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_reflects_added_items() {
        // ── Arrange ──────────────────────────────────────
        let mut cart = ShoppingCart::new();
        let expected_total = 17.97;

        // ── Act ──────────────────────────────────────────
        cart.add_item("Apple",  0.99);
        cart.add_item("Bread",  2.49);
        cart.add_item("Laptop", 14.49);

        // ── Assert ───────────────────────────────────────
        assert!((cart.total() - expected_total).abs() < f64::EPSILON);
        assert_eq!(cart.item_count(), 3);
    }

    #[test]
    fn test_empty_cart_has_zero_total() {
        // ── Arrange ──────────────────────────────────────
        let cart = ShoppingCart::new();

        // ── Act ──────────────────────────────────────────
        let total = cart.total();

        // ── Assert ───────────────────────────────────────
        assert_eq!(total, 0.0);
    }
}
```

Flow Diagram:

```
┌─────────────────────────────────────────────────────┐
│                   TEST FUNCTION                     │
│                                                     │
│  ┌─────────────────────────────────────────────┐    │
│  │  ARRANGE                                    │    │
│  │  • Create ShoppingCart::new()               │    │
│  │  • Define expected_total = 17.97            │    │
│  └──────────────────┬──────────────────────────┘    │
│                     │                               │
│                     ▼                               │
│  ┌─────────────────────────────────────────────┐    │
│  │  ACT                                        │    │
│  │  • cart.add_item("Apple",  0.99)            │    │
│  │  • cart.add_item("Bread",  2.49)            │    │
│  │  • cart.add_item("Laptop", 14.49)           │    │
│  └──────────────────┬──────────────────────────┘    │
│                     │                               │
│                     ▼                               │
│  ┌─────────────────────────────────────────────┐    │
│  │  ASSERT                                     │    │
│  │  • cart.total() ≈ 17.97  ✔ / ✘              │    │
│  │  • cart.item_count() == 3  ✔ / ✘            │    │
│  └─────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────┘
```

Rust-specific notes:

- `#[cfg(test)]` gates the test module so it's only compiled during cargo test,
  keeping production binaries lean. ​

- Floating-point assertions in Rust require a tolerance check
  (`abs() < EPSILON`) rather than a direct `==`, since `f64` arithmetic can
  introduce tiny rounding errors. ​

- Each test function is independent — no shared mutable state bleeds between
  them, reinforcing the one behavior per test principle of AAA. ​

- The **Given-When-Then** equivalent here would be: Given an empty cart, When
  three items are added, Then the total equals their sum. ​
