---
title: LeetCode
---

# LeetCode

Rust solutions and explanations for the most important LeetCode problems.
Each file covers the problem statement, approach, complexity analysis, and a
full idiomatic Rust implementation with tests.

## Problems

| # | Problem | Difficulty | Topics |
|---|---------|------------|--------|
| 1 | [Two Sum](01_two_sum) | Easy | Array, Hash Map |
| 20 | [Valid Parentheses](02_valid_parentheses) | Easy | Stack |
| 21 | [Merge Two Sorted Lists](09_merge_two_sorted_lists) | Easy | Linked List |
| 53 | [Maximum Subarray](05_maximum_subarray) | Medium | DP, Kadane's |
| 70 | [Climbing Stairs](06_climbing_stairs) | Easy | DP, Fibonacci |
| 98 | [Validate Binary Search Tree](10_validate_binary_search_tree) | Medium | Tree, DFS |
| 121 | [Best Time to Buy and Sell Stock](08_best_time_to_buy_sell_stock) | Easy | Greedy, Array |
| 200 | [Number of Islands](07_number_of_islands) | Medium | Graph, DFS, BFS |
| 206 | [Reverse Linked List](03_reverse_linked_list) | Easy | Linked List |
| 704 | [Binary Search](04_binary_search) | Easy | Binary Search |

## Key Patterns

- **Hash Map** — instant lookups to reduce O(n²) to O(n): Two Sum.
- **Stack** — LIFO matching for bracket/nesting problems: Valid Parentheses.
- **Two Pointers / Sliding Window** — linear traversal without backtracking.
- **Binary Search** — halve the search space each step on sorted data.
- **Kadane's Algorithm** — max subarray in a single pass.
- **Dynamic Programming** — break into subproblems; Fibonacci recurrence is
  the simplest form.
- **DFS / BFS Flood-Fill** — graph reachability and island counting.
- **Greedy** — locally optimal choices that guarantee globally optimal results.
- **Bounded Tree Recursion** — pass invariant bounds down a recursive DFS.
