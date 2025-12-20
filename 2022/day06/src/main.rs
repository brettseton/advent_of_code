const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

fn find_marker(input: &str, size: usize) -> Option<usize> {
    let bytes = input.as_bytes();
    let mut last_seen = [None; 256];
    let mut start = 0;

    for (i, &byte) in bytes.iter().enumerate() {
        if let Some(prev_pos) = last_seen[byte as usize] {
            if prev_pos >= start {
                start = prev_pos + 1;
            }
        }
        last_seen[byte as usize] = Some(i);

        if i + 1 - start >= size {
            return Some(i + 1);
        }
    }
    None
}

fn part1(input: &str) -> usize {
    find_marker(input, 4).expect("Should find a start-of-packet marker")
}

fn part2(input: &str) -> usize {
    find_marker(input, 14).expect("Should find a start-of-message marker")
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
    fn test_part1_examples() {
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test1_part1() {
        assert_eq!(part1(TEST_INPUT_1), 7);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 1300);
    }

    #[test]
    fn test_part2_examples() {
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 19);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 3986);
    }
}
