use std::str::FromStr;

const DIAL_START: i32 = 50;
const DIAL_CIRCUMFERENCE: i32 = 100;

const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    pub direction: Direction,
    pub distance: u32,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err("Empty instruction".to_string());
        }

        // Safer parsing using split_at instead of direct indexing
        let (dir_str, dist_str) = s.split_at(1);

        let direction = match dir_str {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(format!("Unknown direction: {}", dir_str)),
        };

        let distance = dist_str
            .parse::<u32>()
            .map_err(|_| format!("Invalid distance: {}", dist_str))?;

        Ok(Instruction {
            direction,
            distance,
        })
    }
}

pub trait Rotatable {
    fn rotate(&mut self, instruction: &Instruction);
    fn reading(&self) -> i32;
}

#[derive(Debug, Clone)]
pub struct Dial {
    position: i32,
    circumference: i32,
}

impl Dial {
    pub fn new(start: i32, circumference: i32) -> Self {
        Self {
            position: start,
            circumference,
        }
    }

    pub fn circumference(&self) -> i32 {
        self.circumference
    }
}

impl Rotatable for Dial {
    fn rotate(&mut self, instruction: &Instruction) {
        // Use i64 to prevent overflow during calculation if distance is large
        let pos = self.position as i64;
        let circ = self.circumference as i64;
        let dist = instruction.distance as i64;

        let new_pos = match instruction.direction {
            Direction::Left => pos - dist,
            Direction::Right => pos + dist,
        };

        self.position = new_pos.rem_euclid(circ) as i32;
    }

    fn reading(&self) -> i32 {
        self.position
    }
}

#[derive(Debug, Clone)]
pub struct CrossingCountingDial {
    inner: Dial,
    crossings: i32,
}

impl CrossingCountingDial {
    pub fn new(start: i32, circumference: i32) -> Self {
        Self {
            inner: Dial::new(start, circumference),
            crossings: 0,
        }
    }

    pub fn crossings(&self) -> i32 {
        self.crossings
    }
}

impl Rotatable for CrossingCountingDial {
    fn rotate(&mut self, instruction: &Instruction) {
        let start_pos = self.inner.reading() as i64;
        let circumference = self.inner.circumference() as i64;
        let dist = instruction.distance as i64;

        // Calculate crossings and target position together
        let (target, crossings) = match instruction.direction {
            Direction::Left => {
                let target = start_pos - dist;
                // Count multiples of circumference in [target, start_pos - 1]
                let start_idx = start_pos - 1;
                let end_idx = target - 1;
                let count = start_idx.div_euclid(circumference) - end_idx.div_euclid(circumference);
                (target, count)
            }
            Direction::Right => {
                let target = start_pos + dist;
                // Count multiples of circumference in (start_pos, target]
                let count = target.div_euclid(circumference) - start_pos.div_euclid(circumference);
                (target, count)
            }
        };

        self.crossings += crossings as i32;
        self.inner.position = target.rem_euclid(circumference) as i32;
    }

    fn reading(&self) -> i32 {
        self.inner.reading()
    }
}

fn part1(input: &str) -> i32 {
    let mut dial = Dial::new(DIAL_START, DIAL_CIRCUMFERENCE);
    let mut zero_count = 0;

    for instruction in input.lines().flat_map(|l| l.parse::<Instruction>()) {
        dial.rotate(&instruction);

        if dial.reading() == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

fn part2(input: &str) -> i32 {
    let mut dial = CrossingCountingDial::new(DIAL_START, DIAL_CIRCUMFERENCE);

    for instruction in input.lines().flat_map(|l| l.parse::<Instruction>()) {
        dial.rotate(&instruction);
    }

    dial.crossings()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Part 1 test 1: {}", part1(TEST_INPUT_1));
    println!("Part 1 test 2: {}", part1(TEST_INPUT_2));

    println!("Part 2 test 1: {}", part2(TEST_INPUT_1));
    println!("Part 2 test 2: {}", part2(TEST_INPUT_2));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() {
        assert_eq!(part1(TEST_INPUT_1), 3);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 1132);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 6);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 6623);
    }
}
