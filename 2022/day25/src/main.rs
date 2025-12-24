const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

fn part1(input: &str) -> String {
    let sum: i64 = input.lines().map(snafu_to_decimal).sum();
    decimal_to_snafu(sum)
}

fn snafu_to_decimal(snafu: &str) -> i64 {
    snafu.chars().fold(0, |acc, c| {
        acc * 5
            + match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => panic!("Invalid SNAFU digit '{}' in input '{}'", c, snafu),
            }
    })
}

fn decimal_to_snafu(mut n: i64) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let mut res = String::new();
    while n > 0 {
        let (digit, carry) = match n % 5 {
            0 => ('0', 0),
            1 => ('1', 0),
            2 => ('2', 0),
            3 => ('=', 1),
            4 => ('-', 1),
            _ => unreachable!(),
        };
        res.push(digit);
        n = n / 5 + carry;
    }
    res.chars().rev().collect()
}

fn main() {
    println!("Part 1 test 1: {}", part1(TEST_INPUT_1));
    println!("Part 1 test 2: {}", part1(TEST_INPUT_2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() {
        assert_eq!(part1(TEST_INPUT_1), "2=-1=0");
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), "2-2--02=1---1200=0-1");
    }
}
