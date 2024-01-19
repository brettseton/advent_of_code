use std::{cmp::Ordering, fs, str::FromStr};

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
    let dig_plan = DigPlan::new(&input);
    return dig_plan.get_dig_area();
}

fn part2(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let dig_plan = DigPlan::new2(&input);
    return dig_plan.get_dig_area();
}

#[derive(PartialEq, Eq, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn to_usize(&self) -> usize {
        match self {
            Self::North => 0,
            Self::East  => 1,
            Self::South => 2,
            Self::West  => 3,
        }
    }

    pub fn get_delta(&self) -> (isize, isize) {
        match self {
            Self::North => (0, -1),
            Self::East  => (1,  0),
            Self::South => (0,  1),
            Self::West  => (-1, 0),
        }
    }

    pub fn new(str: &str) -> Direction {
       return match str.chars().nth(0) {
          Some('U') => Direction::North,
          Some('R') => Direction::East,
          Some('D') => Direction::South,
          Some('L') => Direction::West,

          Some('0') => Direction::East,
          Some('1') => Direction::South,
          Some('2') => Direction::West,
          Some('3') => Direction::North,
           _ => panic!()
       }; 
    }
    
}

impl Ord for Direction {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_usize().cmp(&other.to_usize())
    }
}

impl PartialOrd for Direction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Eq, PartialEq)]
struct DigStep {
    traveling: Direction,
    steps: usize,
    hex_color: String
}


struct DigPlan {
    dig_plan: Vec<DigStep>,
}

impl DigPlan {

    pub fn new(str: &str) -> DigPlan {
        return DigPlan::from_str(str).expect("");
    }

    pub fn new2(str: &str) -> DigPlan {
        return DigPlan::from_str2(str);
    }

    pub fn get_dig_area(&self) -> usize {
        let mut area = 0;
        let mut boundary: isize = 0;

        let mut current_x = 0;
        let mut current_y = 0;
        let mut previous_x = 0;
        let mut previous_y = 0;

        for step in self.dig_plan.iter() {
           let (dx, dy) = step.traveling.get_delta();
           current_x += dx * step.steps as isize;
           current_y += dy * step.steps as isize;
           boundary += step.steps as isize;
           area += (previous_y + current_y) * (previous_x - current_x);
           previous_x = current_x;
           previous_y = current_y;
        }

        // Pick's theorem https://en.wikipedia.org/wiki/Pick%27s_theorem
        return ((boundary + area)/2) as usize + 1 ;
    }

    fn from_str2(s: &str) -> DigPlan {
        let dig_plan = s
            .lines()
            .map(|s| {
                let mut split = s.split_whitespace();
                let hex_color= split.nth(2).unwrap().trim_start_matches('(').trim_end_matches(')').to_string();
                return DigStep{ traveling: Direction::new(&hex_color[6..=6]), steps: usize::from_str_radix(&hex_color[1..=5], 16).unwrap() , hex_color };
            })
            .collect::<Vec<DigStep>>();

        return DigPlan { dig_plan };
    }
}

#[derive(Debug)]
struct DigPlanError;

impl FromStr for DigPlan {
    type Err = DigPlanError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dig_plan = s
            .lines()
            .map(|s| {
                let mut split = s.split_whitespace();
                let traveling = Direction::new(split.nth(0).unwrap());
                let steps = split.nth(0).unwrap().parse::<usize>().unwrap();
                let hex_color= split.nth(0).unwrap().trim_start_matches('(').trim_end_matches(')').to_string();
                return DigStep{ traveling, steps, hex_color };
            })
            .collect::<Vec<DigStep>>();

        return Ok(DigPlan { dig_plan });
    }
}

#[test]
pub fn part1_test1() {
    let ans = part1("input/test1.txt");
    assert_eq!(ans, 62);
}

#[test]
pub fn part1_test2() {
    let ans = part1("input/test2.txt");
    assert_eq!(ans, 50465);
}

#[test]
pub fn part2_test1() {
    let ans = part2("input/test1.txt");
    assert_eq!(ans, 952408144115);
}

#[test]
pub fn part2_test2() {
    let ans = part2("input/test2.txt");
    assert_eq!(ans, 82712746433310);
}
