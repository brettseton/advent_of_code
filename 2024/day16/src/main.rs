use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

type Grid = Vec<Vec<char>>;
type Position = (i32, i32);
type ParseResult = (Grid, Position, Position);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn move_in_direction(&self, pos: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::North => (pos.0 - 1, pos.1),
            Direction::South => (pos.0 + 1, pos.1),
            Direction::East => (pos.0, pos.1 + 1),
            Direction::West => (pos.0, pos.1 - 1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    cost: i32,
    position: (i32, i32),
    direction: Direction,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq)]
struct QueueItem {
    cost: i32,
    position: (i32, i32),
    direction: Direction,
    path: HashSet<(i32, i32)>,
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str) -> ParseResult {
    let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                start = (i as i32, j as i32);
            } else if cell == 'E' {
                end = (i as i32, j as i32);
            }
        }
    }

    (grid, start, end)
}

fn is_valid_position(pos: (i32, i32), grid: &[Vec<char>]) -> bool {
    pos.0 >= 0
        && pos.1 >= 0
        && pos.0 < grid.len() as i32
        && pos.1 < grid[0].len() as i32
        && grid[pos.0 as usize][pos.1 as usize] != '#'
}

fn find_shortest_path(grid: &[Vec<char>], start: (i32, i32), end: (i32, i32)) -> i32 {
    let mut heap = BinaryHeap::new();
    let mut costs = HashMap::new();

    // Start facing East as specified
    heap.push(State {
        cost: 0,
        position: start,
        direction: Direction::East,
    });

    while let Some(State {
        cost,
        position,
        direction,
    }) = heap.pop()
    {
        if position == end {
            return cost;
        }

        let state_key = (position, direction);
        let existing_cost = costs.get(&state_key);
        if existing_cost.is_some_and(|&c| cost >= c) {
            continue;
        }
        costs.insert(state_key, cost);

        // Try turning left
        let left_dir = direction.turn_left();
        heap.push(State {
            cost: cost + 1000,
            position,
            direction: left_dir,
        });

        // Try turning right
        let right_dir = direction.turn_right();
        heap.push(State {
            cost: cost + 1000,
            position,
            direction: right_dir,
        });

        // Try moving forward
        let next_pos = direction.move_in_direction(position);
        if is_valid_position(next_pos, grid) {
            heap.push(State {
                cost: cost + 1,
                position: next_pos,
                direction,
            });
        }
    }

    unreachable!("No path found")
}

#[allow(dead_code)]
fn print_grid_with_path(grid: &[Vec<char>], optimal_tiles: &HashSet<(i32, i32)>) {
    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if optimal_tiles.contains(&(i as i32, j as i32)) {
                print!("O");
            } else {
                print!("{}", cell);
            }
        }
        println!();
    }
    println!();
}

fn find_optimal_path_tiles(
    grid: &[Vec<char>],
    start: (i32, i32),
    end: (i32, i32),
) -> HashSet<(i32, i32)> {
    let mut heap = BinaryHeap::new();
    let mut costs = HashMap::new();
    let mut optimal_tiles = HashSet::new();
    let mut min_cost = i32::MAX;

    // Pre-calculate grid dimensions to avoid repeated conversions
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    // First pass: Find the minimum cost
    heap.push(QueueItem {
        cost: 0,
        position: start,
        direction: Direction::East,
        path: HashSet::from([start]),
    });

    // Use a capacity hint for HashMaps based on grid size
    costs.reserve(height as usize * width as usize * 4);
    optimal_tiles.reserve(height as usize * width as usize);

    while let Some(QueueItem {
        cost,
        position,
        direction,
        path,
    }) = heap.pop()
    {
        if cost > min_cost {
            break;
        }

        if position == end && cost <= min_cost {
            min_cost = min_cost.min(cost);

            optimal_tiles.extend(path);
            continue;
        }

        let state_key = (position, direction);
        let existing_cost = costs.get(&state_key);
        if existing_cost.is_some_and(|&c| cost > c) {
            continue;
        }
        costs.insert(state_key, cost);

        // Try moving forward first (most common case)
        let next_pos = direction.move_in_direction(position);
        if is_valid_position(next_pos, grid) {
            let mut new_path = path.clone();
            new_path.insert(next_pos);
            heap.push(QueueItem {
                cost: cost + 1,
                position: next_pos,
                direction,
                path: new_path,
            });
        }

        for new_dir in [direction.turn_left(), direction.turn_right()] {
            let mut new_path = path.clone();
            new_path.insert(position);
            heap.push(QueueItem {
                cost: cost + 1000,
                position,
                direction: new_dir,
                path: new_path,
            });
        }
    }

    optimal_tiles
}

fn part1(input: &str) -> i32 {
    let (grid, start, end) = parse_input(input);
    find_shortest_path(&grid, start, end)
}

fn part2(input: &str) -> i32 {
    let (grid, start, end) = parse_input(input);

    let optimal_tiles = find_optimal_path_tiles(&grid, start, end);
    //print_grid_with_path(&grid, &optimal_tiles);
    optimal_tiles.len() as i32
}

fn main() {
    let input1 =
        fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
    let input2 =
        fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
    let input3 =
        fs::read_to_string("input/test3.txt").expect("Should have been able to read the file");

    println!("Part 1 test 1: {}", part1(&input1));
    println!("Part 1 test 2: {}", part1(&input2));
    println!("Part 1 test 3: {}", part1(&input3));

    println!("Part 2 test 1: {}", part2(&input1));
    println!("Part 2 test 2: {}", part2(&input2));
    println!("Part 2 test 3: {}", part2(&input3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 7036);
    }

    #[test]
    fn test2_part1() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 11048);
    }

    #[test]
    fn test3_part1() {
        let test_input =
            fs::read_to_string("input/test3.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 143580);
    }

    #[test]
    fn test1_part2() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 45);
    }

    #[test]
    fn test2_part2() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 64);
    }

    #[test]
    fn test3_part2() {
        let test_input =
            fs::read_to_string("input/test3.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 645);
    }
}
