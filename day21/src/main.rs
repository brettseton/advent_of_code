use std::{cmp::Ordering, collections::VecDeque, fs, str::FromStr};

fn main() {
    let ans = part1("input/test1.txt", 6);
    println!("part 1 test 1 : {}", ans);

    let ans = part1("input/test2.txt", 64);
    println!("part 1 test 2 : {}", ans);

    let ans = part2_1("input/test1.txt", 500);
    println!("part 2 test 1 : {}", ans);

    let ans = part2("input/test2.txt");
    println!("part 2 test 2 : {}", ans);
}

fn part1(file_path: &str, steps: usize) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let garden = Garden::new(&input);
    let (start_x, start_y) = garden.get_start();
    let start_step = Step {
        count: 0,
        x: start_x as isize,
        y: start_y as isize,
    };
    return garden.get_reached(&start_step, steps, &Garden::get_connected);
}

fn part2_1(file_path: &str, steps: usize) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let garden = Garden::new(&input);
    let (start_x, start_y) = garden.get_start();
    let start_step = Step {
        count: 0,
        x: start_x as isize,
        y: start_y as isize,
    };

    return garden.get_reached(&start_step, steps, &Garden::get_connected_wrapping);
}

fn part2(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let garden = Garden::new(&input);
    let (start_x, start_y) = garden.get_start();
    let start_step = Step {
        count: 0,
        x: start_x as isize,
        y: start_y as isize,
    };

    let n = 26501365;
    let x_max = n / garden.width;
    let rem = n % garden.width;
    println!("div: {}, rem: {}", x_max, rem);

    let mut y = vec![];
    for x in [0, 2, 4] {
        y.push(garden.get_reached(
            &start_step,
            garden.width * x + rem,
            &Garden::get_connected_wrapping,
        ));
    }

    // Equation is in the following form
    // y(x) = a*x^2 + bx + c
    // find a, b, c using x and y
    // points for my input are {x, y} => {0, 3921}, {2, 96749}, {4, 312993}
    // when x = 0, c = y[0] = 3921
    // y = 0a + 0b + c
    // c = 3921                        -- used below
    //
    // when x = 2, y = y[1] = 96749
    // y = 4a + 2b + c
    // rearrange
    // 2b = y - 4a - c
    // 2b = 96749 - 4a - c             -- Equation 1
    //
    // when x = 4, y = y[2] = 312993
    // y = 16a + 4b + c
    // sub in Equation 1 and then we solve for a
    // y = 16a + 2(96749 - 4a - c) + c
    // y = 8a + 2(96749) - c
    // 8a = y - 2(96749) + c
    // a = (y - 2(96749) + c)/8
    // a = (y[2] - 2 * y[1] + c)/ 8    -- used below
    //
    // use this for b in Equation 1
    // 2b = 96749 - 4a - c
    // b = (96749 - 4a - c)/ 2
    // b = (y[1] - 4a - c)/ 2          -- used below

    let c = y[0]; // 3921;
    let a = (y[2] - 2 * y[1] + c) / 8; // (312993 - 2 * 96749 + c ) / 8
    let b = (y[1] - 4 * a - c) / 2; // (96749 - 4 * a - c)/2

    println!("{}x^2 + {}x + {}", a, b, c);

    return a * x_max * x_max + b * x_max + c;
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
struct Step {
    count: usize,
    x: isize,
    y: isize,
}

struct Garden {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Garden {
    pub fn new(str: &str) -> Garden {
        return Garden::from_str(str).expect("");
    }

    fn get_reached(&self, start: &Step, max_steps: usize, get_neighbors: & dyn Fn(&Self, &Step) -> Vec<Option<Step>>) -> usize {
        let mut queue = VecDeque::new();
        queue.push_back(start.clone());

        let map_count = ((max_steps / self.width) + 1) * 4;
        let mut visited_map: Vec<Vec<usize>> =
            vec![vec![usize::MAX; self.width * map_count]; self.height * map_count];

        let (start_x, start_y) = self.get_start();
        let start_x_offset = (self.width * map_count) / 2 + start_x;
        let start_y_offset = (self.height * map_count) / 2 + start_y;

        while let Some(step) = queue.pop_front() {
            if step.count == max_steps {
                break;
            }

            let steps = get_neighbors(self, &step);

            for step in steps {
                match step {
                    Some(s) => {
                        let grid_x = start_x_offset.checked_add_signed(s.x).unwrap();
                        let grid_y = start_y_offset.checked_add_signed(s.y).unwrap();
                        if s.count < visited_map[grid_y][grid_x] {
                            visited_map[grid_y][grid_x] = s.count;
                            queue.push_back(s);
                        }
                    }
                    None => (),
                }
            }
        }

        let mut even_count: usize = 0;
        let mut odd_count: usize = 0;
        for line in visited_map.iter() {
            let _out: String = line
                .iter()
                .map(|&x| {
                    if x == usize::MAX {
                        return '#';
                    }
                    match x % 2 {
                        0 => {
                            even_count += 1;
                            return 'E';
                        }
                        1 => {
                            odd_count += 1;
                            return 'O';
                        }
                        _ => panic!(),
                    }
                })
                .collect();
            //println!("{}", _out);
        }

        println!(
            "steps: {}, even: {}, odd: {}",
            max_steps, even_count, odd_count
        );
        if max_steps % 2 == 1 {
            return odd_count;
        } else {
            return even_count;
        }
    }

    pub fn get_connected(&self, step: &Step) -> Vec<Option<Step>> {
        return vec![
            self.get_next(step, Direction::North),
            self.get_next(step, Direction::South),
            self.get_next(step, Direction::East),
            self.get_next(step, Direction::West),
        ];
    }

    pub fn get_next(&self, previous: &Step, traveling: Direction) -> Option<Step> {
        let (dx, dy) = traveling.get_delta();

        let new_x = previous.x + dx;
        let new_y = previous.y + dy;
        if new_x >= 0
            && new_x < self.width.try_into().unwrap()
            && new_y >= 0
            && new_y < self.height.try_into().unwrap()
            && self.grid[new_y as usize][new_x as usize] != '#'
        {
            return Some(Step {
                x: new_x,
                y: new_y,
                count: previous.count + 1,
            });
        }

        return None;
    }

    pub fn get_connected_wrapping(&self, step: &Step) -> Vec<Option<Step>> {
        return vec![
            self.get_next_wrapping(step, Direction::North),
            self.get_next_wrapping(step, Direction::South),
            self.get_next_wrapping(step, Direction::East),
            self.get_next_wrapping(step, Direction::West),
        ];
    }

    pub fn get_next_wrapping(&self, previous: &Step, traveling: Direction) -> Option<Step> {
        let (dx, dy) = traveling.get_delta();

        let new_x: isize = previous.x + dx;
        let new_y: isize = previous.y + dy;
        let grid_x: usize = usize::try_from(new_x.rem_euclid(self.width.try_into().unwrap()))
            .expect("this is usize");
        let grid_y: usize = usize::try_from(new_y.rem_euclid(self.height.try_into().unwrap()))
            .expect("this is usize");
        if self.grid[grid_y][grid_x] != '#' {
            return Some(Step {
                x: new_x,
                y: new_y,
                count: previous.count + 1,
            });
        }

        return None;
    }

    pub fn get_start(&self) -> (usize, usize) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x] == 'S' {
                    return (x, y);
                }
            }
        }
        return (0, 0);
    }
}

#[derive(Debug)]
struct GardenError;

impl FromStr for Garden {
    type Err = GardenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().nth(0).unwrap().len();
        let height = s.lines().count();
        let grid = s
            .lines()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        return Ok(Garden {
            grid,
            width,
            height,
        });
    }
}

#[test]
pub fn part1_test1() {
    let ans = part1("input/test1.txt", 6);
    assert_eq!(ans, 16);
}

#[test]
pub fn part1_test2() {
    let ans = part1("input/test2.txt", 64);
    assert_eq!(ans, 3788);
}

#[test]
pub fn part2_test1() {
    let ans = part2_1("input/test1.txt", 500);
    assert_eq!(ans, 167004);
}

#[test]
pub fn part2_test2() {
    let ans = part2("input/test2.txt");
    assert_eq!(ans, 631357596621921);
}
