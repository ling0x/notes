---
title: "Two Sum"
---

# Two Sum

**LeetCode #1 · Easy · Array / Hash Map**

## Problem

Given an array of integers `nums` and an integer `target`, return the indices of
the two numbers that add up to `target`. Each input has exactly one solution and
you may not use the same element twice.

```
Input:  nums = [2, 7, 11, 15], target = 9
Output: [0, 1]  // nums[0] + nums[1] = 2 + 7 = 9
```

## Approach

A brute-force nested loop would be O(n²). The classic O(n) solution uses a hash
map to store `value → index` as you iterate. For each element, check whether
`target - element` is already in the map. If it is, you have your pair.

The key insight is that you only need to look *backwards*: if `a + b = target`
and you encounter `b` second, `a` must already be in the map.

## Complexity

| | |
|---|---|
| Time | O(n) — single pass |
| Space | O(n) — map stores up to n entries |

## Implementation

```rust
use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen: HashMap<i32, usize> = HashMap::new();

    for (i, &val) in nums.iter().enumerate() {
        let complement = target - val;

        if let Some(&j) = seen.get(&complement) {
            return vec![j as i32, i as i32];
        }

        seen.insert(val, i);
    }

    // Problem guarantees exactly one solution, so this is unreachable.
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn example_2() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn same_element_twice() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
```

## Notes

- `HashMap::get` returns `Option<&V>`, so `if let Some(&j)` de-references the
  stored `usize` in one step.
- Inserting *after* the lookup means a number cannot match itself (e.g. if
  `target = 6` and `nums[0] = 3`, the map is empty when we first see `3`, so
  it won't falsely pair with itself).
- The problem promises exactly one answer, so `unreachable!()` is the idiomatic
  way to satisfy the compiler without returning a dummy value.
