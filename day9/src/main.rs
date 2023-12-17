use std::str::FromStr;
use std::{fs};

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

fn part1(file_path: &str) -> i32 {
    let input = fs::read_to_string(file_path).expect("Unable to read the input file");
    let oasis = Oasis::new(&input);
    let predictions = oasis.get_predictions();

    return predictions.iter().sum();
}

fn part2(file_path: &str) -> i32 {
    let input = fs::read_to_string(file_path).expect("Unable to read the input file");
    let oasis = Oasis::new(&input);
    let predictions = oasis.get_left_predictions();
    return predictions.iter().sum();
}

#[derive(Debug)]
struct Oasis {
    reports: Vec<Vec<i32>>
}

impl Oasis {
    pub fn new(str: &str) -> Oasis {
        return Oasis::from_str(str).expect("Ctor from string failed");
    }

    pub fn get_predictions(&self) -> Vec<i32> {
        return self.reports.iter().map(|x| Oasis::get_prediction(x)).collect();
    }

    pub fn get_left_predictions(&self) -> Vec<i32> {
        return self.reports.iter().map(|x| Oasis::get_left_prediction(x)).collect();
    }

    pub fn get_prediction(report: &Vec<i32>) -> i32 {
        let mut pyramid: Vec<Vec<i32>> = Vec::new();
        
        let mut current_line: &Vec<i32> = report;
        
        while !current_line.iter().all(|x| *x == 0) {
            let mut line = Vec::new();
            for window in current_line.windows(2) {
                let left = window[0];
                let right = window[1];
                line.push(right - left);
            }
            pyramid.push(line.clone());
            current_line = pyramid.last().expect("line exists");
        }

        let sum: i32 = pyramid.iter().map(|x| x.last().expect("has elements")).sum();
        return report.last().expect("has elements") + sum;
    }

    pub fn get_left_prediction(report: &Vec<i32>) -> i32 {
        let pyramid: Vec<Vec<i32>> = Oasis::get_pyramid(report);

        //let sum: i32 = pyramid.iter().map(|x| -x.first().expect("has elements")).sum();
        let mut sum = 0;
        for line in pyramid.iter().rev() {
            sum = line.first().expect("has element") - sum;
        }
        return report.first().expect("has elements") - sum;
    }

    pub fn get_pyramid(report: &Vec<i32>) -> Vec<Vec<i32>> {
        let mut pyramid: Vec<Vec<i32>> = Vec::new();
        
        let mut current_line: &Vec<i32> = report;
        
        while !current_line.iter().all(|x| *x == 0) {
            let mut line = Vec::new();
            for window in current_line.windows(2) {
                let left = window[0];
                let right = window[1];
                line.push(right - left);
            }
            pyramid.push(line.clone());
            current_line = pyramid.last().expect("line exists");
        }

        return pyramid;
    }

}

#[derive(Debug)]
struct OasisParseError;

impl FromStr for Oasis {
    type Err = OasisParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let reports: Vec<Vec<i32>> = str.lines().map(|x| x.split_whitespace().map(|x| x.parse::<i32>().expect("not an int?")).collect()).collect();
        return Ok(Oasis { reports });
    }
}

#[test]
fn part1_test1() {
    let result = part1("C:/git/advent_of_code/day9/input/test1.txt");
    assert_eq!(result, 114);
}

#[test]
fn part1_test2() {
    let result = part1("C:/git/advent_of_code/day9/input/test2.txt");
    assert_eq!(result, 2038472161);
}

#[test]
fn part2_test1() {
    let result = part2("C:/git/advent_of_code/day9/input/test1.txt");
    assert_eq!(result, 2);
}

#[test]
fn part2_test2() {
    let result = part2("C:/git/advent_of_code/day9/input/test2.txt");
    assert_eq!(result, 0);
}
