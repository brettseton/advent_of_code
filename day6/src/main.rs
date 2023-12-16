use std::str::FromStr;
use std::{fs, usize};

fn main() {
    let ans = part1("C:/git/advent_of_code/day6/input/test1.txt");
    println!("part 1 test 1 answer: {}", ans);

    let ans = part1("C:/git/advent_of_code/day6/input/test2.txt");
    println!("part 1 test 2 answer: {}", ans);

    let ans = part1("C:/git/advent_of_code/day6/input/test3.txt");
    println!("part 2 test 1 answer: {}", ans);

    let ans = part1("C:/git/advent_of_code/day6/input/test4.txt");
    println!("part 2 test 2 answer: {}", ans);
}

fn part1(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("Unable to read the input file");
    let game = Game::new(&input);

    let total_wins = game
        .races
        .iter()
        .fold(1, |acc, e| acc * e.get_number_of_wins());
    return total_wins;
}

#[derive(Debug)]
struct Game {
    races: Vec<Race>,
}

impl Game {
    pub fn new(str: &str) -> Game {
        return Game::from_str(str).expect("Ctor from string failed");
    }
}

#[derive(Debug)]
struct GameParseError;

impl FromStr for Game {
    type Err = GameParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let times: Vec<usize> = str
            .lines()
            .nth(0)
            .expect("definitely a game")
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse::<usize>().expect("all times are numbers"))
            .collect();
        let distances: Vec<usize> = str
            .lines()
            .nth(1)
            .expect("definitely a game")
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse::<usize>().expect("all distances are numbers"))
            .collect();
        let races: Vec<Race> = times
            .iter()
            .zip(distances.iter())
            .map(|(&t, &d)| Race {
                time: t,
                distance: d,
            })
            .collect();

        return Ok(Game { races });
    }
}

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    pub fn get_number_of_wins(&self) -> usize {
        let b = self.time as f32;
        let c = self.distance as f32;

        // Get the lower square root of 
        // x^2 - time*x+ distance = 0
        let mut l = ((b - f32::sqrt(b * b - 4.0 * c)) / 2.0).ceil() as usize;
        
        // We need to beat the current distance record
        if (self.time - l) * l <= self.distance {
            l += 1;
        }
        let mid = self.time / 2;
        return match self.time % 2 {
            0 => (mid - l + 1) * 2 - 1,
            1 => (mid - l + 1) * 2,
            _ => 0,
        };
    }
}

#[test]
fn part1_test1() {
    let result = part1("C:/git/advent_of_code/day6/input/test1.txt");
    assert_eq!(result, 288);
}

#[test]
fn part1_test2() {
    let result = part1("C:/git/advent_of_code/day6/input/test2.txt");
    assert_eq!(result, 861300);
}

#[test]
fn part2_test1() {
    let result = part1("C:/git/advent_of_code/day6/input/test3.txt");
    assert_eq!(result, 71503);
}

#[test]
fn part2_test2() {
    let result = part1("C:/git/advent_of_code/day6/input/test4.txt");
    assert_eq!(result, 28101347);
}
