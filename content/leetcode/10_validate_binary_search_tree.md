---
title: "Validate Binary Search Tree"
---

# Validate Binary Search Tree

**LeetCode #98 · Medium · Binary Tree / DFS**

## Problem

Given the root of a binary tree, determine whether it is a valid binary search
tree (BST). A valid BST satisfies:

- The left subtree of a node contains only nodes with keys **strictly less**
  than the node's key.
- The right subtree contains only keys **strictly greater** than the node's key.
- Both subtrees are themselves valid BSTs.

```
Input:      2
           / \
          1   3
Output: true

Input:      5
           / \
          1   4
             / \
            3   6
Output: false  // 4 is in the right subtree of 5 but 4 < 5
```

## Approach

### Bounded DFS (preferred)

Pass valid bounds `(min, max)` down the recursion. Every node's value must be
strictly inside `(min, max)`. Start with `(-∞, +∞)` and narrow the window as
you descend:

- Going left from a node with value `v`: upper bound becomes `v`.
- Going right: lower bound becomes `v`.

The common mistake is only comparing a node to its direct parent, which misses
the case where a deeply nested node violates an ancestor's constraint.

### In-order traversal

An in-order traversal of a BST produces strictly ascending values. Traverse in-
order and verify each value is greater than the previous.

## Complexity

| | |
|---|---|
| Time | O(n) — each node visited once |
| Space | O(h) — call stack depth, where h = tree height |

## Implementation

```rust
// LeetCode definition (provided in problem)
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left:  Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

type Node = Option<Rc<RefCell<TreeNode>>>;

// --- Bounded DFS ---
pub fn is_valid_bst(root: Node) -> bool {
    validate(&root, i64::MIN, i64::MAX)
}

fn validate(node: &Node, min: i64, max: i64) -> bool {
    match node {
        None => true,
        Some(n) => {
            let val = n.borrow().val as i64;
            if val <= min || val >= max {
                return false;
            }
            validate(&n.borrow().left,  min, val) &&
            validate(&n.borrow().right, val, max)
        }
    }
}

// --- In-order traversal variant ---
pub fn is_valid_bst_inorder(root: Node) -> bool {
    let mut prev: Option<i64> = None;
    inorder(&root, &mut prev)
}

fn inorder(node: &Node, prev: &mut Option<i64>) -> bool {
    let n = match node {
        None => return true,
        Some(n) => n.clone(),
    };

    // Left
    if !inorder(&n.borrow().left, prev) {
        return false;
    }

    // Visit
    let val = n.borrow().val as i64;
    if let Some(p) = *prev {
        if val <= p {
            return false;
        }
    }
    *prev = Some(val);

    // Right
    inorder(&n.borrow().right, prev)
}

// Test helpers
fn leaf(val: i32) -> Node {
    Some(Rc::new(RefCell::new(TreeNode { val, left: None, right: None })))
}

fn node(val: i32, left: Node, right: Node) -> Node {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_bst() {
        //      2
        //     / \
        //    1   3
        let tree = node(2, leaf(1), leaf(3));
        assert!(is_valid_bst(tree));
    }

    #[test]
    fn invalid_subtree() {
        //      5
        //     / \
        //    1   4
        //       / \
        //      3   6
        let tree = node(5, leaf(1), node(4, leaf(3), leaf(6)));
        assert!(!is_valid_bst(tree));
    }

    #[test]
    fn duplicate_not_valid() {
        // A BST requires strictly greater/less, not equal.
        let tree = node(2, leaf(2), None);
        assert!(!is_valid_bst(tree));
    }

    #[test]
    fn single_node() {
        assert!(is_valid_bst(leaf(1)));
    }

    #[test]
    fn both_methods_agree() {
        let valid   = node(4, node(2, leaf(1), leaf(3)), node(6, leaf(5), leaf(7)));
        let invalid = node(5, leaf(1), node(4, leaf(3), leaf(6)));
        assert_eq!(is_valid_bst(valid.clone()),   is_valid_bst_inorder(valid));
        assert_eq!(is_valid_bst(invalid.clone()), is_valid_bst_inorder(invalid));
    }
}
```

## Notes

- Bounds are widened to `i64` to handle `i32::MIN` and `i32::MAX` as node
  values without the bounds themselves conflicting with valid node values.
- `Rc<RefCell<TreeNode>>` is LeetCode's standard Rust tree scaffold. In your
  own code you would almost certainly use a different arena/index approach,
  but understanding this pattern is required for the platform.
- The bounded-DFS approach is generally preferred over in-order because it
  short-circuits immediately on the first violation rather than traversing the
  entire left subtree first.
- Related problems: Lowest Common Ancestor of a BST (#235), Kth Smallest
  Element in a BST (#230), and Convert Sorted Array to BST (#108).
