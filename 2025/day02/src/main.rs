const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

use std::str::FromStr;

struct Range {
    start: i64,
    end: i64,
}

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .trim()
            .split_once('-')
            .ok_or_else(|| "Invalid range format".to_string())?;

        let start = start.trim().parse().map_err(|_| "Invalid start number")?;
        let end = end.trim().parse().map_err(|_| "Invalid end number")?;

        Ok(Range { start, end })
    }
}

fn write_digits(mut n: i64, buf: &mut [u8; 20]) -> &[u8] {
    if n == 0 {
        buf[19] = b'0';
        return &buf[19..];
    }
    let mut i = 20;
    while n > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    &buf[i..]
}

fn has_repeated_half(n: i64) -> bool {
    let mut buf = [0u8; 20];
    let bytes = write_digits(n, &mut buf);
    let len = bytes.len();

    if !len.is_multiple_of(2) {
        return false;
    }
    let mid = len / 2;
    bytes[..mid] == bytes[mid..]
}

fn has_repeated_substring(n: i64) -> bool {
    let mut buf = [0u8; 20];
    let bytes = write_digits(n, &mut buf);
    let len = bytes.len();

    if len < 2 {
        return false;
    }

    (1..=len / 2)
        .any(|sub_len| len.is_multiple_of(sub_len) && bytes[..len - sub_len] == bytes[sub_len..])
}

fn solve(input: &str, predicate: fn(i64) -> bool) -> i64 {
    input
        .split(',')
        .filter_map(|s| s.parse::<Range>().ok())
        .flat_map(|range| range.start..=range.end)
        .filter(|&n| predicate(n))
        .sum()
}

fn part1(input: &str) -> i64 {
    solve(input, has_repeated_half)
}

fn part2(input: &str) -> i64 {
    solve(input, has_repeated_substring)
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
