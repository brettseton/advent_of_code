use std::{fs, str::FromStr};

fn main() {
    let ans = part1("C:/git/advent_of_code/day12/input/test1.txt");
    println!("part 1 test 1 : {}", ans);

    let ans = part1("C:/git/advent_of_code/day12/input/test2.txt");
    println!("part 1 test 2 : {}", ans);

    let ans = part2("C:/git/advent_of_code/day12/input/test1.txt");
    println!("part 2 test 1 : {}", ans);

    let ans = part2("C:/git/advent_of_code/day12/input/test2.txt");
    println!("part 2 test 2 : {}", ans);
}

fn part1 (file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let condition_report = ConditionReport::new(&input);
    return condition_report.get_arrangements();
}

fn part2 (file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let condition_report = ConditionReport::new(&input);
    return condition_report.get_arrangements();
}

struct ConditionReport{
   rows: Vec<ConditionRecord>
}

impl ConditionReport {
    pub fn new(str: &str) -> ConditionReport {
        return ConditionReport::from_str(str).expect("");
    }
    
    pub fn get_arrangements(&self) -> usize {

        let arrangements = &self.rows.iter().map(|row| row.get_arrangements()).collect::<Vec<usize>>();

        return arrangements.iter().sum();
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

struct ConditionRecord{
    springs: String,
    sizes: Vec<usize>
 }

impl ConditionRecord {
    pub fn new(str: &str) -> ConditionRecord {
        return ConditionRecord::from_str(str).expect("");
    }
    
    pub fn get_arrangements(&self) -> usize {
        return ConditionRecord::get_arrangement_count(self.springs.clone(), self.sizes.clone(), 0);
    }

    pub fn get_arrangement_count(springs: String, sizes: Vec<usize>, start_index: usize) -> usize {
        let mut count = 0;

        if sizes.is_empty() && start_index == springs.len() {
            return 1;
        }

        if let Some(c) = springs.chars().nth(start_index){
            match c {
                '?' => {
                    // '.' option
                    count += ConditionRecord::get_arrangement_count(springs.clone(), sizes.clone(), start_index + 1);
                    // '#' option
                    if let Some(size) = sizes.iter().nth(0) {
                        // Check next

                        if let Some(spring) = springs.get(start_index..start_index+size){
                            if spring.contains('.') {
                                return count;
                            }

                            // if next char is a '.', '?', or none you can skip
                            if let Some(next) = springs.chars().nth(start_index+size){
                                if next == '#' {
                                    return count;
                                }

                                if let Some(new_sizes) = sizes.get(1..sizes.len()){
                                    return ConditionRecord::get_arrangement_count(springs, new_sizes.to_vec().clone(), start_index + size + 1) + count;
                                } else {
                                    return ConditionRecord::get_arrangement_count(springs, vec![], start_index + size + 1) + count;
                                }
                            } else {
                                if sizes.len() == 1 {
                                    return count + 1;
                                } else {
                                    return count;
                                }
                            }

                        } else {
                            return count;
                        }

                    } else {
                        // None left
                        return count;
                    }

                },
                '.' => {
                    return ConditionRecord::get_arrangement_count(springs, sizes, start_index + 1)
                },
                '#' => {

                    if let Some(size) = sizes.iter().nth(0) {
                        // Check next

                        if let Some(spring) = springs.get(start_index..start_index+size){
                            if spring.contains('.') {
                                return 0;
                            }

                            // if next char is a '.', '?', or none you can skip
                            if let Some(next) = springs.chars().nth(start_index+size){
                                if next == '#' {
                                    return 0;
                                }

                                if let Some(new_sizes) = sizes.get(1..sizes.len()){
                                    return ConditionRecord::get_arrangement_count(springs, new_sizes.to_vec().clone(), start_index + size + 1);
                                } else {
                                    return ConditionRecord::get_arrangement_count(springs, vec![], start_index + size + 1);
                                }
                            } else {
                                if sizes.len() == 1 {
                                    return 1;
                                } else {
                                    return 0;
                                }
                            }

                        } else {
                            return 0;
                        }

                    } else {
                        // None left
                        return 0;
                    }
                },
                default => panic!("invalid character '{}'", default),
            };
        }

        return count;
    }
}

#[derive(Debug)]
struct ConditionRecordError;

impl FromStr for ConditionRecord {
    type Err = ConditionRecordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        if let [springs_str, sizes_str] = &s.split_whitespace().map(String::from).collect::<Vec<String>>()[..] {
            let springs = springs_str.to_string();
            let sizes = sizes_str.split(',').map(|x| x.parse::<usize>().expect("")).collect();
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
    assert_eq!(ans, 0);
}