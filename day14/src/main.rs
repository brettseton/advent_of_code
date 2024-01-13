use std::{fs, str::FromStr};

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
    let platform = Platform::new(&input);
    return platform.get_load();
}

fn part2(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let platform = Platform::new(&input);
    return platform.get_load();
}

struct Platform {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize
}

impl Platform {
    pub fn new(str: &str) -> Platform {
        return Platform::from_str(str).expect("");
    }

    pub fn get_load(&self) -> usize {
        let mut load = 0;
        for y in 0..self.height {
            load += (0..self.width).filter_map(|x| self.find_resting_place(x, y)).sum::<usize>();
        }

        return load;
    }

    pub fn find_resting_place(&self, x: usize, y: usize) -> Option<usize>{
        if self.map[y][x] != 'O' {
            return None;
        }

        let mut resting_height = y;

        if y == 0 {
            return Some(self.height - resting_height);
        }

        for y_next in (0..y).rev() {
            if self.map[y_next][x] == '.' {
                resting_height -= 1; 
            }
            if self.map[y_next][x] == '#' {
                break;
            }
        }

        return Some(self.height - resting_height);
    }
}

#[derive(Debug)]
struct PlatformError;

impl FromStr for Platform {
    type Err = PlatformError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().nth(0).unwrap().len();
        let height = s.lines().count();
        let map = s.lines().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        return Ok(Platform { map, width, height });
    }
}



#[test]
pub fn part1_test1() {
    let ans = part1("input/test1.txt");
    assert_eq!(ans, 136);
}

#[test]
pub fn part1_test2() {
    let ans = part1("input/test2.txt");
    assert_eq!(ans, 106186);
}

#[test]
pub fn part2_test1() {
    let ans = part2("input/test1.txt");
    assert_eq!(ans, 0);
}

#[test]
pub fn part2_test2() {
    let ans = part2("input/test2.txt");
    assert_eq!(ans, 0);
}
