const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Operation {
    Add(i64),
    Mul(i64),
    Square,
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let op_str = s
            .strip_prefix("Operation: new = old ")
            .ok_or("Invalid op prefix")?;
        let parts: Vec<&str> = op_str.split_whitespace().collect();

        match (parts.first().copied(), parts.get(1).copied()) {
            (Some("*"), Some("old")) => Ok(Self::Square),
            (Some("*"), Some(n)) => n
                .parse()
                .map(Self::Mul)
                .map_err(|_| "Invalid mul arg".to_string()),
            (Some("+"), Some(n)) => n
                .parse()
                .map(Self::Add)
                .map_err(|_| "Invalid add arg".to_string()),
            _ => Err(format!("Unknown operation format: {}", s)),
        }
    }
}

impl Operation {
    fn apply(&self, old: i64) -> i64 {
        match self {
            Self::Add(n) => old + n,
            Self::Mul(n) => old * n,
            Self::Square => old * old,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<i64>,
    operation: Operation,
    test_divisor: i64,
    true_target: usize,
    false_target: usize,
    inspection_count: usize,
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(block: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = block.lines().map(|l| l.trim()).collect();
        if lines.len() < 6 {
            return Err("Incomplete monkey block".to_string());
        }

        let items = lines[1]
            .strip_prefix("Starting items: ")
            .ok_or("Missing items line")?
            .split(", ")
            .map(|s| s.parse().map_err(|_| "Invalid item".to_string()))
            .collect::<Result<VecDeque<_>, _>>()?;

        let operation = lines[2].parse()?;
        let test_divisor = lines[3]
            .strip_prefix("Test: divisible by ")
            .ok_or("Missing test")?
            .parse()
            .map_err(|_| "Invalid divisor")?;
        let true_target = lines[4]
            .strip_prefix("If true: throw to monkey ")
            .ok_or("Missing true target")?
            .parse()
            .map_err(|_| "Invalid true target")?;
        let false_target = lines[5]
            .strip_prefix("If false: throw to monkey ")
            .ok_or("Missing false target")?
            .parse()
            .map_err(|_| "Invalid false target")?;

        Ok(Self {
            items,
            operation,
            test_divisor,
            true_target,
            false_target,
            inspection_count: 0,
        })
    }
}

trait ReliefStrategy {
    fn apply(&self, worry: i64, common_multiple: i64) -> i64;
}

struct DivideByThree;
impl ReliefStrategy for DivideByThree {
    fn apply(&self, worry: i64, common_multiple: i64) -> i64 {
        (worry / 3) % common_multiple
    }
}

struct NoRelief;
impl ReliefStrategy for NoRelief {
    fn apply(&self, worry: i64, common_multiple: i64) -> i64 {
        worry % common_multiple
    }
}

struct MonkeySimulation {
    monkeys: Vec<Monkey>,
    common_multiple: i64,
}

impl MonkeySimulation {
    fn try_new(input: &str) -> Result<Self, String> {
        let monkeys = input
            .trim()
            .split("\n\n")
            .map(|block| block.parse::<Monkey>())
            .collect::<Result<Vec<_>, _>>()?;
        let common_multiple = monkeys.iter().map(|m| m.test_divisor).product();
        Ok(Self {
            monkeys,
            common_multiple,
        })
    }

    fn run<S: ReliefStrategy>(mut self, rounds: usize, strategy: S) -> Self {
        for _ in 0..rounds {
            for i in 0..self.monkeys.len() {
                // Take items out to avoid borrow checker issues during the turn
                let mut items = std::mem::take(&mut self.monkeys[i].items);
                self.monkeys[i].inspection_count += items.len();

                while let Some(item) = items.pop_front() {
                    let worry =
                        strategy.apply(self.monkeys[i].operation.apply(item), self.common_multiple);

                    let target = if worry % self.monkeys[i].test_divisor == 0 {
                        self.monkeys[i].true_target
                    } else {
                        self.monkeys[i].false_target
                    };

                    self.monkeys[target].items.push_back(worry);
                }
            }
        }
        self
    }

    fn monkey_business(&self) -> i64 {
        let mut counts: Vec<_> = self.monkeys.iter().map(|m| m.inspection_count).collect();
        counts.sort_unstable_by(|a, b| b.cmp(a));
        counts.iter().take(2).map(|&c| c as i64).product()
    }
}

fn part1(input: &str) -> i64 {
    MonkeySimulation::try_new(input)
        .expect("Failed to parse input")
        .run(20, DivideByThree)
        .monkey_business()
}

fn part2(input: &str) -> i64 {
    MonkeySimulation::try_new(input)
        .expect("Failed to parse input")
        .run(10000, NoRelief)
        .monkey_business()
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
        assert_eq!(part1(TEST_INPUT_1), 10605);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 78960);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 2713310158);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 14561971968);
    }
}
