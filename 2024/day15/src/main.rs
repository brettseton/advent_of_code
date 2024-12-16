use std::{self, fmt::Display, fs};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Wall,
    Box,
    BoxLeft,
    BoxRight,
    Robot,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Empty => ".",
                Cell::Wall => "#",
                Cell::Box => "O",
                Cell::BoxLeft => "[",
                Cell::BoxRight => "]",
                Cell::Robot => "@",
            }
        )
    }
}

struct Warehouse {
    grid: Vec<Vec<Cell>>,
    robot_pos: Position,
    wide_boxes: bool,
}

struct Position {
    x: usize,
    y: usize,
}

impl Warehouse {
    fn new(input: &str, wide_boxes: bool) -> Self {
        let mut grid = Vec::new();
        let mut robot_pos = Position { x: 0, y: 0 };

        for (row, line) in input.lines().enumerate() {
            let mut grid_row = Vec::new();
            let chars: Vec<char> = line.chars().collect();
            let mut col = 0;
            for c in chars {
                if wide_boxes {
                    match c {
                        '#' => {
                            grid_row.push(Cell::Wall);
                            grid_row.push(Cell::Wall);
                            col += 1;
                        }
                        'O' => {
                            grid_row.push(Cell::BoxLeft);
                            grid_row.push(Cell::BoxRight);
                            col += 1;
                        }
                        '@' => {
                            robot_pos = Position { x: col, y: row };
                            grid_row.push(Cell::Robot);
                            grid_row.push(Cell::Empty);
                            col += 1;
                        }
                        '.' => {
                            grid_row.push(Cell::Empty);
                            grid_row.push(Cell::Empty);
                            col += 1;
                        }
                        _ => col += 1,
                    }
                } else {
                    let cell = match c {
                        '#' => Cell::Wall,
                        'O' => Cell::Box,
                        '@' => {
                            robot_pos = Position { x: col, y: row };
                            Cell::Robot
                        }
                        _ => Cell::Empty,
                    };
                    grid_row.push(cell);
                }
                col += 1;
            }
            if !grid_row.is_empty() {
                grid.push(grid_row);
            }
        }

        Warehouse {
            grid,
            robot_pos,
            wide_boxes,
        }
    }

    fn try_move(&mut self, direction: char) {
        let (dx, dy) = match direction {
            '^' => (0, -1),
            'v' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => return,
        };

        let new_col = (self.robot_pos.x as i32 + dx) as usize;
        let new_row = (self.robot_pos.y as i32 + dy) as usize;

        // Check if the new position is within bounds
        if new_row >= self.grid.len() || new_col >= self.grid[0].len() {
            return;
        }

        match self.grid[new_row][new_col] {
            Cell::Empty => {
                // Move robot
                self.grid[self.robot_pos.y][self.robot_pos.x] = Cell::Empty;
                self.grid[new_row][new_col] = Cell::Robot;
                self.robot_pos = Position {
                    x: new_col,
                    y: new_row,
                };
            }
            Cell::Box => {
                match self.can_push_box(new_row, new_col, dx, dy) {
                    Some(pos) => {
                        self.grid[pos.y][pos.x] = Cell::Box;
                    }
                    None => return,
                }

                // Move robot
                self.grid[self.robot_pos.y][self.robot_pos.x] = Cell::Empty;
                self.grid[new_row][new_col] = Cell::Robot;
                self.robot_pos = Position {
                    x: new_col,
                    y: new_row,
                };
            }
            Cell::BoxLeft | Cell::BoxRight => {
                if !self.can_push_wide_box(new_row, new_col, dx, dy) {
                    return;
                }

                self.push_wide_box(new_row, new_col, dx, dy);

                self.grid[self.robot_pos.y][self.robot_pos.x] = Cell::Empty;
                self.grid[new_row][new_col] = Cell::Robot;
                self.robot_pos = Position {
                    x: new_col,
                    y: new_row,
                };
            }
            _ => {} // Wall or other - do nothing
        }
    }

    fn can_push_box(&self, row: usize, col: usize, dx: i32, dy: i32) -> Option<Position> {
        let mut curr_row = row;
        let mut curr_col = col;

        while curr_row < self.grid.len() && curr_col < self.grid[0].len() {
            match self.grid[curr_row][curr_col] {
                Cell::Wall => return None,
                Cell::Empty => {
                    return Some(Position {
                        x: curr_col,
                        y: curr_row,
                    })
                }
                _ => {}
            }
            curr_col = (curr_col as i32 + dx) as usize;
            curr_row = (curr_row as i32 + dy) as usize;
        }
        None
    }

    fn can_push_wide_box(&self, row: usize, col: usize, dx: i32, dy: i32) -> bool {
        let curr_cell = self.grid[row][col];
        if curr_cell == Cell::Empty {
            return true;
        }
        if curr_cell == Cell::Wall {
            return false;
        }

        // Check if we're pushing horizontally
        if dx != 0 {
            let next_col = (col as i32 + (dx * 2)) as usize;
            return self.can_push_wide_box(row, next_col, dx, dy);
        } else {
            // For vertical movement, both parts must be able to move
            let partner_col = if curr_cell == Cell::BoxLeft {
                col + 1
            } else {
                col - 1
            };

            let next_row = (row as i32 + dy) as usize;
            return self.can_push_wide_box(next_row, partner_col, dx, dy)
                && self.can_push_wide_box(next_row, col, dx, dy);
        }
    }

    fn push_wide_box(&mut self, row: usize, col: usize, dx: i32, dy: i32) {
        let curr_cell = self.grid[row][col];

        if curr_cell == Cell::Empty {
            return;
        }

        // Check if we're pushing horizontally
        if dx != 0 {
            let next_col = (col as i32 + dx) as usize;
            self.push_wide_box(row, next_col, dx, dy);
            self.grid[row][next_col] = curr_cell;
        } else {
            // For vertical movement, both parts must move
            let partner_col = if curr_cell == Cell::BoxLeft {
                col + 1
            } else {
                col - 1
            };

            let next_row = (row as i32 + dy) as usize;

            self.push_wide_box(next_row, col, dx, dy);
            self.push_wide_box(next_row, partner_col, dx, dy);

            self.grid[next_row][col] = curr_cell;
            self.grid[next_row][partner_col] = self.grid[row][partner_col];

            self.grid[row][col] = Cell::Empty;
            self.grid[row][partner_col] = Cell::Empty;
        }
    }

    fn calculate_gps_sum(&self) -> i32 {
        let mut sum = 0;
        for (row, grid_row) in self.grid.iter().enumerate() {
            for (col, cell) in grid_row.iter().enumerate() {
                match cell {
                    Cell::Box => {
                        sum += (100 * row + col) as i32;
                    }
                    Cell::BoxLeft => {
                        sum += (100 * row + col) as i32;
                    }
                    _ => {}
                }
            }
        }
        sum
    }

    // clear the screen and print the grid
    fn draw(&self) {
        println!("\x1b[2J\x1b[1;1H");

        for row in &self.grid {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }
}

fn parse_input(input: &str, wide_boxes: bool) -> (Warehouse, String) {
    let mut parts = input.split("\n\n");
    let warehouse = Warehouse::new(parts.next().unwrap(), wide_boxes);
    let moves = parts
        .next()
        .unwrap()
        .chars()
        .filter(|c| "^v<>".contains(*c))
        .collect();

    (warehouse, moves)
}

fn part1(input: &str) -> i32 {
    let (mut warehouse, moves) = parse_input(input, false);

    for movement in moves.chars() {
        warehouse.try_move(movement);
    }

    warehouse.calculate_gps_sum()
}

fn part2(input: &str) -> i32 {
    let (mut warehouse, moves) = parse_input(input, true);

    for movement in moves.chars() {
        warehouse.try_move(movement);
        //warehouse.draw();
    }

    warehouse.calculate_gps_sum()
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
        assert_eq!(part1(&test_input), 10092);
    }

    #[test]
    fn test2_part1() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 2028);
    }

    #[test]
    fn test3_part1() {
        let test_input =
            fs::read_to_string("input/test3.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 1429911);
    }

    #[test]
    fn test1_part2() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 9021);
    }

    #[test]
    fn test2_part2() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 1751);
    }

    #[test]
    fn test3_part2() {
        let test_input =
            fs::read_to_string("input/test3.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 1453087);
    }
}
