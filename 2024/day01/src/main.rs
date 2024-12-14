use std::fs;

fn part1(input: &str) -> i32 {
    0 // TODO: Implement part 1
}

fn part2(input: &str) -> i32 {
    0 // TODO: Implement part 2
}

fn main() {
    let input = fs::read_to_string("input/input.txt").expect("Should have been able to read the file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 0);
    }

    #[test]
    fn test_part2() {
        let test_input = fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 0);
    }
} 