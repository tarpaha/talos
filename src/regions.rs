use std::collections::{HashSet, VecDeque};

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