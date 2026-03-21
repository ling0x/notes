---
title: "Binary Search"
---

# Binary Search

**LeetCode #704 · Easy · Binary Search**

## Problem

Given a sorted array of distinct integers `nums` and a target value, return the
index of `target` if it exists, otherwise return `-1`. Your solution must run in
O(log n).

```
Input:  nums = [-1, 0, 3, 5, 9, 12], target = 9
Output: 4

Input:  nums = [-1, 0, 3, 5, 9, 12], target = 2
Output: -1
```

## Approach

Maintain an inclusive range `[lo, hi]`. On each iteration compute the midpoint
and compare `nums[mid]` against `target`:

- Equal → found, return `mid`.
- Less → target is in the right half, set `lo = mid + 1`.
- Greater → target is in the left half, set `hi = mid - 1`.

If `lo > hi` the target is absent.

### Midpoint arithmetic

Computing `mid = (lo + hi) / 2` can overflow when `lo` and `hi` are both large.
The safe form is `mid = lo + (hi - lo) / 2`.

## Complexity

| | |
|---|---|
| Time | O(log n) — halves the search space each iteration |
| Space | O(1) |

## Implementation

```rust
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut lo: usize = 0;
    let mut hi: usize = nums.len().saturating_sub(1);

    // Use a flag to handle the empty-array edge case cleanly.
    if nums.is_empty() {
        return -1;
    }

    loop {
        let mid = lo + (hi - lo) / 2;

        match nums[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return mid as i32,
            std::cmp::Ordering::Less => {
                if mid == hi {
                    break; // would overflow hi + 1 as usize
                }
                lo = mid + 1;
            }
            std::cmp::Ordering::Greater => {
                if mid == lo {
                    break; // would underflow lo - 1 as usize
                }
                hi = mid - 1;
            }
        }
    }

    -1
}

// Alternatively, using signed indices avoids the underflow edge case:
pub fn search_signed(nums: Vec<i32>, target: i32) -> i32 {
    let (mut lo, mut hi) = (0_i32, nums.len() as i32 - 1);

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        match nums[mid as usize].cmp(&target) {
            std::cmp::Ordering::Equal => return mid,
            std::cmp::Ordering::Less => lo = mid + 1,
            std::cmp::Ordering::Greater => hi = mid - 1,
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn found() {
        assert_eq!(search_signed(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }

    #[test]
    fn not_found() {
        assert_eq!(search_signed(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }

    #[test]
    fn single_element_match() {
        assert_eq!(search_signed(vec![5], 5), 0);
    }

    #[test]
    fn empty() {
        assert_eq!(search_signed(vec![], 5), -1);
    }
}
```

## Notes

- Signed indices (`i32`) are simpler in Rust because `lo <= hi` with unsigned
  types can panic or wrap when `hi` would go below zero — a very common bug.
- `nums[mid].cmp(&target)` with a `match` on `Ordering` is more idiomatic than
  chained `if/else if` comparisons and exhaustive by design.
- Binary search is the foundation of many harder problems: search in rotated
  sorted array (#33), find minimum in rotated array (#153), and time-based
  key-value stores.
