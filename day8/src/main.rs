use std::collections::HashMap;
use std::str::FromStr;
use std::{fs, usize};

fn main() {
    let ans = part1("C:/git/advent_of_code/day8/input/test1.txt");
    println!("part 1 test 1 answer: {}", ans);

    let ans = part1("C:/git/advent_of_code/day8/input/test2.txt");
    println!("part 1 test 2 answer: {}", ans);

    let ans = part2("C:/git/advent_of_code/day8/input/test1.txt");
    println!("part 2 test 1 answer: {}", ans);

    let ans = part2("C:/git/advent_of_code/day8/input/test2.txt");
    println!("part 2 test 2 answer: {}", ans);
}

fn part1(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("Unable to read the input file");
    let network = Network::new(&input);
    return 0;
}

fn part2(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("Unable to read the input file");
    let network = Network::new(&input);
    return 0;
}

#[derive(Debug)]
struct Network {
    instructions: String,
    nodes: HashMap<String, (String, String)>,
}

impl Network {
    pub fn new(str: &str) -> Network {
        return Network::from_str(str).expect("Ctor from string failed");
    }
}

#[derive(Debug)]
struct NetworkParseError;

impl FromStr for Network {
    type Err = NetworkParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {

        return Ok(Network { instructions: "".to_string(), nodes : HashMap::new() });
    }
}

#[test]
fn part1_test1() {
    let result = part1("C:/git/advent_of_code/day8/input/test1.txt");
    assert_eq!(result, 2);
}

#[test]
fn part1_test2() {
    let result = part1("C:/git/advent_of_code/day8/input/test2.txt");
    assert_eq!(result, 249390788);
}

#[test]
fn part2_test1() {
    let result = part1("C:/git/advent_of_code/day8/input/test3.txt");
    assert_eq!(result, 5905);
}

#[test]
fn part2_test2() {
    let result = part1("C:/git/advent_of_code/day8/input/test4.txt");
    assert_eq!(result, 0);
}
