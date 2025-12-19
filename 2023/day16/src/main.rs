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
    let contraption = Contraption::new(&input);
    return contraption.get_num_energized(Beam {
        x: 0,
        y: 0,
        traveling: Direction::East,
    });
}

fn part2(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let contraption = Contraption::new(&input);
    return contraption.get_max_energized();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn to_bit(self) -> u8 {
        match self {
            Self::North => 1,
            Self::East => 2,
            Self::South => 4,
            Self::West => 8,
        }
    }
}

struct Contraption {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

#[derive(Clone, Copy)]
struct Beam {
    x: usize,
    y: usize,
    traveling: Direction,
}

impl Contraption {
    pub fn new(str: &str) -> Contraption {
        return Contraption::from_str(str).expect("");
    }

    fn get_max_energized(&self) -> usize {
        let mut starts = Vec::with_capacity(2 * (self.width + self.height));

        for x in 0..self.width {
            starts.push(Beam {
                x,
                y: 0,
                traveling: Direction::South,
            });
            starts.push(Beam {
                x,
                y: self.height - 1,
                traveling: Direction::North,
            });
        }
        for y in 0..self.height {
            starts.push(Beam {
                x: 0,
                y,
                traveling: Direction::East,
            });
            starts.push(Beam {
                x: self.width - 1,
                y,
                traveling: Direction::West,
            });
        }

        starts
            .into_iter()
            .map(|beam| self.get_num_energized(beam))
            .max()
            .unwrap_or(0)
    }

    fn get_num_energized(&self, start_beam: Beam) -> usize {
        let mut queue = vec![start_beam];
        let mut visited = vec![0u8; self.width * self.height];

        while let Some(beam) = queue.pop() {
            let idx = beam.y * self.width + beam.x;
            let bit = beam.traveling.to_bit();

            if visited[idx] & bit != 0 {
                continue;
            }
            visited[idx] |= bit;

            self.push_next_beams(&beam, &mut queue);
        }

        visited.iter().filter(|&&v| v != 0).count()
    }

    fn push_next_beams(&self, beam: &Beam, queue: &mut Vec<Beam>) {
        let x = beam.x;
        let y = beam.y;

        match self.grid[y][x] {
            '.' => match beam.traveling {
                Direction::North => self.push_beam(x, 0, y, -1, Direction::North, queue),
                Direction::South => self.push_beam(x, 0, y, 1, Direction::South, queue),
                Direction::East => self.push_beam(x, 1, y, 0, Direction::East, queue),
                Direction::West => self.push_beam(x, -1, y, 0, Direction::West, queue),
            },
            '\\' => match beam.traveling {
                Direction::North => self.push_beam(x, -1, y, 0, Direction::West, queue),
                Direction::South => self.push_beam(x, 1, y, 0, Direction::East, queue),
                Direction::East => self.push_beam(x, 0, y, 1, Direction::South, queue),
                Direction::West => self.push_beam(x, 0, y, -1, Direction::North, queue),
            },
            '/' => match beam.traveling {
                Direction::North => self.push_beam(x, 1, y, 0, Direction::East, queue),
                Direction::South => self.push_beam(x, -1, y, 0, Direction::West, queue),
                Direction::East => self.push_beam(x, 0, y, -1, Direction::North, queue),
                Direction::West => self.push_beam(x, 0, y, 1, Direction::South, queue),
            },
            '|' => match beam.traveling {
                Direction::North => self.push_beam(x, 0, y, -1, Direction::North, queue),
                Direction::South => self.push_beam(x, 0, y, 1, Direction::South, queue),
                Direction::East | Direction::West => {
                    self.push_beam(x, 0, y, -1, Direction::North, queue);
                    self.push_beam(x, 0, y, 1, Direction::South, queue);
                }
            },
            '-' => match beam.traveling {
                Direction::East => self.push_beam(x, 1, y, 0, Direction::East, queue),
                Direction::West => self.push_beam(x, -1, y, 0, Direction::West, queue),
                Direction::North | Direction::South => {
                    self.push_beam(x, 1, y, 0, Direction::East, queue);
                    self.push_beam(x, -1, y, 0, Direction::West, queue);
                }
            },
            _ => panic!("unexpected character"),
        }
    }

    fn push_beam(
        &self,
        x: usize,
        dx: isize,
        y: usize,
        dy: isize,
        traveling: Direction,
        queue: &mut Vec<Beam>,
    ) {
        if let Some(new_x) = x.checked_add_signed(dx) {
            if let Some(new_y) = y.checked_add_signed(dy) {
                if new_x < self.width && new_y < self.height {
                    queue.push(Beam {
                        x: new_x,
                        y: new_y,
                        traveling,
                    });
                }
            }
        }
    }
}

#[derive(Debug)]
struct ContraptionError;

impl FromStr for Contraption {
    type Err = ContraptionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().nth(0).unwrap().len();
        let height = s.lines().count();
        let grid = s
            .lines()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        return Ok(Contraption {
            grid,
            width,
            height,
        });
    }
}

#[test]
pub fn part1_test1() {
    let ans = part1("input/test1.txt");
    assert_eq!(ans, 46);
}

#[test]
pub fn part1_test2() {
    let ans = part1("input/test2.txt");
    assert_eq!(ans, 8539);
}

#[test]
pub fn part2_test1() {
    let ans = part2("input/test1.txt");
    assert_eq!(ans, 51);
}

#[test]
pub fn part2_test2() {
    let ans = part2("input/test2.txt");
    assert_eq!(ans, 8674);
}
