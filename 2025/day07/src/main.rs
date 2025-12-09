const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

fn part1(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    if grid.is_empty() {
        return 0;
    }

    // Find starting position (S)
    let mut start_col = 0;
    for line in grid.iter() {
        for (col, &ch) in line.iter().enumerate() {
            if ch == 'S' {
                start_col = col;
                break;
            }
        }
    }

    // Track active beam positions (column indices)
    let mut active_beams = vec![start_col];
    let mut split_count = 0;

    // Process each row starting from row 1 (below S)
    for row_data in grid.iter().skip(1) {
        let mut new_beams = Vec::new();

        for &col in &active_beams {
            if col < row_data.len() {
                match row_data[col] {
                    '.' => {
                        // Beam continues straight down
                        new_beams.push(col);
                    }
                    '^' => {
                        // Beam splits: create two new beams at left and right
                        split_count += 1;
                        if col > 0 {
                            new_beams.push(col - 1);
                        }
                        if col < row_data.len() - 1 {
                            new_beams.push(col + 1);
                        }
                    }
                    _ => {
                        // Unknown character, beam stops
                    }
                }
            }
        }

        // Remove duplicates (beams that merge at the same position)
        new_beams.sort();
        new_beams.dedup();
        active_beams = new_beams;
    }

    split_count
}

fn part2(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    if grid.is_empty() {
        return 0;
    }

    // Find starting position (S)
    let mut start_col = 0;
    for line in grid.iter() {
        for (col, &ch) in line.iter().enumerate() {
            if ch == 'S' {
                start_col = col;
                break;
            }
        }
    }

    // Track the number of timelines at each position (column)
    // Key: column index, Value: number of timelines at that position
    let mut timeline_counts: std::collections::HashMap<usize, i64> =
        std::collections::HashMap::new();
    timeline_counts.insert(start_col, 1);

    // Process each row starting from row 1 (below S)
    for row_data in grid.iter().skip(1) {
        let mut new_counts: std::collections::HashMap<usize, i64> =
            std::collections::HashMap::new();

        for (&col, &count) in &timeline_counts {
            if col < row_data.len() {
                match row_data[col] {
                    '.' => {
                        // All timelines continue straight down
                        *new_counts.entry(col).or_insert(0) += count;
                    }
                    '^' => {
                        // Each timeline splits into two: left and right
                        if col > 0 {
                            *new_counts.entry(col - 1).or_insert(0) += count;
                        }
                        if col < row_data.len() - 1 {
                            *new_counts.entry(col + 1).or_insert(0) += count;
                        }
                    }
                    _ => {
                        // Unknown character, timelines end
                    }
                }
            }
        }

        timeline_counts = new_counts;
    }

    // Sum all timelines across all final positions
    timeline_counts.values().sum::<i64>()
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
        assert_eq!(part1(TEST_INPUT_1), 21);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 1541);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 40);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 80158285728929);
    }
}
