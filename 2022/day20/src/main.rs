const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

const DECRYPTION_KEY: i64 = 811_589_153;

fn solve(input: &str, key: i64, rounds: usize) -> i64 {
    let numbers: Vec<i64> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim().parse::<i64>().unwrap() * key)
        .collect();

    let n = numbers.len();
    let mut list: Vec<(usize, i64)> = numbers.into_iter().enumerate().collect();

    for _ in 0..rounds {
        for i in 0..n {
            let current_pos = list
                .iter()
                .position(|&(original_idx, _)| original_idx == i)
                .unwrap();

            let item = list.remove(current_pos);
            let val = item.1;

            let new_pos = (current_pos as i64 + val).rem_euclid(n as i64 - 1);
            list.insert(new_pos as usize, item);
        }
    }

    let zero_pos = list.iter().position(|&(_, val)| val == 0).unwrap();

    let v1 = list[(zero_pos + 1000) % n].1;
    let v2 = list[(zero_pos + 2000) % n].1;
    let v3 = list[(zero_pos + 3000) % n].1;

    v1 + v2 + v3
}

fn part1(input: &str) -> i64 {
    solve(input, 1, 1)
}

fn part2(input: &str) -> i64 {
    solve(input, DECRYPTION_KEY, 10)
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
        assert_eq!(part1(TEST_INPUT_1), 3);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 10707);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 1623178306);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 2488332343098);
    }
}
