use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;

fn part1(input: &str, size: usize, take: usize) -> i32 {
    let byte_positions: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let coords: Vec<usize> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
            (coords[0], coords[1])
        })
        .collect();

    let mut grid = vec![vec!['.'; size]; size];

    // Mark corrupted positions
    for (x, y) in byte_positions.iter().take(take) {
        grid[*y][*x] = '#';
    }

    find_shortest_path(
        &grid,
        Position { x: 0, y: 0 },
        Position {
            x: size - 1,
            y: size - 1,
        },
    )
}

fn part2(input: &str, size: usize) -> String {
    let byte_positions: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let coords: Vec<usize> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
            (coords[0], coords[1])
        })
        .collect();

    let mut step_size = byte_positions.len() / 2;
    let mut take = step_size;
    while step_size > 0 {
        let mut grid = vec![vec!['.'; size]; size];

        // Mark corrupted positions
        for (x, y) in byte_positions.iter().take(take) {
            grid[*y][*x] = '#';
        }

        let result = find_shortest_path(
            &grid,
            Position { x: 0, y: 0 },
            Position {
                x: size - 1,
                y: size - 1,
            },
        );

        step_size /= 2;

        if result == -1 {
            take -= step_size;
        } else {
            take += step_size;
        }
    }

    return format!("{},{}", byte_positions[take].0, byte_positions[take].1);
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Position {
    x: usize,
    y: usize,
}

// Placeholder for the pathfinding function
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: Position,
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

fn find_shortest_path(grid: &[Vec<char>], start: Position, goal: Position) -> i32 {
    let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)]; // Right, Down, Left, Up
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();

    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return cost;
        }

        if !visited.insert(position) {
            continue;
        }

        for &(dx, dy) in &directions {
            let new_x = position.x as isize + dx;
            let new_y = position.y as isize + dy;

            if new_x >= 0
                && new_x < grid.len() as isize
                && new_y >= 0
                && new_y < grid[0].len() as isize
            {
                let new_position = Position {
                    x: new_x as usize,
                    y: new_y as usize,
                };
                if grid[new_position.y][new_position.x] == '#' {
                    continue;
                }
                heap.push(State {
                    cost: cost + 1,
                    position: new_position,
                });
            }
        }
    }

    -1 // Return -1 if no path is found
}

fn main() {
    let input1 =
        fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
    let input2 =
        fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");

    println!("Part 1 test 1: {}", part1(&input1, 7, 12));
    println!("Part 1 test 2: {}", part1(&input2, 71, 1024));

    println!("Part 2 test 1: {:?}", part2(&input1, 7));
    println!("Part 2 test 2: {:?}", part2(&input2, 71));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input, 7, 12), 22);
    }

    #[test]
    fn test2_part1() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input, 71, 1024), 348);
    }

    #[test]
    fn test1_part2() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input, 7), "6,1");
    }

    #[test]
    fn test2_part2() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input, 71), "54,44");
    }
}
