const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
}

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start_str, end_str) = s
            .trim()
            .split_once('-')
            .ok_or_else(|| "Invalid range format: missing hyphen".to_string())?;

        let start = start_str
            .trim()
            .parse()
            .map_err(|_| format!("Invalid start number: {}", start_str))?;
        let end = end_str
            .trim()
            .parse()
            .map_err(|_| format!("Invalid end number: {}", end_str))?;

        Ok(Range { start, end })
    }
}

struct PatternGenerator {
    sub_len: u32,
    multiplier: i64,
}

impl PatternGenerator {
    fn new(len: u32, sub_len: u32) -> Self {
        let reps = len / sub_len;
        let shift = 10_i64.pow(sub_len);
        let mut multiplier = 0_i64;
        for _ in 0..reps {
            multiplier = multiplier.saturating_mul(shift).saturating_add(1);
        }
        Self {
            sub_len,
            multiplier,
        }
    }

    fn generate_in_range(&self, range: &Range, found: &mut HashSet<i64>) {
        // The pattern base P must be of length sub_len
        // P range: [10^(sub_len-1), 10^sub_len - 1]
        let p_min = 10_i64.pow(self.sub_len - 1);
        let p_max = 10_i64.pow(self.sub_len) - 1;

        // P must also satisfy: range.start <= P * multiplier <= range.end
        // Therefore:
        // P >= ceil(range.start / multiplier)
        // P <= floor(range.end / multiplier)
        let start_p = (range.start + self.multiplier - 1) / self.multiplier;
        let end_p = range.end / self.multiplier;

        let effective_min = p_min.max(start_p);
        let effective_max = p_max.min(end_p);

        if effective_min <= effective_max {
            for p in effective_min..=effective_max {
                found.insert(p * self.multiplier);
            }
        }
    }
}

trait LengthStrategy {
    fn sub_lengths(&self, len: u32) -> Vec<u32>;
}

struct HalfLengthStrategy;

impl LengthStrategy for HalfLengthStrategy {
    fn sub_lengths(&self, len: u32) -> Vec<u32> {
        if len.is_multiple_of(2) {
            vec![len / 2]
        } else {
            Vec::new()
        }
    }
}

struct AnyDivisorStrategy;

impl LengthStrategy for AnyDivisorStrategy {
    fn sub_lengths(&self, len: u32) -> Vec<u32> {
        (1..=len / 2).filter(|&sl| len.is_multiple_of(sl)).collect()
    }
}

fn solve(input: &str, strategy: impl LengthStrategy) -> i64 {
    let ranges: Vec<Range> = input
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut total_sum = 0;

    for range in ranges {
        let mut found = HashSet::new();
        let min_len = range.start.checked_ilog10().unwrap_or(0) + 1;
        let max_len = range.end.checked_ilog10().unwrap_or(0) + 1;

        for len in min_len..=max_len {
            for sub_len in strategy.sub_lengths(len) {
                let generator = PatternGenerator::new(len, sub_len);
                generator.generate_in_range(&range, &mut found);
            }
        }
        total_sum += found.iter().sum::<i64>();
    }

    total_sum
}

fn part1(input: &str) -> i64 {
    solve(input, HalfLengthStrategy)
}

fn part2(input: &str) -> i64 {
    solve(input, AnyDivisorStrategy)
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
        assert_eq!(part1(TEST_INPUT_1), 1227775554);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 38437576669);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 4174379265);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 49046150754);
    }
}
