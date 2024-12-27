use std::fs;

#[derive(Debug)]
struct Pattern {
    heights: Vec<i32>,
    is_lock: bool,
}

fn get_heights(pattern: &[String], is_lock: bool) -> Vec<i32> {
    let mut heights = vec![];
    let height = pattern.len() as i32 - 1;

    for col in 0..pattern[0].len() {
        if is_lock {
            // For locks, count from top down
            let mut pin_height = 0;
            for (row, line) in pattern.iter().enumerate() {
                if line.chars().nth(col).unwrap() == '#' {
                    pin_height = row as i32;
                }
            }
            heights.push(pin_height);
        } else {
            // For keys, count from bottom up
            let mut key_height = 0;
            for (row, line) in pattern.iter().enumerate().rev() {
                if line.chars().nth(col).unwrap() == '#' {
                    key_height = height - row as i32;
                }
            }
            heights.push(key_height);
        }
    }
    heights
}

fn parse_input(input: &str) -> Vec<Pattern> {
    let mut patterns = vec![];
    let mut current_pattern: Vec<String> = vec![];

    for line in input.lines() {
        if line.is_empty() && !current_pattern.is_empty() {
            let is_lock = current_pattern[0].chars().all(|c| c == '#');
            let heights = get_heights(&current_pattern[..], is_lock);
            patterns.push(Pattern { heights, is_lock });
            current_pattern.clear();
        } else if !line.is_empty() {
            current_pattern.push(line.to_string());
        }
    }

    if !current_pattern.is_empty() {
        let is_lock = current_pattern[0].chars().all(|c| c == '#');
        let heights = get_heights(&current_pattern[..], is_lock);
        patterns.push(Pattern { heights, is_lock });
    }

    patterns
}

fn can_fit(lock: &[i32], key: &[i32]) -> bool {
    lock.iter().zip(key.iter()).all(|(l, k)| l + k < 6)
}

fn part1(input: &str) -> i32 {
    let patterns = parse_input(input);
    let locks: Vec<_> = patterns.iter().filter(|p| p.is_lock).collect();
    let keys: Vec<_> = patterns.iter().filter(|p| !p.is_lock).collect();

    let mut valid_pairs = 0;
    for lock in locks {
        for key in &keys {
            if can_fit(&lock.heights, &key.heights) {
                valid_pairs += 1;
            }
        }
    }

    valid_pairs
}

fn part2(input: &str) -> i32 {
    let _data = parse_input(input);
    0 // TODO: Implement part 2
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
        assert_eq!(part1(&test_input), 3);
    }

    #[test]
    fn test2_part1() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 3608);
    }

    #[test]
    fn test1_part2() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 0);
    }

    #[test]
    fn test2_part2() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 0);
    }
}
