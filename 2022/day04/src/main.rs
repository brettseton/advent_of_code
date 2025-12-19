const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn from_str(s: &str) -> Self {
        let parts: Vec<&str> = s.split('-').collect();
        Range {
            start: parts[0].parse().unwrap(),
            end: parts[1].parse().unwrap(),
        }
    }

    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

fn part1(input: &str) -> i32 {
    parse_pairs(input)
        .filter(|(r1, r2)| r1.contains(r2) || r2.contains(r1))
        .count() as i32
}

fn part2(input: &str) -> i32 {
    parse_pairs(input)
        .filter(|(r1, r2)| r1.overlaps(r2))
        .count() as i32
}

fn parse_pairs(input: &str) -> impl Iterator<Item = (Range, Range)> + '_ {
    input.lines().filter(|line| !line.is_empty()).map(|line| {
        let ranges: Vec<&str> = line.split(',').collect();
        let r1 = Range::from_str(ranges[0]);
        let r2 = Range::from_str(ranges[1]);
        (r1, r2)
    })
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
