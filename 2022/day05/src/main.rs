const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

trait Crane {
    fn input(&self) -> &str;
    fn move_crates(&self, stacks: &mut [Vec<char>], instruction: &Instruction);

    fn solve(&self) -> String {
        let parts: Vec<&str> = self.input().split("\n\n").collect();
        if parts.len() < 2 {
            return String::new();
        }

        if let Some(mut ship) = Ship::parse(parts[0]) {
            let instructions = Instruction::parse_multiple(parts[1]);
            for instruction in instructions {
                self.move_crates(&mut ship.stacks, &instruction);
            }
            return ship.get_top_crates();
        }

        String::new()
    }
}

struct CrateMover9000<'a>(&'a str);
impl<'a> CrateMover9000<'a> {
    fn new(input: &'a str) -> Self {
        Self(input)
    }
}
impl<'a> Crane for CrateMover9000<'a> {
    fn input(&self) -> &str {
        self.0
    }

    fn move_crates(&self, stacks: &mut [Vec<char>], instruction: &Instruction) {
        for _ in 0..instruction.count {
            if let Some(c) = stacks[instruction.from].pop() {
                stacks[instruction.to].push(c);
            }
        }
    }
}

struct CrateMover9001<'a>(&'a str);
impl<'a> CrateMover9001<'a> {
    fn new(input: &'a str) -> Self {
        Self(input)
    }
}
impl<'a> Crane for CrateMover9001<'a> {
    fn input(&self) -> &str {
        self.0
    }

    fn move_crates(&self, stacks: &mut [Vec<char>], instruction: &Instruction) {
        let from_stack = &mut stacks[instruction.from];
        let drain_start = from_stack.len().saturating_sub(instruction.count);
        let crates: Vec<char> = from_stack.drain(drain_start..).collect();
        stacks[instruction.to].extend(crates);
    }
}

struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn parse_multiple(input: &str) -> Vec<Self> {
        input.lines().filter_map(Self::parse).collect()
    }

    fn parse(line: &str) -> Option<Self> {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.len() < 6 {
            return None;
        }

        let count = words[1].parse().ok()?;
        let from = words[3].parse::<usize>().ok()?.checked_sub(1)?;
        let to = words[5].parse::<usize>().ok()?.checked_sub(1)?;

        Some(Self { count, from, to })
    }
}

struct Ship {
    stacks: Vec<Vec<char>>,
}

impl Ship {
    fn parse(diagram: &str) -> Option<Self> {
        let lines: Vec<&str> = diagram.lines().collect();
        let last_line = lines.last()?;

        let num_stacks = last_line
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .max()?;

        let mut stacks = vec![Vec::new(); num_stacks];

        for line in lines.iter().rev().skip(1) {
            for (i, stack) in stacks.iter_mut().enumerate() {
                let char_idx = 1 + i * 4;
                if let Some(c) = line.chars().nth(char_idx) {
                    if c.is_alphabetic() {
                        stack.push(c);
                    }
                }
            }
        }

        Some(Self { stacks })
    }

    fn get_top_crates(&self) -> String {
        self.stacks.iter().filter_map(|s| s.last()).collect()
    }
}

fn part1(input: &str) -> String {
    CrateMover9000::new(input).solve()
}

fn part2(input: &str) -> String {
    CrateMover9001::new(input).solve()
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
        assert_eq!(part1(TEST_INPUT_1), "CMZ");
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), "WCZTHTMPS");
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), "MCD");
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), "BLSGJSDTS");
    }
}
