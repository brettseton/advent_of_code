use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(input: &str) -> (Self, Position, Direction) {
        let cells: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let cols = cells[0].len();
        let rows = cells.len();

        // Find starting position and direction
        let mut start_pos = Position { x: 0, y: 0 };
        let mut start_dir = Direction::Up;

        for (i, row) in cells.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == '^' {
                    start_pos = Position { x: j, y: i };
                    start_dir = Direction::Up;
                }
            }
        }

        (Grid { cells, rows, cols }, start_pos, start_dir)
    }

    fn is_valid_pos(&self, pos: Position) -> bool {
        pos.y < self.rows && pos.x < self.cols
    }

    fn is_obstacle(&self, pos: Position) -> bool {
        self.cells[pos.y][pos.x] == '#'
    }

    fn next_pos(&self, pos: Position, dir: Direction) -> Option<Position> {
        let (dx, dy) = dir.delta();
        let new_col = pos.x as i32 + dx;
        let new_row = pos.y as i32 + dy;

        if new_row >= 0 && new_col >= 0 {
            let new_pos = Position {
                x: new_col as usize,
                y: new_row as usize,
            };
            if self.is_valid_pos(new_pos) {
                return Some(new_pos);
            }
        }
        None
    }

    fn clone_with_obstacle(&self, pos: Position) -> Self {
        let mut new_cells = self.cells.clone();
        new_cells[pos.y][pos.x] = '#';
        Grid {
            cells: new_cells,
            rows: self.rows,
            cols: self.cols,
        }
    }

    fn is_empty(&self, pos: Position) -> bool {
        self.cells[pos.y][pos.x] == '.'
    }
}

fn simulate_guard_movement(
    grid: &Grid,
    start_pos: Position,
    start_dir: Direction,
) -> HashSet<Position> {
    let mut visited = HashSet::new();
    let mut current_pos = start_pos;
    let mut current_dir = start_dir;

    visited.insert(current_pos);

    while let Some(next_pos) = grid.next_pos(current_pos, current_dir) {
        if grid.is_obstacle(next_pos) {
            current_dir = current_dir.turn_right();
        } else {
            current_pos = next_pos;
            visited.insert(current_pos);
        }

        if visited.len() > grid.rows * grid.cols * 4 {
            break;
        }
    }

    visited
}

fn detect_loop(grid: &Grid, start_pos: Position, start_dir: Direction) -> bool {
    let mut visited = HashSet::new();
    let mut current_pos = start_pos;
    let mut current_dir = start_dir;
    let mut position_states = HashSet::new();

    while let Some(next_pos) = grid.next_pos(current_pos, current_dir) {
        let state = (current_pos, current_dir);
        if position_states.contains(&state) {
            return true;
        }
        position_states.insert(state);

        if grid.is_obstacle(next_pos) {
            current_dir = current_dir.turn_right();
        } else {
            current_pos = next_pos;
            visited.insert(current_pos);
        }

        if visited.len() > grid.rows * grid.cols * 4 {
            return false;
        }
    }
    false
}

fn part1(input: &str) -> i32 {
    let (grid, start_pos, start_dir) = Grid::new(input);
    let visited = simulate_guard_movement(&grid, start_pos, start_dir);
    visited.len() as i32
}

fn part2(input: &str) -> i32 {
    let (grid, start_pos, start_dir) = Grid::new(input);
    let mut loop_positions = 0;

    let visited = simulate_guard_movement(&grid, start_pos, start_dir);

    for pos in visited {
        if pos != start_pos && grid.is_empty(pos) {
            let modified_grid = grid.clone_with_obstacle(pos);
            if detect_loop(&modified_grid, start_pos, start_dir) {
                loop_positions += 1;
            }
        }
    }

    loop_positions
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
        assert_eq!(part1(&test_input), 41);
    }

    #[test]
    fn test2_part1() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 5131);
    }

    #[test]
    fn test1_part2() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 6);
    }

    #[test]
    fn test2_part2() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 1784);
    }
}
