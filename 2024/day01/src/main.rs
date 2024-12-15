use std::fs;

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let mut numbers = line.split_whitespace().map(|n| n.parse::<i32>().unwrap());

        left.push(numbers.next().unwrap());
        right.push(numbers.next().unwrap());
    }

    (left, right)
}

fn part1(input: &str) -> i32 {
    let (mut left, mut right) = parse_input(input);

    // Sort both lists
    left.sort();
    right.sort();

    // Calculate total distance between paired numbers
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn part2(input: &str) -> i32 {
    let (left, right) = parse_input(input);

    // Calculate similarity score
    left.iter()
        .map(|&num| {
            // Count how many times this number appears in right list
            let count = right.iter().filter(|&&x| x == num).count() as i32;
            // Multiply number by its count in right list
            num * count
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
        assert_eq!(part1(&test_input), 11);
    }

    #[test]
    fn test2_part1() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 2086478);
    }

    #[test]
    fn test1_part2() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 31);
    }

    #[test]
    fn test2_part2() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 24941624);
    }
}
