use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

fn main() {
    let ans = part1("input/test1.txt");
    println!("part 1 test 1 answer: {}", ans);

    let ans = part1("input/test2.txt");
    println!("part 1 test 2 answer: {}", ans);

    let ans = part2("input/test1.txt");
    println!("part 2 test 1 answer: {}", ans);

    let ans = part2("input/test2.txt");
    println!("part 2 test 2 answer: {}", ans);
}

// Gets the parts from a schematic and sums them
fn part1(file_path: &str) -> u32 {
    let input = fs::read_to_string(file_path).expect("Unable to read the input file");
    let schematic = Schematic::new(&input);
    return schematic.parts.iter().map(|x| x.value).sum();
}

// Gets the gears from a schematic, multiplies the parts in each gear together and sums the results
fn part2(file_path: &str) -> u32 {
    let input = fs::read_to_string(file_path).expect("Unable to read the input file");
    let schematic = Schematic::new(&input);
    return schematic
        .gears
        .iter()
        .map(|x| x.iter().map(|p| p.value).product::<u32>())
        .sum();
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
                    .filter(|x| is_part(x, str))
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

/// Extracts all numbers from a given line and returns a vector of corresponding `Number` structs.
///
/// Given a line of characters (`line`) and the line number (`line_number`), this function scans
/// the line, identifies numeric parts, and creates a vector of `Number` structs representing
/// those numbers. Each `Number` struct includes the start and end indices, line number, and
/// the parsed numeric value of the identified part.
///
/// # Arguments
///
/// * `line` - The input line containing characters to be scanned for numeric parts.
/// * `line_number` - The line number in the overall text.
///
/// # Returns
///
/// Returns a vector of `Number` structs representing the identified numeric parts in the line.
///
/// # Examples
///
/// ```rust

/// // Create sample input data
/// let line = "12*34";
/// let line_number = 0;
///
/// // Call the function
/// let result = get_numbers(line, line_number);
///
/// // Assert the result
/// let expected_result: Vec<Number> = vec![Number { start_index: 0, end_index: 1, line_number: 0, value: 12 },
///                                         Number { start_index: 3, end_index: 4, line_number: 0, value: 34 }];
/// assert_eq!(result, expected_result);
/// ```
fn get_numbers(line: &str, line_number: usize) -> Vec<Number> {
    let mut numbers_vec = Vec::new();
    let mut current_number = String::new();

    for ch in line.char_indices() {
        if ch.1.is_ascii_digit() {
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

/// Extracts neighboring parts around the '*' character in a specified line.
/// If there are two or more parts then it is a gear
///
/// Given a vector of potential gear indices (`potential_gears`), a line index (`line_index`),
/// a string (`str`), and a vector of `Number` structs (`parts`), this function searches for
/// neighboring parts around each '*' character in the specified line. The result is a vector
/// of hash sets, where each hash set represents a group of neighboring parts found.
///
/// # Arguments
///
/// * `potential_gears` - A vector containing potential gear indices.
/// * `line_index` - The index of the line containing the '*' character.
/// * `str` - The input string containing the lines of characters.
/// * `parts` - A vector of `Number` structs representing numeric parts in the lines.
///
/// # Returns
///
/// A vector of hash sets, where each hash set contains unique instances of the `Number` struct
/// corresponding to neighboring parts found around each '*' character. If a hash set contains
/// more than one element, it is added to the result vector.
///
/// # Panics
///
/// This function panics if the specified line index is out of bounds.
///
/// # Examples
///
/// ```rust
/// use std::collections::HashSet;
/// use your_module::{get_gears, Number};
///
/// // Create sample input data
/// let potential_gears = vec![2];
/// let line_index = 0;
/// let input_str = "12*34";
/// let parts = vec![Number { line_number: 0, start_index: 0, end_index: 1, value: 12 },
///                  Number { line_number: 0, start_index: 3, end_index: 4, value: 34 }];
///
/// // Call the function
/// let result = get_gears(potential_gears, line_index, input_str, &parts);
///
/// // Assert the result
/// let expected_result: Vec<HashSet<Number>> = vec![hashset!{
///                  Number { line_number: 0, start_index: 0, end_index: 1, value: 12 },
///                  Number { line_number: 0, start_index: 3, end_index: 4, value: 34 }
///                  }];
/// assert_eq!(result, expected_result);
/// ```
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
            let y = line_index - 1;
            if let Some(prev) = str.lines().nth(y) {
                for x in start_x..=end_x {
                    if let Some(ch) = prev.chars().nth(x) {
                        if ch.is_ascii_digit() {
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
            if ch.is_ascii_digit() {
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
            if ch.is_ascii_digit() {
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

        let y = line_index + 1;
        if let Some(next) = str.lines().nth(y) {
            for x in start_x..=end_x {
                if let Some(ch) = next.chars().nth(x) {
                    if ch.is_ascii_digit() {
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

/// Checks whether a specified `Number` represents a part in the given string.
/// A part is any number adjacent to a symbol, even diagonally, next to a symbol that is not a number or a '.'
///
/// Given a `Number` struct (`number`) and an input string (`str`), this method determines whether
/// the numeric part represented by the `Number` is a standalone part or if it is connected to
/// non-numeric characters. It returns `true` if the number is a part and `false` otherwise.
///
/// # Arguments
///
/// * `number` - A reference to a `Number` struct representing the number to be checked.
/// * `str` - The input string containing the lines of characters.
///
/// # Returns
///
/// Returns `true` if the specified `Number` represents a standalone part, and `false` otherwise.
///
/// # Panics
///
/// This method panics if the line index in the `Number` struct is out of bounds.
///
/// # Examples
///
/// ```rust
/// use your_module::{is_part, Number};
///
/// // Create sample input data
/// let number = Number { line_number: 0, start_index: 3, end_index: 4, value: 34 };
/// let input_str = "12*34";
///
/// // Call the method
/// let result = is_part(&number, input_str);
///
/// // Assert the result
/// assert_eq!(result, true);
/// ```
fn is_part(number: &Number, str: &str) -> bool {
    let line = str
        .lines()
        .nth(number.line_number)
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
        if let Some(prev) = str.lines().nth(number.line_number - 1) {
            for x in start_x..=end_x {
                if let Some(ch) = prev.chars().nth(x) {
                    if !(ch.is_ascii_digit() || ch == '.') {
                        return true;
                    }
                }
            }
        }
    }

    if let Some(ch) = line.chars().nth(start_x) {
        if !(ch.is_ascii_digit() || ch == '.') {
            return true;
        }
    }

    if let Some(ch) = line.chars().nth(end_x) {
        if !(ch.is_ascii_digit() || ch == '.') {
            return true;
        }
    }

    if let Some(next) = str.lines().nth(number.line_number + 1) {
        for x in start_x..=end_x {
            if let Some(ch) = next.chars().nth(x) {
                if !(ch.is_ascii_digit() || ch == '.') {
                    return true;
                }
            }
        }
    }

    return false;
}

#[test]
fn part1_test1() {
    let result = part1("input/test1.txt");
    assert_eq!(result, 4361);
}

#[test]
fn part1_test2() {
    let result = part1("input/test2.txt");
    assert_eq!(result, 521515);
}

#[test]
fn part2_test1() {
    let result = part2("input/test1.txt");
    assert_eq!(result, 467835);
}

#[test]
fn part2_test2() {
    let result = part2("input/test2.txt");
    assert_eq!(result, 69527306);
}
