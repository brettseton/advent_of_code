use std::fs;

struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn new(levels: Vec<i32>) -> Self {
        Report { levels }
    }

    fn is_safe(&self) -> bool {
        if self.levels.len() < 2 {
            return true;
        }

        // Check if sequence is increasing or decreasing
        let mut increasing = None;

        for window in self.levels.windows(2) {
            let diff = window[1] - window[0];

            // Check if difference is between 1 and 3 (inclusive)
            if diff.abs() < 1 || diff.abs() > 3 {
                return false;
            }

            match increasing {
                None => {
                    // First pair determines if sequence should be increasing or decreasing
                    increasing = Some(diff > 0);
                }
                Some(should_increase) => {
                    // Check if direction matches what we expect
                    if (diff > 0) != should_increase {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn can_be_made_safe(&self) -> bool {
        // If it's already safe, no need to remove anything
        if self.is_safe() {
            return true;
        }

        // Try removing each level one at a time
        for i in 0..self.levels.len() {
            let mut modified = self.levels.clone();
            modified.remove(i);
            let modified_report = Report::new(modified);
            if modified_report.is_safe() {
                return true;
            }
        }

        false
    }
}

struct ReportAnalyzer {
    reports: Vec<Report>,
}

impl ReportAnalyzer {
    fn from_input(input: &str) -> Self {
        let reports = input
            .lines()
            .map(|line| {
                let levels = line
                    .split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect();
                Report::new(levels)
            })
            .collect();
        ReportAnalyzer { reports }
    }

    fn count_safe_reports(&self) -> usize {
        self.reports
            .iter()
            .filter(|report| report.is_safe())
            .count()
    }

    fn count_potentially_safe_reports(&self) -> usize {
        self.reports
            .iter()
            .filter(|report| report.can_be_made_safe())
            .count()
    }
}

fn part1(input: &str) -> usize {
    let analyzer = ReportAnalyzer::from_input(input);
    analyzer.count_safe_reports()
}

fn part2(input: &str) -> usize {
    let analyzer = ReportAnalyzer::from_input(input);
    analyzer.count_potentially_safe_reports()
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
        assert_eq!(part1(&test_input), 2);
    }

    #[test]
    fn test2_part1() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 326);
    }

    #[test]
    fn test1_part2() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 4);
    }

    #[test]
    fn test2_part2() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 381);
    }
}
