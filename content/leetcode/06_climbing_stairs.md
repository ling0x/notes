---
title: "Climbing Stairs"
---

# Climbing Stairs

**LeetCode #70 · Easy · Dynamic Programming**

## Problem

You are climbing a staircase. It takes `n` steps to reach the top. Each time
you can climb either 1 or 2 steps. In how many distinct ways can you climb to
the top?

```
Input:  n = 2
Output: 2   // [1+1, 2]

Input:  n = 3
Output: 3   // [1+1+1, 1+2, 2+1]
```

## Approach

Let `f(n)` be the number of ways to reach step `n`. To reach step `n`, you
must have come from step `n-1` (one step) or step `n-2` (two steps):

```
f(n) = f(n-1) + f(n-2)
```

This is exactly the Fibonacci recurrence with base cases `f(1) = 1`,
`f(2) = 2`.

The full table for small n:

| n | ways |
|---|------|
| 1 | 1    |
| 2 | 2    |
| 3 | 3    |
| 4 | 5    |
| 5 | 8    |

## Complexity

| | Memoised / Bottom-up | Space-optimised |
|---|---|---|
| Time | O(n) | O(n) |
| Space | O(n) | O(1) |

## Implementation

```rust
// Space-optimised: O(1) — only keep the last two values.
pub fn climb_stairs(n: i32) -> i32 {
    if n <= 2 {
        return n;
    }

    let (mut prev2, mut prev1) = (1_i32, 2_i32); // f(1), f(2)

    for _ in 3..=n {
        let current = prev1 + prev2;
        prev2 = prev1;
        prev1 = current;
    }

    prev1
}

// Bottom-up DP table (easier to reason about, O(n) space).
pub fn climb_stairs_dp(n: usize) -> usize {
    if n <= 2 {
        return n;
    }

    let mut dp = vec![0usize; n + 1];
    dp[1] = 1;
    dp[2] = 2;

    for i in 3..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    dp[n]
}

// Recursive with memoisation (top-down).
pub fn climb_stairs_memo(n: i32) -> i32 {
    fn dp(n: i32, memo: &mut Vec<i32>) -> i32 {
        if n <= 2 {
            return n;
        }
        if memo[n as usize] != 0 {
            return memo[n as usize];
        }
        memo[n as usize] = dp(n - 1, memo) + dp(n - 2, memo);
        memo[n as usize]
    }

    let mut memo = vec![0_i32; (n + 1) as usize];
    dp(n, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_steps() {
        assert_eq!(climb_stairs(2), 2);
    }

    #[test]
    fn three_steps() {
        assert_eq!(climb_stairs(3), 3);
    }

    #[test]
    fn ten_steps() {
        // Fibonacci: f(10) = 89
        assert_eq!(climb_stairs(10), 89);
    }

    #[test]
    fn all_variants_agree() {
        for n in 1..=20 {
            let a = climb_stairs(n);
            let b = climb_stairs_dp(n as usize) as i32;
            let c = climb_stairs_memo(n);
            assert_eq!(a, b, "mismatch at n={n}");
            assert_eq!(a, c, "mismatch at n={n}");
        }
    }
}
```

## Notes

- The space-optimised rolling-variable approach is the interview answer to give.
  It demonstrates that you understand the recurrence and can avoid unnecessary
  allocations.
- The top-down memo version uses a `Vec` instead of a `HashMap` because `n` is
  bounded, making index-based lookup O(1) and cache-friendly.
- This problem is a gateway to 1D DP. The same recurrence structure appears in
  House Robber (#198), Decode Ways (#91), and Minimum Cost Climbing Stairs (#746).
