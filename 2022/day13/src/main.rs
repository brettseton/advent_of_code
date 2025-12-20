const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Int(l), Self::Int(r)) => l.cmp(r),
            (Self::List(l), Self::List(r)) => l.cmp(r),
            (Self::Int(l), Self::List(_)) => Self::List(vec![Self::Int(*l)]).cmp(other),
            (Self::List(_), Self::Int(r)) => self.cmp(&Self::List(vec![Self::Int(*r)])),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Packet, String> {
            match chars.peek() {
                Some('[') => {
                    chars.next();
                    let mut list = Vec::new();
                    while let Some(&c) = chars.peek() {
                        if c == ']' {
                            chars.next();
                            return Ok(Packet::List(list));
                        }
                        if c == ',' {
                            chars.next();
                        } else {
                            list.push(parse(chars)?);
                        }
                    }
                    Err("Incomplete list: missing closing bracket".into())
                }
                Some(c) if c.is_ascii_digit() => {
                    let mut n = 0;
                    while let Some(d) = chars.peek().and_then(|c| c.to_digit(10)) {
                        n = n * 10 + d;
                        chars.next();
                    }
                    Ok(Packet::Int(n))
                }
                Some(c) => Err(format!("Unexpected character: {}", c)),
                None => Err("Unexpected end of input".into()),
            }
        }
        parse(&mut s.chars().peekable())
    }
}

struct PacketPair<'a> {
    index: usize,
    left: &'a Packet,
    right: &'a Packet,
}

impl<'a> PacketPair<'a> {
    fn new(index: usize, left: &'a Packet, right: &'a Packet) -> Self {
        Self { index, left, right }
    }

    fn is_ordered_correctly(&self) -> bool {
        self.left <= self.right
    }
}

struct TwoSixDecoder;

impl TwoSixDecoder {
    fn decode(&self, packets: &[Packet]) -> usize {
        let d2 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
        let d6 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);

        let i2 = packets.iter().filter(|&p| p < &d2).count() + 1;
        let i6 = packets.iter().filter(|&p| p < &d6).count() + 2;

        i2 * i6
    }
}

struct PairOrderValidator;

impl PairOrderValidator {
    fn sum_ordered_indices(&self, packets: &[Packet]) -> usize {
        packets
            .chunks(2)
            .enumerate()
            .filter_map(|(i, chunk)| {
                let pair = PacketPair::new(i + 1, &chunk[0], &chunk[1]);
                pair.is_ordered_correctly().then_some(pair.index)
            })
            .sum()
    }
}

struct Signal {
    packets: Vec<Packet>,
}

impl Signal {
    fn packets(&self) -> &[Packet] {
        &self.packets
    }
}

impl FromStr for Signal {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let packets = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.parse::<Packet>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { packets })
    }
}

fn part1(input: &str) -> usize {
    let signal = Signal::from_str(input).expect("Signal input should be well-formed");
    PairOrderValidator.sum_ordered_indices(signal.packets())
}

fn part2(input: &str) -> usize {
    let signal = Signal::from_str(input).expect("Signal input should be well-formed");
    TwoSixDecoder.decode(signal.packets())
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
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 13);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 5808);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_1), 140);
    }
    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 22713);
    }
}
