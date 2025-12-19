use std::fmt;
use std::str::FromStr;

const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

use std::marker::PhantomData;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct SectionId<Role>(u32, PhantomData<Role>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct StartRole;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct EndRole;

type StartId = SectionId<StartRole>;
type EndId = SectionId<EndRole>;

impl<Role> fmt::Display for SectionId<Role> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<Role> SectionId<Role> {
    fn new(val: u32) -> Self {
        SectionId(val, PhantomData)
    }
}

// Special comparison logic to allow comparing Start with End
impl PartialEq<SectionId<EndRole>> for SectionId<StartRole> {
    fn eq(&self, other: &SectionId<EndRole>) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd<SectionId<EndRole>> for SectionId<StartRole> {
    fn partial_cmp(&self, other: &SectionId<EndRole>) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl PartialEq<SectionId<StartRole>> for SectionId<EndRole> {
    fn eq(&self, other: &SectionId<StartRole>) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd<SectionId<StartRole>> for SectionId<EndRole> {
    fn partial_cmp(&self, other: &SectionId<StartRole>) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Range {
    start: StartId,
    end: EndId,
}

#[derive(Debug)]
enum ParseRangeError {
    InvalidFormat(String),
    InvalidBound(String, std::num::ParseIntError),
    InvertedRange(StartId, EndId),
}

impl fmt::Display for ParseRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat(s) => {
                write!(f, "Invalid range format (expected 'start-end'): {}", s)
            }
            Self::InvalidBound(s, e) => write!(f, "Failed to parse bound '{}': {}", s, e),
            Self::InvertedRange(s, e) => {
                write!(f, "Range start ({}) is greater than end ({})", s, e)
            }
        }
    }
}

impl std::error::Error for ParseRangeError {}

impl FromStr for Range {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start_s, end_s) = s
            .split_once('-')
            .ok_or_else(|| ParseRangeError::InvalidFormat(s.to_string()))?;

        let start_val = start_s
            .parse::<u32>()
            .map_err(|e| ParseRangeError::InvalidBound(start_s.to_string(), e))?;
        let end_val = end_s
            .parse::<u32>()
            .map_err(|e| ParseRangeError::InvalidBound(end_s.to_string(), e))?;

        let start = StartId::new(start_val);
        let end = EndId::new(end_val);

        if start.0 > end.0 {
            return Err(ParseRangeError::InvertedRange(start, end));
        }

        Ok(Range { start, end })
    }
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        // Here we compare Start with Start, and End with End
        self.start.0 <= other.start.0 && self.end.0 >= other.end.0
    }

    fn overlaps(&self, other: &Range) -> bool {
        // Here we compare Start with End, enabled by our cross-type PartialOrd
        self.start <= other.end && self.end >= other.start
    }
}

fn part1(input: &str) -> usize {
    parse_pairs(input)
        .expect("Failed to parse input")
        .filter(|(r1, r2)| r1.contains(r2) || r2.contains(r1))
        .count()
}

fn part2(input: &str) -> usize {
    parse_pairs(input)
        .expect("Failed to parse input")
        .filter(|(r1, r2)| r1.overlaps(r2))
        .count()
}

fn parse_pairs(
    input: &str,
) -> Result<impl Iterator<Item = (Range, Range)> + '_, Box<dyn std::error::Error>> {
    let mut results = Vec::new();
    for line in input.lines().filter(|l| !l.is_empty()) {
        let (r1_s, r2_s) = line
            .split_once(',')
            .ok_or_else(|| format!("Invalid line format: {}", line))?;
        let r1 = Range::from_str(r1_s)?;
        let r2 = Range::from_str(r2_s)?;
        results.push((r1, r2));
    }
    Ok(results.into_iter())
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
        assert_eq!(part1(TEST_INPUT_1), 2);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 540);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 4);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 872);
    }
}
