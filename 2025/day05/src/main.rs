const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

#[derive(Debug, Clone, Copy)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn new(start: i64, end: i64) -> Result<Self, String> {
        if start > end {
            return Err(format!(
                "Range start {} cannot be greater than end {}",
                start, end
            ));
        }
        Ok(Self { start, end })
    }

    fn contains(&self, id: i64) -> bool {
        id >= self.start && id <= self.end
    }

    fn len(&self) -> i64 {
        self.end - self.start + 1
    }

    fn intersects(&self, other: &Range) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    fn merge(&self, other: &Range) -> Range {
        Range::new(self.start.min(other.start), self.end.max(other.end))
            .expect("Merging valid ranges should always produce a valid range")
    }
}

#[derive(Debug)]
struct MergedRanges {
    ranges: Vec<Range>,
}

impl From<Vec<Range>> for MergedRanges {
    fn from(mut ranges: Vec<Range>) -> Self {
        ranges.sort_by_key(|r| r.start);

        let mut merged: Vec<Range> = Vec::new();
        for range in ranges {
            if let Some(last) = merged.last_mut() {
                if last.intersects(&range) {
                    *last = last.merge(&range);
                } else {
                    merged.push(range);
                }
            } else {
                merged.push(range);
            }
        }
        Self { ranges: merged }
    }
}

impl MergedRanges {
    fn contains(&self, id: i64) -> bool {
        self.ranges.iter().any(|r| r.contains(id))
    }

    fn total_len(&self) -> i64 {
        self.ranges.iter().map(|r| r.len()).sum()
    }
}

#[derive(Debug)]
struct ParsedInput {
    ranges: Vec<Range>,
    ids: Vec<i64>,
}

fn parse_input(input: &str) -> Option<ParsedInput> {
    let mut lines = input.lines();

    let ranges: Vec<Range> = lines
        .by_ref()
        .take_while(|line| !line.trim().is_empty())
        .filter_map(|line| {
            let (start, end) = line.split_once('-')?;
            Range::new(start.trim().parse().ok()?, end.trim().parse().ok()?).ok()
        })
        .collect();

    if ranges.is_empty() {
        return None;
    }

    let ids: Vec<i64> = lines.filter_map(|line| line.trim().parse().ok()).collect();

    Some(ParsedInput { ranges, ids })
}

fn part1(input: &str) -> i32 {
    let parsed = parse_input(input).expect("Failed to parse input");
    let merged_ranges = MergedRanges::from(parsed.ranges);

    parsed
        .ids
        .iter()
        .filter(|&&id| merged_ranges.contains(id))
        .count() as i32
}

fn part2(input: &str) -> i64 {
    let parsed = parse_input(input).expect("Failed to parse input");
    let merged_ranges = MergedRanges::from(parsed.ranges);

    merged_ranges.total_len()
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
        assert_eq!(part1(TEST_INPUT_1), 3);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 577);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 14);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 350513176552950);
    }
}
