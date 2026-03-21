---
title: "Reverse Linked List"
---

# Reverse Linked List

**LeetCode #206 · Easy · Linked List**

## Problem

Given the head of a singly linked list, reverse it and return the new head.

```
Input:  1 → 2 → 3 → 4 → 5 → null
Output: 5 → 4 → 3 → 2 → 1 → null
```

## Approach

### Iterative (O(1) space)

Maintain three pointers: `prev` (starts as `None`), `curr` (starts at head),
and `next_node` (lookahead). On each step:

1. Save `curr.next` into `next_node`.
2. Point `curr.next` to `prev`.
3. Advance: `prev = curr`, `curr = next_node`.

When `curr` is `None` the list is fully reversed and `prev` is the new head.

### Recursive (O(n) stack space)

Recurse to the tail, then on the way back re-wire each `next` pointer.

## Complexity

| | Iterative | Recursive |
|---|---|---|
| Time | O(n) | O(n) |
| Space | O(1) | O(n) — call stack |

## Implementation

LeetCode's Rust scaffold uses a `Box<ListNode>` ownership model:

```rust
// LeetCode definition (provided in problem)
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

// --- Iterative solution ---
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev: Option<Box<ListNode>> = None;
    let mut curr = head;

    while let Some(mut node) = curr {
        // Detach the rest of the list before re-wiring.
        curr = node.next.take();
        node.next = prev;
        prev = Some(node);
    }

    prev
}

// --- Recursive solution ---
pub fn reverse_list_recursive(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn helper(
        node: Option<Box<ListNode>>,
        prev: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match node {
            None => prev,
            Some(mut n) => {
                let next = n.next.take();
                n.next = prev;
                helper(next, Some(n))
            }
        }
    }

    helper(head, None)
}
```

## Notes

- `node.next.take()` is essential in the iterative version: it moves ownership
  of the tail out of the current node *before* we reassign `node.next = prev`.
  Without `take()` the borrow checker would flag a conflict.
- Rust's ownership model means the recursive version is essentially a
  tail-recursive accumulator, though the compiler does not guarantee TCO (tail
  call optimisation). For very long lists, prefer the iterative approach.
- The `while let Some(mut node) = curr` pattern simultaneously checks for
  `None` and binds the inner `Box<ListNode>` — idiomatic Rust linked-list
  traversal.
