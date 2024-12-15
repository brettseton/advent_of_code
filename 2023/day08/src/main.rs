use std::collections::HashMap;
use std::str::FromStr;
use std::{fs, usize};

fn main() {
    let ans = part1("input/test1.txt");
    println!("part 1 test 1 answer: {}", ans);

    let ans = part1("input/test2.txt");
    println!("part 1 test 2 answer: {}", ans);

    let ans = part2("input/test1.txt");
    println!("part 2 test 1 answer: {}", ans);

    let ans = part2("input/test2.txt");
    println!("part 2 test 2 answer: {}", ans);

    let ans = part2("input/test3.txt");
    println!("part 2 test 3 answer: {}", ans);
}

fn part1(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("Unable to read the input file");
    let network = Network::new(&input);
    let path = network.get_path("AAA", "ZZZ");

    return path.len() - 1;
}

fn part2(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("Unable to read the input file");
    let network = Network::new(&input);
    let path_length = network.get_ghost_path_length();
    return path_length;
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

    pub fn get_path(&self, start: &str, end: &str) -> Vec<String> {
        let mut path = vec![start.to_string()];
        let mut current = start;

        let mut direction = self.instructions.chars().cycle();

        while current != end {
            let next_direction = direction.next().expect("direction not empty");
            let (left, right) = match self.nodes.get(current) {
                Some((left, right)) => (left, right),
                None => break, // No valid path found, break out of the loop
            };

            current = match next_direction {
                'L' => &left,
                'R' => &right,
                _ => panic!("path must continue"),
            };
            path.push(current.to_string());
        }

        return path;
    }

    pub fn get_ghost_path_length(&self) -> usize {
        let current_paths: Vec<String> = self
            .nodes
            .keys()
            .filter(|x| x.ends_with('A'))
            .map(String::from)
            .collect();

        let mut lcm = 1;

        for path in current_paths {
            let mut current: &str = &path;
            let mut direction = self.instructions.chars().cycle();
            let mut step_count = 0;
            while !current.ends_with('Z') {
                let next_direction = direction.next().expect("direction not empty");
                let (left, right) = match self.nodes.get(current) {
                    Some((left, right)) => (left, right),
                    None => panic!("path must exist"),
                };

                current = match next_direction {
                    'L' => &left,
                    'R' => &right,
                    _ => panic!("path must continue"),
                };
                step_count += 1;
            }
            lcm = num::integer::lcm(lcm, step_count);
        }

        return lcm;
    }
}

#[derive(Debug)]
struct NetworkParseError;

impl FromStr for Network {
    type Err = NetworkParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let instructions = str.lines().nth(0).expect("file needs data").to_string();
        let mut nodes = HashMap::new();

        for line in str.lines().skip(2) {
            if let [node, map] = &line.split('=').map(String::from).collect::<Vec<String>>()[..] {
                if let [left, right] =
                    &map.split(',').map(String::from).collect::<Vec<String>>()[..]
                {
                    nodes.insert(
                        node.trim().to_string(),
                        (left[2..=4].to_string(), right[1..=3].to_string()),
                    );
                }
            }
        }
        return Ok(Network {
            instructions,
            nodes,
        });
    }
}

#[test]
fn part1_test1() {
    let result = part1("input/test1.txt");
    assert_eq!(result, 2);
}

#[test]
fn part1_test2() {
    let result = part1("input/test2.txt");
    assert_eq!(result, 11911);
}

#[test]
fn part2_test1() {
    let result = part2("input/test1.txt");
    assert_eq!(result, 2);
}

#[test]
fn part2_test2() {
    let result = part2("input/test2.txt");
    assert_eq!(result, 10151663816849);
}

#[test]
fn part2_test3() {
    let result = part2("input/test3.txt");
    assert_eq!(result, 6);
}
