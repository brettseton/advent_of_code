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

fn concatenate(a: i64, b: i64) -> i64 {
    let b_str = b.to_string();
    let result = format!("{}{}", a, b_str);
    result.parse().unwrap()
}

fn evaluate(nums: &[i64], ops: &[char]) -> i64 {
    let mut result = nums[0];
    for i in 0..ops.len() {
        match ops[i] {
            '+' => result += nums[i + 1],
            '*' => result *= nums[i + 1],
            '|' => result = concatenate(result, nums[i + 1]),
            _ => panic!("Invalid operator"),
        }
    }
    result
}

fn can_make_target_part1(expr: &Expression) -> bool {
    let num_ops = expr.numbers.len() - 1;
    let total_combinations = 1 << num_ops; // 2^num_ops combinations

    // Try all possible combinations of operators
    for i in 0..total_combinations {
        let mut ops = Vec::new();
        for j in 0..num_ops {
            // Use bit j of i to determine operator
            if (i & (1 << j)) == 0 {
                ops.push('+');
            } else {
                ops.push('*');
            }
        }
        if evaluate(&expr.numbers, &ops) == expr.target {
            return true;
        }
    }
    false
}

fn can_make_target_part2(expr: &Expression) -> bool {
    let num_ops = expr.numbers.len() - 1;
    let total_combinations = 3_i32.pow(num_ops as u32); // 3^num_ops combinations for 3 operators

    // Pre-calculate powers of 3 to avoid repeated division
    let mut powers = vec![1; num_ops];
    for i in 1..num_ops {
        powers[i] = powers[i - 1] * 3;
    }

    // Try all possible combinations of operators
    for i in 0..total_combinations {
        let mut current_result = expr.numbers[0];

        // Try operators one at a time and check if result exceeds target
        let mut valid = true;
        for (j, power) in powers.iter().enumerate().take(num_ops) {
            let op = match (i / power) % 3 {
                0 => '+',
                1 => '*',
                2 => '|',
                _ => unreachable!(),
            };

            let next_num = expr.numbers[j + 1];

            current_result = match op {
                '+' => {
                    let result = current_result + next_num;
                    if result > expr.target {
                        valid = false;
                        break;
                    }
                    result
                }
                '*' => {
                    let result = current_result * next_num;
                    if result > expr.target {
                        valid = false;
                        break;
                    }
                    result
                }
                '|' => {
                    let result = concatenate(current_result, next_num);
                    if result > expr.target {
                        valid = false;
                        break;
                    }
                    result
                }
                _ => unreachable!(),
            };
        }

        if valid && current_result == expr.target {
            return true;
        }
    }
    false
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
