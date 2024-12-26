use std::fs;

fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}

fn mix(secret: i64, value: i64) -> i64 {
    secret ^ value
}

fn prune(secret: i64) -> i64 {
    secret % 16777216
}

fn generate_next_secret(secret: i64) -> i64 {
    let mut result = secret;

    // Step 1: multiply by 64
    result = mix(result, result * 64);
    result = prune(result);

    // Step 2: divide by 32
    result = mix(result, result / 32);
    result = prune(result);

    // Step 3: multiply by 2048
    result = mix(result, result * 2048);
    result = prune(result);

    result
}

fn generate_nth_secret(initial: i64, n: usize) -> i64 {
    let mut secret = initial;
    for _ in 0..n {
        secret = generate_next_secret(secret);
    }
    secret
}

fn part1(input: &str) -> i64 {
    let initial_secrets = parse_input(input);
    initial_secrets
        .iter()
        .map(|&secret| generate_nth_secret(secret, 2000))
        .sum()
}

fn part2(input: &str) -> i32 {
    let numbers = parse_input(input);
    let mut sequence_price_sums = vec![0; 130321]; // Stores accumulated prices for each sequence pattern
    let mut seen = vec![u16::MAX; 130321];

    // Process each number in the input
    for (id, &number) in numbers.iter().enumerate() {
        let id = id as u16;
        let number = number as usize;

        let zeroth = number;
        let first = hash(zeroth);
        let second = hash(first);
        let third = hash(second);

        let mut a;
        let mut b = to_index(zeroth, first);
        let mut c = to_index(first, second);
        let mut d = to_index(second, third);

        let mut number = third;
        let mut previous = third % 10;

        // Process the sequence
        for _ in 3..2000 {
            number = hash(number);
            let price = number % 10;

            (a, b, c, d) = (b, c, d, 9 + price - previous);
            let index = 6859 * a + 361 * b + 19 * c + d;

            if seen[index] != id {
                sequence_price_sums[index] += price as u16;
                seen[index] = id;
            }

            previous = price;
        }
    }

    // Find the maximum sum
    *sequence_price_sums.iter().max().unwrap() as i32
}

fn hash(mut n: usize) -> usize {
    n = (n ^ (n << 6)) & 0xffffff;
    n = (n ^ (n >> 5)) & 0xffffff;
    (n ^ (n << 11)) & 0xffffff
}

/// Convert -9..9 to 0..18.
fn to_index(previous: usize, current: usize) -> usize {
    9 + current % 10 - previous % 10
}

fn main() {
    let input1 =
        fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
    let input2 =
        fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
    let input3 =
        fs::read_to_string("input/test3.txt").expect("Should have been able to read the file");

    println!("Part 1 test 1: {}", part1(&input1));
    println!("Part 1 test 2: {}", part1(&input2));

    println!("Part 2 test 1: {}", part2(&input1));
    println!("Part 2 test 2: {}", part2(&input2));
    println!("Part 2 test 3: {}", part2(&input3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 37327623);
    }

    #[test]
    fn test2_part1() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 16039090236);
    }

    #[test]
    fn test1_part2() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 24);
    }

    #[test]
    fn test2_part2() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 1808);
    }

    #[test]
    fn test3_part2() {
        let test_input =
            fs::read_to_string("input/test3.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 23);
    }
}
