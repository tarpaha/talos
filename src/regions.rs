use std::collections::{HashSet, VecDeque};

/// Finds and calculates the sizes of all connected regions of free cells within a grid.
///
/// # Parameters
/// - `width`: The width of the grid (number of columns).
/// - `height`: The height of the grid (number of rows).
/// - `free_cells`: A `HashSet` containing the coordinates of the free cells within the grid.
///   Each coordinate is represented as a tuple `(x, y)`, where `0 <= x < width` and
///   `0 <= y < height`.
///
/// # Returns
/// A `Vec<usize>` where each element represents the size of a connected region of free cells. 
/// A region is defined as a set of free cells that are connected horizontally or vertically.
///
/// # Algorithm
/// - The function uses a breadth-first search (BFS) algorithm to traverse the grid.
/// - It iterates over all free cells provided in the `free_cells` set.
/// - For each free cell that has not been visited yet, a BFS is initiated to explore
///   all connected cells, marking them as visited and calculating the size of the
///   connected region.
/// - The function ensures that only cells within the grid boundaries and present in the
///   `free_cells` set are considered during exploration.
///
/// # Constraints
/// - Assumes that the grid dimensions (`width` and `height`) are non-negative integers.
/// - Assumes that the `free_cells` set contains valid grid coordinates within `0 <= x < width`
///   and `0 <= y < height`.
///
/// # Complexity
/// - Time Complexity: O(N), where N is the number of free cells in the grid, as each cell
///   is visited exactly once.
/// - Space Complexity: O(N), for the `visited` set and BFS queue.
///
/// # Example
/// ```rust
/// use std::collections::HashSet;
///
/// let width = 5;
/// let height = 5;
/// let free_cells: HashSet<(i32, i32)> =
///     [(0, 0), (0, 1), (1, 1), (3, 3), (4, 3), (4, 4)].iter().cloned().collect();
/// let region_sizes = find_connected_region_sizes(width, height, &free_cells);
/// assert_eq!(region_sizes, vec![3, 3]); // Two regions: size 3 and size 3
/// ```
pub fn find_connected_region_sizes(width: i32, height: i32, free_cells: &HashSet<(i32, i32)>) -> Vec<usize> {
    let mut spaces = Vec::with_capacity(free_cells.len());
    let mut visited = HashSet::with_capacity(free_cells.len());
    let mut bfs = VecDeque::with_capacity(free_cells.len());

    const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for &start in free_cells {
        if visited.contains(&start) {
            continue;
        }
        
        bfs.clear();
        bfs.push_back(start);
        let mut region_size = 0;

        while let Some((x, y)) = bfs.pop_front() {
            if !visited.insert((x, y)) {
                continue;
            }
            region_size += 1;

            for &(dx, dy) in &DIRECTIONS {
                let new_x = x + dx;
                let new_y = y + dy;
                if new_x >= 0 && new_x < width && new_y >= 0 && new_y < height {
                    let neighbor = (new_x, new_y);
                    if !visited.contains(&neighbor) && free_cells.contains(&neighbor) {
                        bfs.push_back(neighbor);
                    }
                }
            }
        }
        spaces.push(region_size);
    }
    spaces
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_empty_u_sharped_space() {
        let free_cells = HashSet::from([
            (0, 0),         (2, 0),
            (0, 1),         (2, 1),
            (0, 2), (1, 2), (2, 2),
        ]);
        let mut spaces = find_connected_region_sizes(3, 3, &free_cells);
        spaces.sort_unstable();
        assert_eq!(spaces, vec![7]);
    }

    #[test]
    fn test_three_empty_spaces() {
        let free_cells = HashSet::from([
            (0, 0),         (2, 0),
                    (1, 1), (2, 1),
            (0, 2),         (2, 2),
        ]);
        let mut spaces = find_connected_region_sizes(3, 3, &free_cells);
        spaces.sort_unstable();
        assert_eq!(spaces, vec![1, 1, 4]);
    }

    #[test]
    fn test_single_cell() {
        let free_cells = HashSet::from([(0, 0)]);
        let spaces = find_connected_region_sizes(1, 1, &free_cells);
        assert_eq!(spaces, vec![1]);
    }

    #[test]
    fn test_no_free_cells() {
        let free_cells = HashSet::new();
        let spaces = find_connected_region_sizes(3, 3, &free_cells);
        assert_eq!(spaces, Vec::new());
    }

    #[test]
    fn test_all_cells_are_free() {
        let free_cells = HashSet::from([
            (0, 0), (0, 1),
            (1, 0), (1, 1),
        ]);
        let spaces = find_connected_region_sizes(2, 2, &free_cells);
        assert_eq!(spaces, vec![4]);
    }
}