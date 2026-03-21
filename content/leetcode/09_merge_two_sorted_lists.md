---
title: "Merge Two Sorted Lists"
---

# Merge Two Sorted Lists

**LeetCode #21 · Easy · Linked List**

## Problem

Merge two sorted linked lists and return the head of the merged list. The
merged list should be sorted by splicing together the nodes of the two given
lists.

```
Input:  list1 = 1 → 2 → 4,  list2 = 1 → 3 → 4
Output: 1 → 1 → 2 → 3 → 4 → 4
```

## Approach

### Iterative

Use a dummy head node to simplify edge cases. Maintain a `curr` pointer to the
tail of the merged list. On each step, compare the front nodes of the two lists
and append the smaller one to `curr`, advancing that list's head.

When one list is exhausted, append the remainder of the other directly (since
it's already sorted).

### Recursive

The recursive structure mirrors the iterative logic directly: pick the smaller
head, then recursively merge the rest.

## Complexity

| | |
|---|---|
| Time | O(m + n) — each node visited once |
| Space | O(1) iterative / O(m + n) recursive (call stack) |

## Implementation

```rust
// LeetCode definition (provided in problem)
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

type List = Option<Box<ListNode>>;

// --- Iterative ---
pub fn merge_two_lists(mut l1: List, mut l2: List) -> List {
    // A dummy node whose `next` will become the merged head.
    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy.next; // &mut List

    loop {
        match (l1.as_ref(), l2.as_ref()) {
            (None, _) => {
                *tail = l2;
                break;
            }
            (_, None) => {
                *tail = l1;
                break;
            }
            (Some(a), Some(b)) => {
                if a.val <= b.val {
                    // Advance l1: take ownership of the node.
                    let next = l1.as_mut().unwrap().next.take();
                    *tail = l1;
                    tail = &mut tail.as_mut().unwrap().next;
                    l1 = next;
                } else {
                    let next = l2.as_mut().unwrap().next.take();
                    *tail = l2;
                    tail = &mut tail.as_mut().unwrap().next;
                    l2 = next;
                }
            }
        }
    }

    dummy.next
}

// --- Recursive (cleaner but O(n) stack) ---
pub fn merge_two_lists_recursive(l1: List, l2: List) -> List {
    match (l1, l2) {
        (None, list) | (list, None) => list,
        (Some(mut a), Some(mut b)) => {
            if a.val <= b.val {
                a.next = merge_two_lists_recursive(a.next.take(), Some(b));
                Some(a)
            } else {
                b.next = merge_two_lists_recursive(Some(a), b.next.take());
                Some(b)
            }
        }
    }
}

// Helper for tests: build a list from a slice.
fn to_list(vals: &[i32]) -> List {
    vals.iter().rev().fold(None, |next, &val| {
        Some(Box::new(ListNode { val, next }))
    })
}

// Helper for tests: collect list values into a Vec.
fn to_vec(mut node: &List) -> Vec<i32> {
    let mut out = vec![];
    while let Some(n) = node {
        out.push(n.val);
        node = &n.next;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_merge() {
        let merged = merge_two_lists(to_list(&[1, 2, 4]), to_list(&[1, 3, 4]));
        assert_eq!(to_vec(&merged), vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn both_empty() {
        assert!(merge_two_lists(None, None).is_none());
    }

    #[test]
    fn one_empty() {
        let merged = merge_two_lists(None, to_list(&[1]));
        assert_eq!(to_vec(&merged), vec![1]);
    }

    #[test]
    fn recursive_matches_iterative() {
        let l1 = to_list(&[1, 3, 5]);
        let l2 = to_list(&[2, 4, 6]);
        let ri = to_vec(&merge_two_lists(l1.clone(), l2.clone()));
        let rr = to_vec(&merge_two_lists_recursive(l1, l2));
        assert_eq!(ri, rr);
    }
}
```

## Notes

- The iterative version is tricky in Rust because threading a `&mut` tail
  pointer through `Option<Box<ListNode>>` requires careful use of `.take()` to
  move ownership without conflicting borrows.
- The recursive version is much cleaner thanks to `match (l1, l2)` with an
  or-pattern `(None, list) | (list, None)`, which handles both "one exhausted"
  cases in a single arm.
- This problem is a building block for **Merge K Sorted Lists (#23)**, which
  uses a min-heap (BinaryHeap) to generalise the two-pointer step.
