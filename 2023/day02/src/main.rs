use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const RED_LIMIT: u32 = 12;
const GREEN_LIMIT: u32 = 13;
const BLUE_LIMIT: u32 = 14;

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

fn part1(file_path: &str) -> u32 {
    let mut sum = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok) {
            let game_split: Vec<&str> = line.split(':').collect();
            if game_split.len() != 2 {
                println!("Error with line so skipping: {}", line);
                continue;
            }

            let game_id = game_split[0]
                .split_whitespace()
                .nth(1)
                .expect("No Game Id present")
                .parse::<u32>()
                .expect("Unable to parse Game Id");

            if game_split[1].split(';').any(exceeds_limit) {
                continue;
            }
            sum += game_id;
        }
    }
    return sum;
}

fn part2(file_path: &str) -> u32 {
    let mut sum = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok) {
            let game_split: Vec<&str> = line.split(':').collect();
            if game_split.len() != 2 {
                println!("Error with line so skipping: {}", line);
                continue;
            }

            let max_rgb = game_split[1]
                .split(';')
                .fold(RGB::default(), |acc, x| get_max(&acc, &get_rgb(x)));

            sum += max_rgb.r * max_rgb.g * max_rgb.b;
        }
    }
    return sum;
}

fn exceeds_limit(set: &str) -> bool {
    //println!("Set: {}", set);
    for roll in set.split(',') {
        //println!("roll: {}", roll);
        let amount = roll
            .split_whitespace()
            .next()
            .expect("No roll count present")
            .parse::<u32>()
            .expect("Unable to parse roll count");
        let color = roll.split_whitespace().nth(1).expect("No color present");
        let limit = match color {
            "red" => RED_LIMIT,
            "green" => GREEN_LIMIT,
            "blue" => BLUE_LIMIT,
            _ => RED_LIMIT,
        };

        if amount > limit {
            return true;
        }
    }
    return false;
}

#[derive(Default)]
struct RGB {
    r: u32,
    g: u32,
    b: u32,
}

fn get_rgb(set: &str) -> RGB {
    let mut rgb = RGB::default();
    for roll in set.split(',') {
        //println!("roll: {}", roll);
        let amount = roll
            .split_whitespace()
            .next()
            .expect("No roll count present")
            .parse::<u32>()
            .expect("Unable to parse roll count");
        let color = roll.split_whitespace().nth(1).expect("No color present");
        match color {
            "red" => rgb.r = amount,
            "green" => rgb.g = amount,
            "blue" => rgb.b = amount,
            _ => rgb.r = amount,
        };
    }
    return rgb;
}

fn get_max(a: &RGB, b: &RGB) -> RGB {
    let rgb = RGB {
        r: max(a.r, b.r),
        g: max(a.g, b.g),
        b: max(a.b, b.b),
    };
    return rgb;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("file not found");
    Ok(io::BufReader::new(file).lines())
}

#[test]
fn part1_test1() {
    let result = part1("input/test1.txt");
    assert_eq!(result, 8);
}

#[test]
fn part1_test2() {
    let result = part1("input/test2.txt");
    assert_eq!(result, 2285);
}

#[test]
fn part2_test1() {
    let result = part2("input/test1.txt");
    assert_eq!(result, 2286);
}

#[test]
fn part2_test2() {
    let result = part2("input/test2.txt");
    assert_eq!(result, 77021);
}
