const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

fn parse_sorted_calories(input: &str) -> Vec<i32> {
    let mut calories: Vec<i32> = input
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|elf_inventory| {
            elf_inventory
                .lines()
                .map(|line| line.parse::<i32>().unwrap_or(0))
                .sum()
        })
        .collect();

    calories.sort_unstable_by(|a, b| b.cmp(a));
    calories
}

fn part1(input: &str) -> i32 {
    parse_sorted_calories(input).first().copied().unwrap_or(0)
}

fn part2(input: &str) -> i32 {
    parse_sorted_calories(input).iter().take(3).sum()
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
    fn test1_part1() {
        assert_eq!(part1(TEST_INPUT_1), 24000);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 69501);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 45000);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 202346);
    }
}
