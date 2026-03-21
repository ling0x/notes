---
title: "Number of Islands"
---

# Number of Islands

**LeetCode #200 · Medium · Graph / DFS / BFS**

## Problem

Given an `m × n` 2D binary grid of `'1'` (land) and `'0'` (water), return the
number of islands. An island is surrounded by water and is formed by connecting
adjacent land cells horizontally or vertically.

```
Input:
  11110
  11010
  11000
  00000

Output: 1

Input:
  11000
  11000
  00100
  00011

Output: 3
```

## Approach

Iterate every cell. When you find a `'1'`, increment the island count and then
flood-fill (DFS or BFS) to mark every connected land cell as visited (by
changing it to `'0'`). This ensures each island is counted only once.

The four cardinal directions are `(±1, 0)` and `(0, ±1)`.

## Complexity

| | |
|---|---|
| Time | O(m × n) — each cell visited at most once |
| Space | O(m × n) — worst-case DFS stack (all land) |

## Implementation

```rust
pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { return 0 };
    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '1' {
                count += 1;
                dfs(&mut grid, r, c, rows, cols);
            }
        }
    }

    count
}

fn dfs(grid: &mut Vec<Vec<char>>, r: usize, c: usize, rows: usize, cols: usize) {
    if grid[r][c] != '1' {
        return;
    }
    // Mark as visited by "sinking" the cell.
    grid[r][c] = '0';

    // Explore all four neighbours (bounds-checked with saturating arithmetic).
    if r + 1 < rows { dfs(grid, r + 1, c, rows, cols); }
    if c + 1 < cols { dfs(grid, r, c + 1, rows, cols); }
    if r > 0        { dfs(grid, r - 1, c, rows, cols); }
    if c > 0        { dfs(grid, r, c - 1, rows, cols); }
}

// BFS variant using an explicit queue (avoids deep recursion stacks).
use std::collections::VecDeque;

pub fn num_islands_bfs(mut grid: Vec<Vec<char>>) -> i32 {
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { return 0 };
    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '1' {
                count += 1;
                grid[r][c] = '0';

                let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
                queue.push_back((r, c));

                while let Some((row, col)) = queue.pop_front() {
                    let neighbours = [
                        (row.wrapping_sub(1), col),
                        (row + 1, col),
                        (row, col.wrapping_sub(1)),
                        (row, col + 1),
                    ];
                    for (nr, nc) in neighbours {
                        if nr < rows && nc < cols && grid[nr][nc] == '1' {
                            grid[nr][nc] = '0';
                            queue.push_back((nr, nc));
                        }
                    }
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn g(s: &[&str]) -> Vec<Vec<char>> {
        s.iter().map(|row| row.chars().collect()).collect()
    }

    #[test]
    fn single_island() {
        assert_eq!(num_islands(g(&["11110", "11010", "11000", "00000"])), 1);
    }

    #[test]
    fn three_islands() {
        assert_eq!(num_islands(g(&["11000", "11000", "00100", "00011"])), 3);
    }

    #[test]
    fn no_land() {
        assert_eq!(num_islands(g(&["000", "000"])), 0);
    }

    #[test]
    fn all_land() {
        assert_eq!(num_islands(g(&["111", "111"])), 1);
    }
}
```

## Notes

- Mutating the grid in-place (`'1'` → `'0'`) is the standard trick to avoid
  allocating a separate `visited` matrix. If the input must be preserved,
  clone the grid first.
- The BFS version with `VecDeque` is preferable for very large grids where the
  DFS call stack could overflow (Rust's default stack is 8 MB).
- `wrapping_sub(1)` on `usize` produces `usize::MAX` for `0 - 1`, which is
  then immediately rejected by the `< rows` / `< cols` bounds check — a tidy
  way to handle the upper-left boundary without signed arithmetic.
- This is the canonical entry point to the Union-Find (disjoint-set) data
  structure, which can also solve this problem in near-O(1) amortised per cell.
