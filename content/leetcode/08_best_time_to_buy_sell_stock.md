---
title: "Best Time to Buy and Sell Stock"
---

# Best Time to Buy and Sell Stock

**LeetCode #121 · Easy · Greedy / Array**

## Problem

Given an array `prices` where `prices[i]` is the price of a stock on day `i`,
return the maximum profit you can achieve by buying on one day and selling on a
later day. If no profit is possible, return `0`.

```
Input:  prices = [7, 1, 5, 3, 6, 4]
Output: 5   // Buy on day 2 (price=1), sell on day 5 (price=6)

Input:  prices = [7, 6, 4, 3, 1]
Output: 0   // Prices only decrease; no profitable trade
```

## Approach

Track the minimum price seen so far (`min_price`) and the best profit seen so
far (`max_profit`). For each new price, update:

```
max_profit = max(max_profit, price - min_price)
min_price  = min(min_price,  price)
```

This works because `price - min_price` computes the profit if we sell today
and bought at the best possible earlier day. We never need to look back further.

## Complexity

| | |
|---|---|
| Time | O(n) — single pass |
| Space | O(1) |

## Implementation

```rust
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = i32::MAX;
    let mut max_profit = 0;

    for price in prices {
        max_profit = max_profit.max(price - min_price);
        min_price = min_price.min(price);
    }

    max_profit
}

// The same logic, written more explicitly to show the invariant:
pub fn max_profit_verbose(prices: Vec<i32>) -> i32 {
    if prices.is_empty() {
        return 0;
    }

    let mut min_price = prices[0];
    let mut max_profit = 0;

    for &price in &prices[1..] {
        let profit = price - min_price;
        if profit > max_profit {
            max_profit = profit;
        }
        if price < min_price {
            min_price = price;
        }
    }

    max_profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn example_2() {
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn single_day() {
        assert_eq!(max_profit(vec![5]), 0);
    }

    #[test]
    fn buy_day_1_sell_day_n() {
        assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
    }
}
```

## Notes

- Initialising `min_price = i32::MAX` means the first price always becomes the
  initial minimum without needing a special case.
- The order matters: update `max_profit` *before* updating `min_price`. If you
  updated `min_price` first and set it to today's price, you'd be computing a
  zero profit for the same-day buy/sell, which the problem disallows.
- Follow-up variations:
  - **#122** — unlimited transactions: sum every upward day-to-day difference.
  - **#123** — at most 2 transactions: requires a 4-state DP.
  - **#188** — at most k transactions: generalises #123.
  - **#309** — with cooldown: adds a rest state to the DP.
