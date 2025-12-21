use std::collections::HashMap;
use std::str::FromStr;

const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

const ROOT: &str = "root";
const HUMN: &str = "humn";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Monkey(String);

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl AsRef<str> for Monkey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl TryFrom<char> for Operator {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '+' => Ok(Self::Add),
            '-' => Ok(Self::Sub),
            '*' => Ok(Self::Mul),
            '/' => Ok(Self::Div),
            _ => Err(format!("Unknown operator: {}", c)),
        }
    }
}

impl Operator {
    fn apply(&self, left: i64, right: i64) -> i64 {
        match self {
            Self::Add => left + right,
            Self::Sub => left - right,
            Self::Mul => left * right,
            Self::Div => left / right,
        }
    }

    fn inverse_left(&self, target: i64, right_val: i64) -> i64 {
        match self {
            Self::Add => target - right_val,
            Self::Sub => target + right_val,
            Self::Mul => target / right_val,
            Self::Div => target * right_val,
        }
    }

    fn inverse_right(&self, target: i64, left_val: i64) -> i64 {
        match self {
            Self::Add => target - left_val,
            Self::Sub => left_val - target,
            Self::Mul => target / left_val,
            Self::Div => left_val / target,
        }
    }
}

enum Job {
    Number(i64),
    Operation(Monkey, Operator, Monkey),
}

impl FromStr for Job {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = s.parse::<i64>() {
            Ok(Job::Number(num))
        } else {
            let parts: Vec<&str> = s.split_whitespace().collect();
            if parts.len() != 3 {
                return Err(format!("Invalid job format: {}", s));
            }
            let left = Monkey::from(parts[0]);
            let op = Operator::try_from(parts[1].chars().next().ok_or("Empty operator")?)?;
            let right = Monkey::from(parts[2]);
            Ok(Job::Operation(left, op, right))
        }
    }
}

struct MonkeyEntry {
    name: Monkey,
    job: Job,
}

impl FromStr for MonkeyEntry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(": ").collect();
        if parts.len() != 2 {
            return Err(format!("Invalid line format: {}", s));
        }
        let name = Monkey::from(parts[0]);
        let job = Job::from_str(parts[1])?;
        Ok(Self { name, job })
    }
}

struct Monkeys(HashMap<Monkey, Job>);

impl FromStr for Monkeys {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| {
                let entry = MonkeyEntry::from_str(l)?;
                Ok((entry.name, entry.job))
            })
            .collect::<Result<HashMap<_, _>, String>>()?;
        Ok(Self(map))
    }
}

impl Monkeys {
    fn evaluate(&self, name: &Monkey) -> i64 {
        match self
            .0
            .get(name)
            .unwrap_or_else(|| panic!("Monkey not found: {}", name.as_ref()))
        {
            Job::Number(n) => *n,
            Job::Operation(left, op, right) => {
                let left_val = self.evaluate(left);
                let right_val = self.evaluate(right);
                op.apply(left_val, right_val)
            }
        }
    }

    fn has_humn(&self, name: &Monkey) -> bool {
        if name.as_ref() == HUMN {
            return true;
        }
        match self
            .0
            .get(name)
            .unwrap_or_else(|| panic!("Monkey not found: {}", name.as_ref()))
        {
            Job::Number(_) => false,
            Job::Operation(left, _, right) => self.has_humn(left) || self.has_humn(right),
        }
    }

    fn solve_for(&self, name: &Monkey, target: i64) -> i64 {
        if name.as_ref() == HUMN {
            return target;
        }

        match self
            .0
            .get(name)
            .unwrap_or_else(|| panic!("Monkey not found: {}", name.as_ref()))
        {
            Job::Number(_) => panic!(
                "Reached constant while solving for human path: {}",
                name.as_ref()
            ),
            Job::Operation(left, op, right) => {
                if self.has_humn(left) {
                    let right_val = self.evaluate(right);
                    let new_target = op.inverse_left(target, right_val);
                    self.solve_for(left, new_target)
                } else {
                    let left_val = self.evaluate(left);
                    let new_target = op.inverse_right(target, left_val);
                    self.solve_for(right, new_target)
                }
            }
        }
    }

    fn get_yell_number(&self) -> i64 {
        let (left, right) = match self
            .0
            .get(&Monkey::from(ROOT))
            .expect("Root monkey not found")
        {
            Job::Operation(l, _, r) => (l, r),
            _ => panic!("Root monkey must have an operation job"),
        };

        if self.has_humn(left) {
            let target = self.evaluate(right);
            self.solve_for(left, target)
        } else {
            let target = self.evaluate(left);
            self.solve_for(right, target)
        }
    }
}

fn part1(input: &str) -> i64 {
    let monkeys = Monkeys::from_str(input).expect("Failed to parse monkeys");
    monkeys.evaluate(&Monkey::from(ROOT))
}

fn part2(input: &str) -> i64 {
    let monkeys = Monkeys::from_str(input).expect("Failed to parse monkeys");
    monkeys.get_yell_number()
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
        assert_eq!(part1(TEST_INPUT_1), 152);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 268597611536314);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 301);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 3451534022348);
    }
}
