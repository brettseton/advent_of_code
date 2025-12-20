const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

struct Grid {
    trees: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let trees: Vec<Vec<u8>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .filter_map(|c| c.to_digit(10).map(|d| d as u8))
                    .collect()
            })
            .collect();
        let rows = trees.len();
        let cols = if rows > 0 { trees[0].len() } else { 0 };
        Self { trees, rows, cols }
    }

    fn height_at(&self, r: usize, c: usize) -> u8 {
        self.trees[r][c]
    }

    fn is_visible(&self, r: usize, c: usize) -> bool {
        let h = self.height_at(r, c);
        let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        directions.iter().any(|&(dr, dc)| {
            let mut curr_r = r as isize + dr;
            let mut curr_c = c as isize + dc;

            while curr_r >= 0
                && curr_r < self.rows as isize
                && curr_c >= 0
                && curr_c < self.cols as isize
            {
                if self.height_at(curr_r as usize, curr_c as usize) >= h {
                    return false;
                }
                curr_r += dr;
                curr_c += dc;
            }
            true
        })
    }

    fn scenic_score(&self, r: usize, c: usize) -> i32 {
        let h = self.height_at(r, c);
        let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        directions
            .iter()
            .map(|&(dr, dc)| {
                let mut count = 0;
                let mut curr_r = r as isize + dr;
                let mut curr_c = c as isize + dc;

                while curr_r >= 0
                    && curr_r < self.rows as isize
                    && curr_c >= 0
                    && curr_c < self.cols as isize
                {
                    count += 1;
                    if self.height_at(curr_r as usize, curr_c as usize) >= h {
                        break;
                    }
                    curr_r += dr;
                    curr_c += dc;
                }
                count
            })
            .product()
    }

    fn get_max_scenic_score(&self) -> i32 {
        let mut max_score = 0;
        for r in 0..self.rows {
            for c in 0..self.cols {
                let score = self.scenic_score(r, c);
                if score > max_score {
                    max_score = score;
                }
            }
        }
        max_score
    }

    fn count_visible_trees(&self) -> i32 {
        let mut count = 0;
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.is_visible(r, c) {
                    count += 1;
                }
            }
        }
        count
    }
}

fn part1(input: &str) -> i32 {
    let grid = Grid::new(input);
    grid.count_visible_trees()
}

fn part2(input: &str) -> i32 {
    let grid = Grid::new(input);
    grid.get_max_scenic_score()
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
        assert_eq!(part1(TEST_INPUT_2), 1851);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 8);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 574080);
    }
}
