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
    return platform.get_load_after_tilt();
}

fn part2(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let mut platform = Platform::new(&input);
    return platform.cycle_n(1_000_000_000);
}

struct Platform {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
    map_history: Vec<String>,
}

impl Platform {
    pub fn new(str: &str) -> Platform {
        return Platform::from_str(str).expect("");
    }

    pub fn get_load_after_tilt(&self) -> usize {
        let mut load = 0;
        for y in 0..self.height {
            load += (0..self.width)
                .filter_map(|x| self.find_resting_place(x, y))
                .sum::<usize>();
        }

        return load;
    }

    pub fn find_resting_place(&self, x: usize, y: usize) -> Option<usize> {
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

    fn cycle_n(&mut self, n: usize) -> usize {
        self.cycle();
        let state = self.to_string();

        // If we have already been in this state we have entered a cycle
        // meaning we can now calculate what state will be the end state
        match self
            .map_history
            .iter()
            .enumerate()
            .find(|(_, x)| *x == &state)
        {
            Some((index, _)) => self.map_history
                [index + (n - 1) % (self.map_history.len() - index)]
                .parse::<Platform>()
                .expect("should be a platform")
                .get_load(),
            _ => {
                self.map_history.push(self.to_string());
                self.cycle_n(n - 1)
            }
        }
    }

    fn cycle(&mut self) {
        // North -> West -> South -> East
        self.tilt_y(0, 1);
        self.tilt_x(0, 1);
        self.tilt_y(self.height - 1, -1);
        self.tilt_x(self.width - 1, -1);
    }

    fn tilt_y(&mut self, start_row_index: usize, delta: i64) {
        for x in 0..self.width {
            let mut rock_stack_start_y = start_row_index as i64;
            let mut y: i64 = start_row_index as i64;

            while y >= 0 && y < self.height as i64 {
                match self.map[y as usize][x] {
                    '.' => (),
                    '#' => rock_stack_start_y = y + delta,
                    'O' => {
                        self.map[y as usize][x] = '.';
                        self.map[rock_stack_start_y as usize][x] = 'O';
                        rock_stack_start_y += delta;
                    }
                    _ => panic!("invalid"),
                }

                y += delta;
            }
        }
    }

    fn tilt_x(&mut self, start_col_index: usize, delta: i64) {
        for y in 0..self.height {
            let mut rock_stack_start_x = start_col_index as i64;
            let mut x = start_col_index as i64;

            while x >= 0 && x < self.width as i64 {
                match self.map[y][x as usize] {
                    '.' => (),
                    '#' => rock_stack_start_x = x + delta,
                    'O' => {
                        self.map[y][x as usize] = '.';
                        self.map[y][rock_stack_start_x as usize] = 'O';
                        rock_stack_start_x += delta;
                    }
                    _ => panic!("invalid"),
                }

                x += delta;
            }
        }
    }

    fn get_load(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .map(|(i, row)| (self.width - i) * row.iter().filter(|&&x| x == 'O').count())
            .sum()
    }
}


#[derive(Debug)]
struct PlatformError;

impl FromStr for Platform {
    type Err = PlatformError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().nth(0).unwrap().len();
        let height = s.lines().count();
        let map = s
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
        return Ok(Platform {
            map,
            width,
            height,
            map_history: Vec::from([s.to_string()]),
        });
    }
}

impl ToString for Platform {
    fn to_string(&self) -> String {
        self.map
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
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
    assert_eq!(ans, 64);
}

#[test]
pub fn part2_test2() {
    let ans = part2("input/test2.txt");
    assert_eq!(ans, 106390);
}
