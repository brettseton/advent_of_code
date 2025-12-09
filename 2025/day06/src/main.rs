const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn apply(&self, numbers: &[i64]) -> i64 {
        match self {
            Operation::Add => numbers.iter().sum(),
            Operation::Multiply => numbers.iter().product(),
        }
    }
}

trait NumberParser {
    fn extract_numbers(&self, grid: &[Vec<char>]) -> Vec<i64>;
}

struct RowParser;
impl NumberParser for RowParser {
    fn extract_numbers(&self, grid: &[Vec<char>]) -> Vec<i64> {
        grid.iter()
            .filter_map(|row| {
                let s: String = row.iter().collect();
                s.trim().parse().ok()
            })
            .collect()
    }
}

struct ColumnParser;
impl NumberParser for ColumnParser {
    fn extract_numbers(&self, grid: &[Vec<char>]) -> Vec<i64> {
        if grid.is_empty() {
            return Vec::new();
        }

        let width = grid[0].len();
        (0..width)
            .rev()
            .filter_map(|x| {
                let s: String = grid
                    .iter()
                    .filter_map(|row| row.get(x).filter(|c| c.is_ascii_digit()))
                    .collect();
                if s.is_empty() {
                    None
                } else {
                    s.parse().ok()
                }
            })
            .collect()
    }
}

struct Problem {
    grid: Vec<Vec<char>>,
    op: Option<Operation>,
}

impl Problem {
    fn solve<P: NumberParser>(&self, parser: P) -> i64 {
        if let Some(op) = self.op {
            let numbers = parser.extract_numbers(&self.grid);
            op.apply(&numbers)
        } else {
            0
        }
    }
}

fn parse_problems(input: &str) -> Vec<Problem> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    if grid.is_empty() {
        return Vec::new();
    }
    let max_len = grid.iter().map(|r| r.len()).max().unwrap_or(0);

    // Use the last line to identify problem boundaries based on operators
    let last_row = grid.last().unwrap();
    let split_indices: Vec<usize> = last_row
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| if c == '+' || c == '*' { Some(i) } else { None })
        .collect();

    if split_indices.is_empty() || split_indices[0] != 0 {
        return Vec::new();
    }

    let mut problems = Vec::new();

    for i in 0..split_indices.len() {
        let start = split_indices[i];
        let end = if i + 1 < split_indices.len() {
            split_indices[i + 1]
        } else {
            max_len
        };

        if start >= max_len {
            continue;
        }

        let mut prob_grid = Vec::with_capacity(grid.len());
        for row in &grid {
            let mut sub_row = Vec::with_capacity(end - start);
            for x in start..end {
                sub_row.push(*row.get(x).unwrap_or(&' '));
            }
            prob_grid.push(sub_row);
        }

        let op = match grid.last().and_then(|row| row.get(start)) {
            Some('+') => Some(Operation::Add),
            Some('*') => Some(Operation::Multiply),
            _ => None,
        };

        problems.push(Problem {
            grid: prob_grid,
            op,
        });
    }

    problems
}

fn part1(input: &str) -> i64 {
    parse_problems(input)
        .iter()
        .map(|p| p.solve(RowParser))
        .sum()
}

fn part2(input: &str) -> i64 {
    parse_problems(input)
        .iter()
        .map(|p| p.solve(ColumnParser))
        .sum()
}

fn main() {
    println!("Part 1 test 1: {}", part1(TEST_INPUT_1));
    println!("Part 1 test 2: {}", part1(TEST_INPUT_2));

    println!("Part 2 test 1: {}", part2(TEST_INPUT_1));
    println!("Part 2 test 2: {}", part2(TEST_INPUT_2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() {
        assert_eq!(part1(TEST_INPUT_1), 4277556);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 7229350537438);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 3263827);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 11479269003550);
    }
}
