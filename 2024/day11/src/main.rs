use std::collections::HashMap;
use std::fs;

fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .next()
        .unwrap_or("")
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

// If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
// If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
// If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
fn transform_single_stone(stone: i64) -> Vec<i64> {
    if stone == 0 {
        // Rule 1: 0 becomes 1
        vec![1]
    } else if stone.to_string().len().is_multiple_of(2) {
        // Rule 2: Even number of digits splits into two stones
        let num_digits = (stone as f64).log10().floor() as i32 + 1;
        let divisor = 10_i64.pow((num_digits / 2) as u32);
        let right_num = stone % divisor;
        let left_num = stone / divisor;
        vec![left_num, right_num]
    } else {
        // Rule 3: Multiply by 2024
        vec![stone * 2024]
    }
}

fn transform_stones(stones: Vec<i64>) -> Vec<i64> {
    stones
        .into_iter()
        .flat_map(transform_single_stone)
        .collect()
}

fn part1(input: &str) -> i32 {
    let mut stones = parse_input(input);

    // Blink 25 times
    for _ in 0..25 {
        stones = transform_stones(stones);
    }

    stones.len() as i32
}

fn count_stones(
    stone: i64,
    depth: usize,
    max: usize,
    memo: &mut HashMap<(i64, usize), i64>,
) -> i64 {
    if depth == max {
        return 1;
    }

    let key = (stone, depth);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    let stones = if stone == 0 {
        count_stones(1, depth + 1, max, memo)
    } else if stone.to_string().len().is_multiple_of(2) {
        let num_digits = (stone as f64).log10().floor() as u32 + 1;
        let divisor = 10_i64.pow(num_digits / 2);
        let right_num = stone % divisor;
        let left_num = stone / divisor;
        count_stones(left_num, depth + 1, max, memo) + count_stones(right_num, depth + 1, max, memo)
    } else {
        count_stones(stone * 2024, depth + 1, max, memo)
    };

    memo.insert(key, stones);
    stones
}

fn part2(input: &str) -> i64 {
    let stones = parse_input(input);
    let mut memo = HashMap::new();

    stones
        .iter()
        .map(|&stone| count_stones(stone, 0, 75, &mut memo))
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
        assert_eq!(part1(&test_input), 55312);
    }

    #[test]
    fn test2_part1() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 193269);
    }

    #[test]
    fn test1_part2() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 65601038650482);
    }

    #[test]
    fn test2_part2() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 228449040027793);
    }
}
