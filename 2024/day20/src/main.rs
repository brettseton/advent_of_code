use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Position {
    x: usize,
    y: usize,
}

fn generate_cost_map(grid: &[Vec<char>], start: Position) -> Vec<Vec<i32>> {
    let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)]; // Right, Down, Left, Up
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    visited.insert(start);

    queue.push_back((start, 0));
    // generate a floodfill cost map
    let mut cost_map = vec![vec![i32::MAX; grid[0].len()]; grid.len()];
    cost_map[start.y][start.x] = 0;

    while let Some((pos, time)) = queue.pop_front() {
        for (dx, dy) in &directions {
            let new_x = pos.x as isize + dx;
            let new_y = pos.y as isize + dy;

            if new_x >= 0
                && new_x < grid.len() as isize
                && new_y >= 0
                && new_y < grid[0].len() as isize
                && grid[new_y as usize][new_x as usize] != '#'
            {
                let new_position = Position {
                    x: new_x as usize,
                    y: new_y as usize,
                };

                if visited.insert(new_position) {
                    cost_map[new_position.y][new_position.x] = time + 1;
                    queue.push_back((new_position, time + 1));
                }
            }
        }
    }

    cost_map
}

fn get_shortest_path(grid: &[Vec<char>], start: Position, end: Position) -> Vec<Position> {
    let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)]; // Right, Down, Left, Up
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();

    queue.push_back(start);

    while let Some(pos) = queue.pop_front() {
        if pos == end {
            let mut path = vec![pos];
            let mut prev = visited[&pos];
            while prev != start {
                path.push(prev);
                prev = visited[&prev];
            }
            path.push(start);
            return path;
        }
        for (dx, dy) in &directions {
            let new_x = pos.x as isize + dx;
            let new_y = pos.y as isize + dy;

            if new_x >= 0
                && new_x < grid.len() as isize
                && new_y >= 0
                && new_y < grid[0].len() as isize
                && grid[new_y as usize][new_x as usize] != '#'
            {
                let new_position = Position {
                    x: new_x as usize,
                    y: new_y as usize,
                };

                if visited.contains_key(&new_position) {
                    continue;
                }
                visited.insert(new_position, pos);
                queue.push_back(new_position);
            }
        }
    }

    vec![]
}

fn part2(input: &str, time_limit: i32, distance: i32) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut start: Position = Position { x: 0, y: 0 };
    let mut end: Position = Position { x: 0, y: 0 };

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                start = Position { x, y };
            } else if cell == 'E' {
                end = Position { x, y };
            }
        }
    }

    // generate a floodfill cost map
    let end_cost_map = generate_cost_map(&grid, end);
    let start_cost_map = generate_cost_map(&grid, start);

    let start_cost = end_cost_map[start.y][start.x];
    let shortest_path = get_shortest_path(&grid, start, end);
    let mut total_cheats = 0; // Counter for cheats saving at least 100 picoseconds

    // Check for possible cheats
    let mut distinct_cheats = HashSet::new();
    for (_idx, step) in shortest_path.iter().rev().enumerate() {
        for cheat_x in -distance..=distance {
            for cheat_y in (-distance + cheat_x.abs())..=(distance - cheat_x.abs()) {
                let new_x = step.x as i32 + cheat_x;
                let new_y = step.y as i32 + cheat_y;
                if new_x < 0
                    || new_y < 0
                    || new_x >= grid.len() as i32
                    || new_y >= grid[0].len() as i32
                    || grid[new_y as usize][new_x as usize] == '#'
                {
                    continue;
                }
                // Loop through all possible cheat times
                let destination = Position {
                    x: new_x as usize,
                    y: new_y as usize,
                };
                let shortcut_cost =
                    start_cost_map[step.y][step.x] + end_cost_map[destination.y][destination.x];
                let saved_time = start_cost - shortcut_cost - cheat_x.abs() - cheat_y.abs();
                if saved_time >= time_limit && distinct_cheats.insert((step, destination)) {
                    total_cheats += 1;
                }
            }
        }
    }

    total_cheats
}

fn main() {
    let input1 =
        fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
    let input2 =
        fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");

    println!("Part 1 test 1: {}", part2(&input1, 20, 2));
    println!("Part 1 test 2: {}", part2(&input2, 100, 2));

    println!("Part 2 test 1: {}", part2(&input1, 74, 6));
    println!("Part 2 test 2: {}", part2(&input2, 100, 20));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input, 20, 2), 5);
    }

    #[test]
    fn test2_part1() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input, 100, 2), 1499);
    }

    #[test]
    fn test1_part2() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input, 74, 6), 1);
    }

    #[test]
    fn test2_part2() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input, 100, 20), 1027164);
    }
}
