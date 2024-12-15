use std::fs;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn check_direction(grid: &[Vec<char>], row: i32, col: i32, dx: i32, dy: i32) -> bool {
    let target = "XMAS".chars().collect::<Vec<char>>();
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    for i in 0..4 {
        let new_row = row + i * dy;
        let new_col = col + i * dx;

        if new_row < 0 || new_row >= rows || new_col < 0 || new_col >= cols {
            return false;
        }

        if grid[new_row as usize][new_col as usize] != target[i as usize] {
            return false;
        }
    }
    true
}

fn check_xmas(grid: &[Vec<char>], row: i32, col: i32) -> bool {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    // Check if we have enough space for an X pattern
    if row < 1 || row >= rows - 1 || col < 1 || col >= cols - 1 {
        return false;
    }

    // Center must be 'A'
    if grid[row as usize][col as usize] != 'A' {
        return false;
    }

    // Check all possible combinations of MAS and SAM in X shape
    let patterns = [
        // MAS, MAS
        [(-1, -1, 'M'), (1, 1, 'S'), (1, -1, 'M'), (-1, 1, 'S')],
        // MAS, SAM
        [(-1, -1, 'M'), (1, 1, 'S'), (1, -1, 'S'), (-1, 1, 'M')],
        // SAM, MAS
        [(-1, -1, 'S'), (1, 1, 'M'), (1, -1, 'M'), (-1, 1, 'S')],
        // SAM, SAM
        [(-1, -1, 'S'), (1, 1, 'M'), (1, -1, 'S'), (-1, 1, 'M')],
    ];

    for pattern in patterns.iter() {
        let mut valid = true;
        for &(dy, dx, expected) in pattern {
            let new_row = row + dy;
            let new_col = col + dx;
            if grid[new_row as usize][new_col as usize] != expected {
                valid = false;
                break;
            }
        }
        if valid {
            return true;
        }
    }
    false
}

fn part1(input: &str) -> i32 {
    let grid = parse_input(input);
    let mut count = 0;
    let directions = [
        (0, 1),   // right
        (1, 0),   // down
        (1, 1),   // diagonal down-right
        (-1, 1),  // diagonal up-right
        (0, -1),  // left
        (-1, 0),  // up
        (-1, -1), // diagonal up-left
        (1, -1),  // diagonal down-left
    ];

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            for &(dy, dx) in &directions {
                if check_direction(&grid, row as i32, col as i32, dx, dy) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn part2(input: &str) -> i32 {
    let grid = parse_input(input);
    let mut count = 0;

    for row in 1..grid.len() - 1 {
        for col in 1..grid[0].len() - 1 {
            if check_xmas(&grid, row as i32, col as i32) {
                count += 1;
            }
        }
    }
    count
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
        assert_eq!(part1(&test_input), 18);
    }

    #[test]
    fn test2_part1() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 2603);
    }

    #[test]
    fn test1_part2() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 9);
    }

    #[test]
    fn test2_part2() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 1965);
    }
}
