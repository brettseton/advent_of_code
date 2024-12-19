use std::fs;

#[derive(Debug)]
struct ClawMachine {
    button_a: Point,
    button_b: Point,
    prize: Point,
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn parse_input(input: &str) -> Vec<ClawMachine> {
    input
        .split("\n\n")
        .map(|machine| {
            let lines: Vec<&str> = machine.lines().collect();
            let parse_coords = |line: &str| {
                let parts: Vec<&str> = line.split(", ").collect();
                let x = if parts[0].contains("+") {
                    parts[0].split("+").nth(1).unwrap()
                } else {
                    parts[0].split("=").nth(1).unwrap()
                }
                .parse::<i64>()
                .unwrap();

                let y = if parts[1].contains("+") {
                    parts[1].split("+").nth(1).unwrap()
                } else {
                    parts[1].split("=").nth(1).unwrap()
                }
                .parse::<i64>()
                .unwrap();

                Point { x, y }
            };

            ClawMachine {
                button_a: parse_coords(lines[0].strip_prefix("Button A: ").unwrap()),
                button_b: parse_coords(lines[1].strip_prefix("Button B: ").unwrap()),
                prize: parse_coords(lines[2].strip_prefix("Prize: ").unwrap()),
            }
        })
        .collect()
}

fn is_solvable_large(machine: &ClawMachine) -> Option<(i64, i64)> {
    let numerator = (machine.button_b.y * machine.prize.x) - (machine.button_b.x * machine.prize.y);
    let denominator =
        (machine.button_a.x * machine.button_b.y) - (machine.button_a.y * machine.button_b.x);

    let x_remainder = numerator % denominator;

    if x_remainder != 0 {
        return None;
    }

    let a_presses = numerator / denominator;
    let b_presses = (machine.prize.x - (a_presses * machine.button_a.x)) / machine.button_b.x;

    let y_result = a_presses * machine.button_a.y + b_presses * machine.button_b.y;
    if y_result == machine.prize.y {
        Some((a_presses, b_presses))
    } else {
        None
    }
}

fn calculate_tokens(a_presses: i64, b_presses: i64) -> i64 {
    a_presses * 3 + b_presses
}

fn part1(input: &str) -> i64 {
    let machines = parse_input(input);
    let mut total_tokens = 0;

    for machine in machines {
        if let Some((a_presses, b_presses)) = is_solvable_large(&machine) {
            total_tokens += calculate_tokens(a_presses, b_presses);
        }
    }

    total_tokens
}

fn part2(input: &str) -> i64 {
    let machines = parse_input(input);
    let mut total_tokens = 0;
    let offset = 10000000000000i64;

    for mut machine in machines {
        machine.prize.x += offset;
        machine.prize.y += offset;

        if let Some((a_presses, b_presses)) = is_solvable_large(&machine) {
            total_tokens += calculate_tokens(a_presses, b_presses);
        }
    }

    total_tokens
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
        assert_eq!(part1(&test_input), 480);
    }

    #[test]
    fn test2_part1() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 39996);
    }

    #[test]
    fn test1_part2() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 875318608908);
    }

    #[test]
    fn test2_part2() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 73267584326867);
    }
}
