use std::fs;
#[derive(Debug)]
struct Expression {
    target: i64,
    numbers: Vec<i64>,
}

fn parse_line(line: &str) -> Expression {
    let parts: Vec<&str> = line.split(':').collect();
    let target = parts[0].trim().parse().unwrap();
    let numbers: Vec<i64> = parts[1]
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    Expression {
        target: target,
        numbers: numbers,
    }
}

fn can_make_target_part1(expr: &Expression) -> bool {
    fn recurse(target: i64, nums: &[i64]) -> bool {
        if nums.len() == 1 {
            return nums[0] == target;
        }

        let last = nums[nums.len() - 1];
        let remaining = &nums[..nums.len() - 1];

        if target % last == 0 && recurse(target / last, remaining) {
            return true;
        }

        if target > last && recurse(target - last, remaining) {
            return true;
        }

        false
    }
    recurse(expr.target, &expr.numbers)
}

fn can_make_target_part2(expr: &Expression) -> bool {
    fn recurse(target: i64, nums: &[i64]) -> bool {
        if nums.len() == 1 {
            return nums[0] == target;
        }

        let last = nums[nums.len() - 1];
        let remaining = &nums[..nums.len() - 1];

        if target % last == 0 && recurse(target / last, remaining) {
            return true;
        }

        let mut divisor = 1i64;
        let mut temp = last;
        loop {
            divisor *= 10;
            temp /= 10;
            if temp == 0 {
                break;
            }
        }

        if target > last && (target - last) % divisor == 0 && recurse(target / divisor, remaining) {
            return true;
        }

        if target > last && recurse(target - last, remaining) {
            return true;
        }

        false
    }
    recurse(expr.target, &expr.numbers)
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let expr = parse_line(line);
            if can_make_target_part1(&expr) {
                expr.target
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let expr = parse_line(line);
            if can_make_target_part2(&expr) {
                expr.target
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let input1 =
        fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
    let input2 =
        fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
    println!("Part 1 test 1: {}", part1(&input1));
    println!("Part 1 test 2: {}", part1(&input2));

    println!("Part 2 test 1: {}", part2(&input1));
    println!("Part 2 test 2: {}", part2(&input2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 3749);
    }

    #[test]
    fn test2_part1() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 4122618559853);
    }

    #[test]
    fn test1_part2() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 11387);
    }

    #[test]
    fn test2_part2() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 227615740238334);
    }
}
