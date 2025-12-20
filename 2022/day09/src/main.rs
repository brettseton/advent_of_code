use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");
const TEST_INPUT_3: &str = include_str!("../input/test3.txt");

#[derive(Debug)]
enum RopeError {
    InvalidDirection(String),
    InvalidStepCount(String),
    EmptyRope,
    ParsingError(String),
}

impl fmt::Display for RopeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDirection(s) => write!(f, "Invalid direction: {}", s),
            Self::InvalidStepCount(s) => write!(f, "Invalid step count: {}", s),
            Self::EmptyRope => write!(f, "Rope must have at least one knot"),
            Self::ParsingError(s) => write!(f, "Failed to parse input: {}", s),
        }
    }
}

impl std::error::Error for RopeError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_in(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

trait FollowStrategy {
    fn follow(&self, leader: &Point, follower: &mut Point);
}

struct StandardFollowStrategy;

impl FollowStrategy for StandardFollowStrategy {
    fn follow(&self, leader: &Point, follower: &mut Point) {
        let dx = leader.x - follower.x;
        let dy = leader.y - follower.y;

        if dx.abs() > 1 || dy.abs() > 1 {
            follower.x += dx.signum();
            follower.y += dy.signum();
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = RopeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(RopeError::InvalidDirection(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    direction: Direction,
    steps: usize,
}

impl FromStr for Instruction {
    type Err = RopeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let dir_str = parts
            .next()
            .ok_or_else(|| RopeError::ParsingError("Missing direction".into()))?;
        let steps_str = parts
            .next()
            .ok_or_else(|| RopeError::ParsingError("Missing steps".into()))?;

        let direction = dir_str.parse()?;
        let steps = steps_str
            .parse()
            .map_err(|_| RopeError::InvalidStepCount(steps_str.to_string()))?;

        Ok(Instruction { direction, steps })
    }
}

struct Rope<'a, S: FollowStrategy> {
    knots: Vec<Point>,
    instructions: &'a [Instruction],
    strategy: S,
}

impl Rope<'static, StandardFollowStrategy> {
    fn new(num_knots: usize) -> Self {
        Self {
            knots: vec![Point::default(); num_knots],
            instructions: &[],
            strategy: StandardFollowStrategy,
        }
    }
}

impl<'a, S: FollowStrategy> Rope<'a, S> {
    fn add_instructions(self, instructions: &'a [Instruction]) -> Self {
        Self {
            knots: self.knots,
            instructions,
            strategy: self.strategy,
        }
    }

    fn simulate(&mut self) -> Result<i32, RopeError> {
        if self.knots.is_empty() {
            return Err(RopeError::EmptyRope);
        }

        let mut visited = HashSet::new();
        let tail_idx = self.knots.len() - 1;
        visited.insert(self.knots[tail_idx]);

        for instruction in self.instructions {
            for _ in 0..instruction.steps {
                self.knots[0].move_in(instruction.direction);

                for i in 1..self.knots.len() {
                    let leader = self.knots[i - 1];
                    self.strategy.follow(&leader, &mut self.knots[i]);
                }
                visited.insert(self.knots[tail_idx]);
            }
        }

        Ok(visited.len() as i32)
    }
}

fn parse_instructions(input: &str) -> Result<Vec<Instruction>, RopeError> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse())
        .collect()
}

fn solve(input: &str, num_knots: usize) -> Result<i32, RopeError> {
    let instructions = parse_instructions(input)?;
    Rope::new(num_knots)
        .add_instructions(&instructions)
        .simulate()
}

fn part1(input: &str) -> i32 {
    solve(input, 2).unwrap_or(0)
}

fn part2(input: &str) -> i32 {
    solve(input, 10).unwrap_or(0)
}

fn main() {
    println!("Part 1 test 1: {}", part1(TEST_INPUT_1));
    println!("Part 1 test 2: {}", part1(TEST_INPUT_2));
    println!("Part 1 test 3: {}", part1(TEST_INPUT_3));

    println!("Part 2 test 1: {}", part2(TEST_INPUT_1));
    println!("Part 2 test 2: {}", part2(TEST_INPUT_2));
    println!("Part 2 test 3: {}", part2(TEST_INPUT_3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() {
        assert_eq!(part1(TEST_INPUT_1), 13);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 6081);
    }

    #[test]
    fn test3_part1() {
        assert_eq!(part1(TEST_INPUT_3), 88);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 1);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 2487);
    }

    #[test]
    fn test3_part2() {
        assert_eq!(part2(TEST_INPUT_3), 36);
    }
}
