const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

const PART1_SUBSEQUENCE_LEN: usize = 2;
const PART2_SUBSEQUENCE_LEN: usize = 12;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigitSequence<'a> {
    digits: &'a [u8],
}

impl<'a> DigitSequence<'a> {
    pub fn new(s: &'a str) -> Option<Self> {
        if s.bytes().all(|b| b.is_ascii_digit()) {
            Some(Self {
                digits: s.as_bytes(),
            })
        } else {
            None
        }
    }

    pub fn find_largest_subsequence(&self, k: usize) -> Option<u64> {
        let n = self.digits.len();
        if n < k {
            return None;
        }

        // We want to keep `k` elements, so we can remove `n - k` elements.
        let mut to_remove = n - k;
        let mut stack = Vec::with_capacity(k);

        for &digit in self.digits {
            // Maintain decreasing monotonic stack property where possible
            while to_remove > 0 {
                if let Some(&top) = stack.last() {
                    if digit > top {
                        stack.pop();
                        to_remove -= 1;
                        continue;
                    }
                }
                break;
            }
            stack.push(digit);
        }

        stack.truncate(k);

        let result = stack
            .iter()
            .fold(0u64, |acc, &d| acc * 10 + ((d - b'0') as u64));
        Some(result)
    }
}

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| {
            DigitSequence::new(line)
                .and_then(|seq| seq.find_largest_subsequence(PART1_SUBSEQUENCE_LEN))
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| {
            DigitSequence::new(line)
                .and_then(|seq| seq.find_largest_subsequence(PART2_SUBSEQUENCE_LEN))
        })
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
        assert_eq!(part1(TEST_INPUT_1), 357);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 17229);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 3121910778619);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 170520923035051);
    }
}
