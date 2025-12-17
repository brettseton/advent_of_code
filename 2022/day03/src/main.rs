use std::collections::HashSet;

const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

fn priority(c: char) -> i32 {
    match c {
        'a'..='z' => (c as i32) - ('a' as i32) + 1,
        'A'..='Z' => (c as i32) - ('A' as i32) + 27,
        _ => 0,
    }
}

struct Rucksack {
    compartment1: String,
    compartment2: String,
}

impl Rucksack {
    fn from_line(line: &str) -> Self {
        let mid = line.len() / 2;
        Self {
            compartment1: line[..mid].to_string(),
            compartment2: line[mid..].to_string(),
        }
    }

    fn find_common_item(&self) -> char {
        let set1: HashSet<char> = self.compartment1.chars().collect();
        self.compartment2
            .chars()
            .find(|c| set1.contains(c))
            .expect("No common item found between compartments")
    }

    fn priority(&self) -> i32 {
        priority(self.find_common_item())
    }
}

struct Group {
    rucksacks: Vec<Rucksack>,
}

impl Group {
    fn from_lines(lines: &[&str]) -> Self {
        Self {
            rucksacks: lines.iter().map(|line| Rucksack::from_line(line)).collect(),
        }
    }

    fn find_badge(&self) -> char {
        if self.rucksacks.is_empty() {
            panic!("Cannot find badge in empty group");
        }

        let mut candidates: HashSet<char> = self.rucksacks[0]
            .compartment1
            .chars()
            .chain(self.rucksacks[0].compartment2.chars())
            .collect();

        for rucksack in self.rucksacks.iter().skip(1) {
            let rucksack_chars: HashSet<char> = rucksack
                .compartment1
                .chars()
                .chain(rucksack.compartment2.chars())
                .collect();
            candidates.retain(|c| rucksack_chars.contains(c));
        }

        candidates
            .into_iter()
            .next()
            .expect("No common badge found in group")
    }

    fn badge_priority(&self) -> i32 {
        priority(self.find_badge())
    }
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| Rucksack::from_line(line).priority())
        .sum()
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|group| Group::from_lines(group).badge_priority())
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
        assert_eq!(part1(TEST_INPUT_1), 157);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 8123);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 70);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 2620);
    }
}
