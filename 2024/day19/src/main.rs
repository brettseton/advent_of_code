use std::collections::HashMap;
use std::fs;

fn part1(input: &str) -> i32 {
    let mut lines = input.lines();
    let patterns: Vec<&str> = lines.next().unwrap().split(", ").collect(); // Available towel patterns
    let designs: Vec<&str> = lines.skip(1).collect(); // Desired designs

    designs
        .iter()
        .filter(|&&design| {
            let mut memo = HashMap::new();
            can_form_design(design, &patterns, &mut memo)
        })
        .count() as i32
}

fn can_form_design(design: &str, patterns: &[&str], memo: &mut HashMap<String, bool>) -> bool {
    if design.is_empty() {
        return true;
    }

    if let Some(&result) = memo.get(design) {
        return result;
    }

    for pattern in patterns {
        if let Some(remaining_design) = design.strip_prefix(pattern) {
            if can_form_design(remaining_design, patterns, memo) {
                memo.insert(design.to_string(), true);
                return true;
            }
        }
    }

    memo.insert(design.to_string(), false);
    false
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let patterns: Vec<&str> = lines.next().unwrap().split(", ").collect(); // Available towel patterns
    let designs: Vec<&str> = lines.skip(1).collect(); // Desired designs

    designs
        .iter()
        .map(|&design| {
            let mut memo = HashMap::new();
            count_ways(design, &patterns, &mut memo)
        })
        .sum()
}

fn count_ways(design: &str, patterns: &[&str], memo: &mut HashMap<String, u64>) -> u64 {
    if design.is_empty() {
        return 1; // An empty design can be formed in one way (by using no patterns)
    }

    if let Some(&result) = memo.get(design) {
        return result;
    }

    let total_ways = patterns
        .iter()
        .filter_map(|&pattern| {
            design
                .strip_prefix(pattern)
                .map(|remaining_design| count_ways(remaining_design, patterns, memo))
        })
        .sum();

    memo.insert(design.to_string(), total_ways);
    total_ways
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
        assert_eq!(part1(&test_input), 6);
    }

    #[test]
    fn test2_part1() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 306);
    }

    #[test]
    fn test1_part2() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 16);
    }

    #[test]
    fn test2_part2() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 604622004681855);
    }
}
