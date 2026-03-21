---
title: "Maximum Subarray"
---

# Maximum Subarray

**LeetCode #53 · Medium · Dynamic Programming / Kadane's Algorithm**

## Problem

Given an integer array `nums`, find the subarray with the largest sum and return
that sum. A subarray is a contiguous non-empty portion of the array.

```
Input:  nums = [-2, 1, -3, 4, -1, 2, 1, -5, 4]
Output: 6   // subarray [4, -1, 2, 1]

Input:  nums = [1]
Output: 1

Input:  nums = [5, 4, -1, 7, 8]
Output: 23  // entire array
```

## Approach: Kadane's Algorithm

Track two variables:

- `current_sum` — the maximum sum of a subarray *ending at the current index*.
- `best` — the global maximum seen so far.

At each element `x`:

```
current_sum = max(x, current_sum + x)
best        = max(best, current_sum)
```

The first `max` answers: "is it better to start a fresh subarray here, or
extend the previous one?" If `current_sum` went negative, it's always better to
start fresh.

## Complexity

| | |
|---|---|
| Time | O(n) — single pass |
| Space | O(1) |

## Implementation

```rust
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    // Start with the first element to handle all-negative arrays correctly.
    let mut current_sum = nums[0];
    let mut best = nums[0];

    for &x in &nums[1..] {
        // Extend or restart.
        current_sum = x.max(current_sum + x);
        best = best.max(current_sum);
    }

    best
}

// Variant: also return the subarray bounds (useful to know in practice).
pub fn max_sub_array_with_indices(nums: &[i32]) -> (i32, usize, usize) {
    let mut best = nums[0];
    let mut current_sum = nums[0];
    let mut start = 0usize;
    let mut best_start = 0usize;
    let mut best_end = 0usize;

    for (i, &x) in nums[1..].iter().enumerate() {
        let i = i + 1; // offset for the slice
        if current_sum + x < x {
            current_sum = x;
            start = i;
        } else {
            current_sum += x;
        }
        if current_sum > best {
            best = current_sum;
            best_start = start;
            best_end = i;
        }
    }

    (best, best_start, best_end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixed() {
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    }

    #[test]
    fn single() {
        assert_eq!(max_sub_array(vec![1]), 1);
    }

    #[test]
    fn all_positive() {
        assert_eq!(max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }

    #[test]
    fn all_negative() {
        // Must return the least-negative element, not 0.
        assert_eq!(max_sub_array(vec![-3, -1, -2]), -1);
    }

    #[test]
    fn with_indices() {
        let (sum, lo, hi) = max_sub_array_with_indices(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]);
        assert_eq!(sum, 6);
        assert_eq!(&[-2_i32, 1, -3, 4, -1, 2, 1, -5, 4][lo..=hi], &[4, -1, 2, 1]);
    }
}
```

## Notes

- `i32::max` is called as a method in Rust (`x.max(current_sum + x)`), which is
  cleaner than `std::cmp::max(x, current_sum + x)`.
- The all-negative case is why we initialise with `nums[0]` instead of `0`.
  Initialising with `0` would incorrectly return `0` for `[-3, -1, -2]`.
- Kadane's is a 1D DP: `dp[i] = max(nums[i], dp[i-1] + nums[i])`, but because
  each state depends only on the previous one, we reduce the O(n) array to two
  scalar variables.
