use std::{fs, str::FromStr};

fn main() {
    let ans = part1("input/test1.txt");
    println!("part 1 test 1 : {}", ans);

    let ans = part1("input/test2.txt");
    println!("part 1 test 2 : {}", ans);

    let ans = part2("input/test1.txt");
    println!("part 2 test 1 : {}", ans);

    let ans = part2("input/test2.txt");
    println!("part 2 test 2 : {}", ans);
}

fn part1(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let condition_report = ConditionReport::new(&input);
    return condition_report.get_arrangements();
}

fn part2(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let condition_report = ConditionReport::new(&input);
    let unfolded_report = condition_report.unfold();
    return unfolded_report.get_arrangements();
}

struct ConditionReport {
    rows: Vec<ConditionRecord>,
}

impl ConditionReport {
    pub fn new(str: &str) -> ConditionReport {
        return ConditionReport::from_str(str).expect("");
    }

    pub fn unfold(&self) -> ConditionReport {
        let rows: Vec<ConditionRecord> = self.rows.iter().map(|x| x.unfold()).collect();
        return ConditionReport { rows };
    }

    pub fn get_arrangements(&self) -> usize {
        return self.rows.iter().map(|x| x.get_arrangements(false)).sum();
    }
}

#[derive(Debug)]
struct ConditionReportError;

impl FromStr for ConditionReport {
    type Err = ConditionReportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s.lines().map(|x| ConditionRecord::new(x)).collect();
        return Ok(ConditionReport { rows });
    }
}

struct ConditionRecord {
    springs: String,
    sizes: Vec<usize>,
}

impl ConditionRecord {
    pub fn new(str: &str) -> ConditionRecord {
        return ConditionRecord::from_str(str).expect("");
    }

    pub fn unfold(&self) -> ConditionRecord {
        let unfolded_springs = (0..5)
            .map(|_| self.springs.clone())
            .collect::<Vec<_>>()
            .join("?");

        let unfolded_sizes = (0..5).flat_map(|_| self.sizes.iter().cloned()).collect();
        return ConditionRecord {
            springs: unfolded_springs,
            sizes: unfolded_sizes,
        };
    }

    pub fn get_arrangements(&self, use_dynamic_solution: bool) -> usize {

        if use_dynamic_solution {
            return ConditionRecord::get_arrangement_count_dp(&self.springs, &self.sizes);
        }

        let mut lookup = vec![vec![None; self.sizes.len()]; self.springs.len()];
        return ConditionRecord::get_arrangement_count(
            &self.springs,
            &self.sizes,
            0,
            0,
            &mut lookup,
        );
    }

    pub fn get_arrangement_count(
        springs: &str,
        sizes: &[usize],
        start_index: usize,
        size_index: usize,
        lookup: &mut Vec<Vec<Option<usize>>>,
    ) -> usize {
        let mut count = 0;

        if size_index == sizes.len() {
            if (start_index..springs.len()).any(|i| springs.chars().nth(i).unwrap() == '#') {
                return 0;
            } else {
                return 1;
            }
        }

        if start_index == springs.len() {
            return 0;
        }

        if let Some(count_lookup) = lookup[start_index][size_index] {
            return count_lookup;
        }

        if let Some(c) = springs.chars().nth(start_index) {
            match c {
                '.' => {
                    count = ConditionRecord::get_arrangement_count(
                        springs,
                        sizes,
                        start_index + 1,
                        size_index,
                        lookup,
                    )
                }
                '?' => {
                    // '.' option
                    count += ConditionRecord::get_arrangement_count(
                        springs,
                        sizes,
                        start_index + 1,
                        size_index,
                        lookup,
                    );
                    // '#' option
                    count += ConditionRecord::count_hash(
                        springs,
                        sizes,
                        start_index,
                        size_index,
                        lookup,
                    );
                }
                '#' => {
                    count = ConditionRecord::count_hash(
                        springs,
                        sizes,
                        start_index,
                        size_index,
                        lookup,
                    );
                }
                default => panic!("invalid character '{}'", default),
            };
        }

        lookup[start_index][size_index] = Some(count);
        return count;
    }

    fn count_hash(
        springs: &str,
        sizes: &[usize],
        start_index: usize,
        size_index: usize,
        lookup: &mut Vec<Vec<Option<usize>>>,
    ) -> usize {
        if let Some(&size) = sizes.get(size_index) {
            if let Some(spring) = springs.get(start_index..start_index + size) {
                if !spring.contains('.') {
                    if let Some(next_char) = springs.chars().nth(start_index + size) {
                        if next_char != '#' {
                            return ConditionRecord::get_arrangement_count(
                                springs,
                                sizes,
                                start_index + size + 1,
                                size_index + 1,
                                lookup,
                            );
                        }
                    } else {
                        return if sizes.len() == size_index + 1 { 1 } else { 0 };
                    }
                }
            }
        }

        return 0;
    }

    /// Solves a dynamic programming problem to count the number of arrangements of springs.
    ///
    /// The function takes a string `spring` representing a field of springs and an array `sizes`
    /// containing the sizes of contiguous groups of damaged springs. It calculates the total number
    /// of valid arrangements where contiguous groups of damaged springs of specified sizes are allowed.
    ///
    /// # Arguments
    ///
    /// * `spring` - A string representing the field of springs.
    /// * `sizes` - An array of sizes for contiguous groups of damaged springs.
    ///
    /// # Returns
    ///
    /// The total number of valid arrangements based on the specified sizes of damaged springs.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = get_arrangement_count_dp(".??.###", &[2, 3]);
    /// assert_eq!(result, 1);
    /// ```
    fn get_arrangement_count_dp(spring: &str, sizes: &[usize]) -> usize {
        //Prepend a '.' and trim any '.' from the end 
        let spring = format!(".{}", spring.trim_end_matches('.')).chars().collect::<Vec<_>>();
    
        let mut arrangement_count:Vec<usize> = vec![0; spring.len() + 1];
        arrangement_count[0] = 1;
    
        for (i, _) in spring.iter().take_while(|&&c| c != '#').enumerate() {
            arrangement_count[i + 1] = 1;
        }
    
        for &size in sizes {
            let mut new_arrangement_count = vec![0; spring.len() + 1];
            let mut group_length = 0;
    
            for (i, &c) in spring.iter().enumerate() {
                // reset the group length or increase
                if c == '.' {
                    group_length = 0;
                } else {
                    group_length += 1;
                }
    
                if c != '#' {
                    new_arrangement_count[i + 1] += new_arrangement_count[i];
                }
    
                if group_length >= size && spring[i - size] != '#' {
                    new_arrangement_count[i + 1] += arrangement_count[i - size];
                }
            }
    
            arrangement_count = new_arrangement_count;
        }
    
        return *arrangement_count.last().unwrap();
    }

}

#[derive(Debug)]
struct ConditionRecordError;

impl FromStr for ConditionRecord {
    type Err = ConditionRecordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [springs_str, sizes_str] = &s
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>()[..]
        {
            let springs = springs_str.to_string();
            let sizes = sizes_str
                .split(',')
                .map(|x| x.parse::<usize>().expect(""))
                .collect();
            return Ok(ConditionRecord { springs, sizes });
        }

        return Err(ConditionRecordError);
    }
}

#[test]
pub fn part1_test1() {
    let ans = part1("C:/git/advent_of_code/day12/input/test1.txt");
    assert_eq!(ans, 21);
}

#[test]
pub fn part1_test2() {
    let ans = part1("C:/git/advent_of_code/day12/input/test2.txt");
    assert_eq!(ans, 7857);
}

#[test]
pub fn part2_test1() {
    let ans = part2("C:/git/advent_of_code/day12/input/test1.txt");
    assert_eq!(ans, 525152);
}

#[test]
pub fn part2_test2() {
    let ans = part2("C:/git/advent_of_code/day12/input/test2.txt");
    assert_eq!(ans, 28606137449920);
}