use std::collections::{HashSet, VecDeque};
use std::fs;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn get_neighbors(pos: (usize, usize), height: usize, width: usize) -> Vec<(usize, usize)> {
    let (row, col) = pos;
    let mut neighbors = Vec::new();

    if row > 0 {
        neighbors.push((row - 1, col));
    }
    if row < height - 1 {
        neighbors.push((row + 1, col));
    }
    if col > 0 {
        neighbors.push((row, col - 1));
    }
    if col < width - 1 {
        neighbors.push((row, col + 1));
    }

    neighbors
}

fn count_reachable_nines(grid: &[Vec<u32>], start: (usize, usize)) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut reachable_nines = HashSet::new();

    // Start BFS from the trailhead
    queue.push_back((start, 0));
    visited.insert(start);

    while let Some(((row, col), current_height)) = queue.pop_front() {
        // If we reached a 9, add it to our set of reachable nines
        if grid[row][col] == 9 {
            reachable_nines.insert((row, col));
            continue;
        }

        // Check all neighbors
        for (next_row, next_col) in get_neighbors((row, col), height, width) {
            let next_height = grid[next_row][next_col];

            // Only follow path if it increases by exactly 1
            if next_height == current_height + 1 && !visited.contains(&(next_row, next_col)) {
                queue.push_back(((next_row, next_col), next_height));
                visited.insert((next_row, next_col));
            }
        }
    }

    reachable_nines.len()
}

fn count_paths(
    grid: &Vec<Vec<u32>>,
    pos: (usize, usize),
    current_height: u32,
    visited: &mut HashSet<(usize, usize)>,
) -> usize {
    let (row, col) = pos;

    // If we reached a 9, we've found a valid path
    if grid[row][col] == 9 {
        return 1;
    }

    let mut total_paths = 0;
    let height = grid.len();
    let width = grid[0].len();

    // Try all possible next steps
    for (next_row, next_col) in get_neighbors(pos, height, width) {
        let next_height = grid[next_row][next_col];

        // Only follow path if it increases by exactly 1 and we haven't visited it
        if next_height == current_height + 1 && !visited.contains(&(next_row, next_col)) {
            visited.insert((next_row, next_col));
            total_paths += count_paths(grid, (next_row, next_col), next_height, visited);
            visited.remove(&(next_row, next_col));
        }
    }

    total_paths
}

fn part1(input: &str) -> i32 {
    let grid = parse_input(input);
    let height = grid.len();
    let width = grid[0].len();
    let mut total_score = 0;

    // Find all trailheads (positions with height 0)
    for row in 0..height {
        for col in 0..width {
            if grid[row][col] == 0 {
                let score = count_reachable_nines(&grid, (row, col));
                total_score += score;
            }
        }
    }

    total_score as i32
}

fn part2(input: &str) -> i32 {
    let grid = parse_input(input);
    let height = grid.len();
    let width = grid[0].len();
    let mut total_rating = 0;
    let mut visited = HashSet::new();

    // Find all trailheads (positions with height 0)
    for row in 0..height {
        for col in 0..width {
            if grid[row][col] == 0 {
                visited.clear();
                visited.insert((row, col));
                let rating = count_paths(&grid, (row, col), 0, &mut visited);
                total_rating += rating;
            }
        }
    }

    total_rating as i32
}

fn main() {
    let input1 =
        fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
    let input2 =
        fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");

    println!("Part 1 test 1: {}", part1(&input1));
    println!("Part 1 test 2: {}", part1(&input2));

    println!("Part 2 test 1: {}", part2(&input1));
    println!("Part 2 test 2: {}", part2(&input2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 36);
    }

    #[test]
    fn test2_part1() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 694);
    }

    #[test]
    fn test1_part2() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 81);
    }

    #[test]
    fn test2_part2() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 1497);
    }
}
