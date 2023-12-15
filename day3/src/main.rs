use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

fn main() {
    let ans = part1("C:/git/advent_of_code/day3/input/test1.txt");
    println!("part 1 test 1 answer: {}", ans);

    let ans = part1("C:/git/advent_of_code/day3/input/test2.txt");
    println!("part 1 test 2 answer: {}", ans);

    let ans = part2("C:/git/advent_of_code/day3/input/test1.txt");
    println!("part 2 test 1 answer: {}", ans);

    let ans = part2("C:/git/advent_of_code/day3/input/test2.txt");
    println!("part 2 test 2 answer: {}", ans);
}

#[derive(Default, Debug, Eq, Hash, Clone)]
struct Number {
    start_index: usize,
    end_index: usize,
    line_number: usize,
    value: u32,
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.start_index == other.start_index
            && self.end_index == other.end_index
            && self.line_number == other.line_number
            && self.value == other.value
    }
}

struct Schematic {
    parts: Vec<Number>,
    gears: Vec<HashSet<Number>>,
}

impl Schematic {
    pub fn new(str: &str) -> Schematic {
        return Schematic::from_str(str).expect("Ctor from string failed");
    }
}

#[derive(Debug)]
struct SchematicParseError;

impl FromStr for Schematic {
    type Err = SchematicParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut parts: Vec<Number> = Vec::new();
        let mut line_index: usize = 0;

        // Get Parts
        for line in str.lines() {
            parts.append(
                &mut get_numbers(line, line_index)
                    .into_iter()
                    .filter(|x| is_part(&x, str))
                    .collect(),
            );
            line_index += 1;
        }

        // Get Gears
        let mut gears = Vec::new();
        line_index = 0;
        for line in str.lines() {
            let potential_gears: Vec<usize> = line
                .chars()
                .enumerate()
                .filter_map(|(index, char)| if char == '*' { Some(index) } else { None })
                .collect();
            gears.append(&mut get_gears(potential_gears, line_index, str, &parts));

            line_index += 1;
        }

        return Ok(Schematic { parts, gears });
    }
}

fn get_numbers(line: &str, line_number: usize) -> Vec<Number> {
    let mut numbers_vec = Vec::new();
    let mut current_number = String::new();

    for ch in line.char_indices() {
        if ch.1.is_digit(10) {
            current_number.push(ch.1);
        } else if !current_number.is_empty() {
            if let Ok(number) = current_number.parse::<u32>() {
                numbers_vec.push(Number {
                    start_index: ch.0 - current_number.len(),
                    end_index: ch.0 - 1,
                    line_number: line_number,
                    value: number,
                });
            }
            current_number.clear();
        }
    }

    if let Ok(number) = current_number.parse::<u32>() {
        numbers_vec.push(Number {
            start_index: line.len() - current_number.len(),
            end_index: line.len() - 1,
            line_number: line_number,
            value: number,
        });
    }

    return numbers_vec;
}

fn get_gears(
    potential_gears: Vec<usize>,
    line_index: usize,
    str: &str,
    parts: &Vec<Number>,
) -> Vec<HashSet<Number>> {
    let mut gears = Vec::new();

    let line = str.lines().nth(line_index).expect("no line");

    // search nearest neighbor of each '*' and only add each number once
    for p_gear in potential_gears {
        let mut numbers = HashSet::new();

        let start_x = if p_gear == 0 { 0 } else { p_gear - 1 };

        let end_x = if p_gear == line.len() - 1 {
            p_gear
        } else {
            p_gear + 1
        };

        if line_index > 0 {
            let y = (line_index - 1) as usize;
            if let Some(prev) = str.lines().nth(y) {
                for x in start_x..=end_x {
                    if let Some(ch) = prev.chars().nth(x) {
                        if ch.is_digit(10) {
                            numbers.insert(
                                parts
                                    .iter()
                                    .find(|p| {
                                        p.line_number == y && x >= p.start_index && x <= p.end_index
                                    })
                                    .expect("what happened?")
                                    .clone(),
                            );
                        }
                    }
                }
            }
        }

        if let Some(ch) = line.chars().nth(start_x) {
            if ch.is_digit(10) {
                numbers.insert(
                    parts
                        .iter()
                        .find(|p| {
                            p.line_number == line_index
                                && start_x >= p.start_index
                                && start_x <= p.end_index
                        })
                        .expect("what happened?")
                        .clone(),
                );
            }
        }

        if let Some(ch) = line.chars().nth(end_x) {
            if ch.is_digit(10) {
                numbers.insert(
                    parts
                        .iter()
                        .find(|p| {
                            p.line_number == line_index
                                && end_x >= p.start_index
                                && end_x <= p.end_index
                        })
                        .expect("what happened?")
                        .clone(),
                );
            }
        }

        let y = (line_index + 1) as usize;
        if let Some(next) = str.lines().nth(y) {
            for x in start_x..=end_x {
                if let Some(ch) = next.chars().nth(x) {
                    if ch.is_digit(10) {
                        numbers.insert(
                            parts
                                .iter()
                                .find(|p| {
                                    p.line_number == y && x >= p.start_index && x <= p.end_index
                                })
                                .expect("what happened?")
                                .clone(),
                        );
                    }
                }
            }
        }

        if numbers.len() > 1 {
            gears.push(numbers);
        }
    }

    return gears;
}

fn is_part(number: &Number, str: &str) -> bool {
    let line = str
        .lines()
        .nth(number.line_number as usize)
        .expect("no line");

    let start_x = if number.start_index == 0 {
        0
    } else {
        number.start_index - 1
    };

    let end_x = if number.end_index == line.len() - 1 {
        number.end_index
    } else {
        number.end_index + 1
    };

    if number.line_number > 0 {
        if let Some(prev) = str.lines().nth((number.line_number - 1) as usize) {
            for x in start_x..=end_x {
                if let Some(ch) = prev.chars().nth(x) {
                    if !(ch.is_digit(10) || ch == '.') {
                        return true;
                    }
                }
            }
        }
    }

    if let Some(ch) = line.chars().nth(start_x) {
        if !(ch.is_digit(10) || ch == '.') {
            return true;
        }
    }

    if let Some(ch) = line.chars().nth(end_x) {
        if !(ch.is_digit(10) || ch == '.') {
            return true;
        }
    }

    if let Some(next) = str.lines().nth((number.line_number + 1) as usize) {
        for x in start_x..=end_x {
            if let Some(ch) = next.chars().nth(x) {
                if !(ch.is_digit(10) || ch == '.') {
                    return true;
                }
            }
        }
    }

    return false;
}

fn part1(file_path: &str) -> u32 {
    let input = fs::read_to_string(file_path).expect("Unable to read the input file");
    let schematic = Schematic::new(&input);
    return schematic.parts.iter().map(|x| x.value).sum();
}

fn part2(file_path: &str) -> u32 {
    let input = fs::read_to_string(file_path).expect("Unable to read the input file");
    let schematic = Schematic::new(&input);
    return schematic
        .gears
        .iter()
        .map(|x| x.iter().map(|p| p.value).fold(1, |acc, e| acc * e))
        .sum();
}

#[test]
fn part1_test1() {
    let result = part1("C:/git/advent_of_code/day3/input/test1.txt");
    assert_eq!(result, 4361);
}

#[test]
fn part1_test2() {
    let result = part1("C:/git/advent_of_code/day3/input/test2.txt");
    assert_eq!(result, 521515);
}

#[test]
fn part2_test1() {
    let result = part2("C:/git/advent_of_code/day3/input/test1.txt");
    assert_eq!(result, 467835);
}

#[test]
fn part2_test2() {
    let result = part2("C:/git/advent_of_code/day3/input/test2.txt");
    assert_eq!(result, 69527306);
}
