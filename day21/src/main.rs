use std::{cmp::Ordering, fs, str::FromStr, collections::VecDeque};

fn main() {
    let ans = part1("input/test1.txt", 6);
    println!("part 1 test 1 : {}", ans);

    let ans = part1("input/test2.txt", 64);
    println!("part 1 test 2 : {}", ans);

    let ans = part2("input/test1.txt");
    println!("part 2 test 1 : {}", ans);

    let ans = part2("input/test2.txt");
    println!("part 2 test 2 : {}", ans);
}

fn part1(file_path: &str, steps: usize) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let garden = Garden::new(&input);
    let (start_x, start_y) = garden.get_start();
    let start_step = Step {count: 0, x: start_x, y: start_y };
    return garden.get_reached(start_step, steps);
}

fn part2(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let garden = Garden::new(&input);
    let (start_x, start_y) = garden.get_start();
    let start_step = Step {count: 0, x: start_x, y: start_y };
    return garden.get_reached(start_step, 0);
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
struct Step {
    count: usize,
    x: usize,
    y: usize,
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

    fn get_reached(&self, start: Step, max_steps: usize) -> usize {
        let mut queue = VecDeque::new();
        queue.push_back(start);

        let mut visited_map: Vec<Vec<Vec<usize>>> =
        vec![vec![vec![usize::MAX; max_steps + 1]; self.width]; self.height];

        while let Some(step) = queue.pop_front() {
            if step.count == max_steps {
                break;
            }

            let steps = self.get_connected(&step);

            for step in steps {
                match step {
                    
                    Some(s) => {
                        if s.count < visited_map[s.y][s.x][s.count] {
                            visited_map[s.y][s.x][s.count] = s.count;
                            queue.push_back(s);
                        }
                    },
                    None => (),
                }
            }
        }
        
        return queue.len() + 1;
    }

    pub fn get_connected(&self, step: &Step) -> Vec<Option<Step>> {
        return vec![
                self.get_next(step, Direction::North),
                self.get_next(step, Direction::South),
                self.get_next(step, Direction::East ),
                self.get_next(step, Direction::West ),
            ];
    }

    pub fn get_next(&self, previous: &Step, traveling: Direction) -> Option<Step> {
        let (dx, dy) = traveling.get_delta();

        let new_x = previous.x.checked_add_signed(dx);
        let new_y = previous.y.checked_add_signed(dy);
        if new_x.is_some_and(|x| x <self.width)
          && new_y.is_some_and(|y| y <self.height)
          && self.grid[new_y.unwrap()][new_x.unwrap()] != '#' {
            return Some(Step { x: new_x.unwrap(), y: new_y.unwrap(), count: previous.count + 1});
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
        return (0,0);
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
    assert_eq!(ans, 0);
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
