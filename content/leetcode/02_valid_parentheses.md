---
title: "Valid Parentheses"
---

# Valid Parentheses

**LeetCode #20 · Easy · Stack**

## Problem

Given a string containing only `(`, `)`, `{`, `}`, `[`, `]`, determine whether
the input string is valid. An input string is valid if:

1. Open brackets are closed by the same type of bracket.
2. Open brackets are closed in the correct order.
3. Every close bracket has a corresponding open bracket.

```
Input:  s = "({[]})"
Output: true

Input:  s = "(]"
Output: false
```

## Approach

A stack is the natural fit here. Push every opening bracket onto the stack. When
you see a closing bracket, check that the top of the stack is its matching
opener — if it is, pop; if it isn't (or the stack is empty), the string is
invalid. At the end the stack must be empty.

## Complexity

| | |
|---|---|
| Time | O(n) — single pass through the string |
| Space | O(n) — worst case all openers, e.g. `(((` |

## Implementation

```rust
pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for ch in s.chars() {
        match ch {
            '(' | '{' | '[' => stack.push(ch),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            _ => {} // problem guarantees only bracket chars, but be safe
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_valid() {
        assert!(is_valid("()[]{}".to_string()));
    }

    #[test]
    fn nested_valid() {
        assert!(is_valid("({[]})".to_string()));
    }

    #[test]
    fn mismatched() {
        assert!(!is_valid("(]".to_string()));
    }

    #[test]
    fn unclosed() {
        assert!(!is_valid("([".to_string()));
    }

    #[test]
    fn empty_string() {
        assert!(is_valid("".to_string()));
    }
}
```

## Notes

- `Vec::pop` returns `Option<char>`, which makes the match against `Some('(')`
  a clean one-liner — it handles both the "wrong bracket" and "empty stack"
  cases simultaneously.
- Returning `stack.is_empty()` at the end catches inputs like `"((("` where
  every opener is valid but nothing closes them.
