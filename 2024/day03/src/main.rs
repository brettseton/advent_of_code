use regex::Regex;
use std::fs;

fn part1(input: &str) -> u32 {
    find_mul_instructions(input)
        .into_iter()
        .map(|(x, y)| x * y)
        .sum()
}

fn part2(input: &str) -> u32 {
    find_mul_instructions_with_state(input)
        .into_iter()
        .map(|(x, y)| x * y)
        .sum()
}

fn find_mul_instructions(input: &str) -> Vec<(u32, u32)> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let x = cap[1].parse::<u32>().unwrap();
            let y = cap[2].parse::<u32>().unwrap();
            (x, y)
        })
        .collect()
}

fn find_mul_instructions_with_state(input: &str) -> Vec<(u32, u32)> {
    let mut instructions = Vec::new();
    let mut enabled = true; // Start with multiplications enabled
    let mut pos = 0;

    let re = Regex::new(r"^mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    while pos < input.len() {
        let remaining = &input[pos..];

        if remaining.starts_with("do()") {
            enabled = true;
            pos += 4;
            continue;
        }

        if remaining.starts_with("don't()") {
            enabled = false;
            pos += 7;
            continue;
        }

        if enabled && remaining.starts_with("mul") {
            if let Some(cap) = re.find(remaining) {
                if let Some(captures) = re.captures(cap.as_str()) {
                    let x = captures[1].parse::<u32>().unwrap();
                    let y = captures[2].parse::<u32>().unwrap();
                    instructions.push((x, y));
                }
                pos += cap.end() - cap.start();
                continue;
            }
        }

        pos += 1;
    }

    instructions
}

fn main() {
    let input1 =
        fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
    let input2 =
        fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
    let input3 =
        fs::read_to_string("input/test3.txt").expect("Should have been able to read the file");

    println!("Part 1 test1: {}", part1(&input1));
    println!("Part 1 test2: {}", part1(&input2));

    println!("Part 2 test1: {}", part2(&input1));
    println!("Part 2 test2: {}", part2(&input2));
    println!("Part 2 test3: {}", part2(&input3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() {
        let input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&input), 161);
    }

    #[test]
    fn test2_part1() {
        let input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&input), 174103751);
    }

    #[test]
    fn test1_part2() {
        let input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&input), 161);
    }

    #[test]
    fn test2_part2() {
        let input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&input), 100411201);
    }

    #[test]
    fn test3_part2() {
        let input =
            fs::read_to_string("input/test3.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&input), 48);
    }
}
