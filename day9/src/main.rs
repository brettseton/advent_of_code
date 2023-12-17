use std::str::FromStr;
use std::{fs, usize};

fn main() {
    let ans = part1("C:/git/advent_of_code/day9/input/test1.txt");
    println!("part 1 test 1 answer: {}", ans);

    let ans = part1("C:/git/advent_of_code/day9/input/test2.txt");
    println!("part 1 test 2 answer: {}", ans);

    let ans = part2("C:/git/advent_of_code/day9/input/test1.txt");
    println!("part 2 test 1 answer: {}", ans);

    let ans = part2("C:/git/advent_of_code/day9/input/test2.txt");
    println!("part 2 test 2 answer: {}", ans);
}

fn part1(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("Unable to read the input file");
    let oasis = Oasis::new(&input);

    return 0;
}

fn part2(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("Unable to read the input file");
    let oasis = Oasis::new(&input);
    return 0;
}

#[derive(Debug)]
struct Oasis {
    report: Vec<Vec<usize>>
}

impl Oasis {
    pub fn new(str: &str) -> Oasis {
        return Oasis::from_str(str).expect("Ctor from string failed");
    }
}

#[derive(Debug)]
struct OasisParseError;

impl FromStr for Oasis {
    type Err = OasisParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let instructions = str.lines().nth(0).expect("file needs data").to_string();

        return Ok(Oasis { report: Vec::new() });
    }
}

#[test]
fn part1_test1() {
    let result = part1("C:/git/advent_of_code/day9/input/test1.txt");
    assert_eq!(result, 68);
}

#[test]
fn part1_test2() {
    let result = part1("C:/git/advent_of_code/day9/input/test2.txt");
    assert_eq!(result, 0);
}

#[test]
fn part2_test1() {
    let result = part2("C:/git/advent_of_code/day9/input/test1.txt");
    assert_eq!(result, 0);
}

#[test]
fn part2_test2() {
    let result = part2("C:/git/advent_of_code/day9/input/test2.txt");
    assert_eq!(result, 0);
}
