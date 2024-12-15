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
            Self::East => 1,
            Self::South => 2,
            Self::West => 3,
        }
    }
}

struct Contraption {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

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
        let max_down = (0..self.width)
            .map(|x| {
                self.get_num_energized(Beam {
                    x,
                    y: 0,
                    traveling: Direction::South,
                })
            })
            .max()
            .unwrap();
        let max_up = (0..self.width)
            .map(|x| {
                self.get_num_energized(Beam {
                    x,
                    y: self.height - 1,
                    traveling: Direction::North,
                })
            })
            .max()
            .unwrap();
        let max_right = (0..self.height)
            .map(|y| {
                self.get_num_energized(Beam {
                    x: 0,
                    y: y,
                    traveling: Direction::East,
                })
            })
            .max()
            .unwrap();
        let max_left = (0..self.height)
            .map(|y| {
                self.get_num_energized(Beam {
                    x: self.width - 1,
                    y: y,
                    traveling: Direction::West,
                })
            })
            .max()
            .unwrap();

        let max = *[max_down, max_up, max_right, max_left]
            .iter()
            .max()
            .unwrap();
        return max;
    }

    fn get_num_energized(&self, start_beam: Beam) -> usize {
        let mut queue = vec![start_beam];
        let mut visited_map: Vec<Vec<Vec<bool>>> =
            vec![vec![vec![false; 4]; self.width]; self.height];

        while let Some(beam) = queue.pop() {
            if visited_map[beam.y][beam.x][beam.traveling.to_usize()] {
                continue;
            } else {
                visited_map[beam.y][beam.x][beam.traveling.to_usize()] = true;
            }

            let beams = self.get_connected_beams(&beam);

            for beam in beams.into_iter().flatten() {
                queue.push(beam)
            }
        }

        let energized: usize = visited_map
            .iter()
            .map(|x| x.iter().filter(|y| y.iter().any(|&v| v)).count())
            .sum();
        return energized;
    }

    pub fn get_connected_beams(&self, beam: &Beam) -> Vec<Option<Beam>> {
        match self.grid[beam.y][beam.x] {
            '.' => {
                return match beam.traveling {
                    Direction::North => {
                        vec![self.get_beam(beam.x, 0, beam.y, -1, Direction::North)]
                    }
                    Direction::South => vec![self.get_beam(beam.x, 0, beam.y, 1, Direction::South)],
                    Direction::East => vec![self.get_beam(beam.x, 1, beam.y, 0, Direction::East)],
                    Direction::West => vec![self.get_beam(beam.x, -1, beam.y, 0, Direction::West)],
                };
            }
            '\\' => {
                return match beam.traveling {
                    Direction::North => vec![self.get_beam(beam.x, -1, beam.y, 0, Direction::West)],
                    Direction::South => vec![self.get_beam(beam.x, 1, beam.y, 0, Direction::East)],
                    Direction::East => vec![self.get_beam(beam.x, 0, beam.y, 1, Direction::South)],
                    Direction::West => vec![self.get_beam(beam.x, 0, beam.y, -1, Direction::North)],
                };
            }
            '/' => {
                return match beam.traveling {
                    Direction::North => vec![self.get_beam(beam.x, 1, beam.y, 0, Direction::East)],
                    Direction::South => vec![self.get_beam(beam.x, -1, beam.y, 0, Direction::West)],
                    Direction::East => vec![self.get_beam(beam.x, 0, beam.y, -1, Direction::North)],
                    Direction::West => vec![self.get_beam(beam.x, 0, beam.y, 1, Direction::South)],
                };
            }
            '|' => {
                return match beam.traveling {
                    Direction::North => {
                        vec![self.get_beam(beam.x, 0, beam.y, -1, Direction::North)]
                    }
                    Direction::South => vec![self.get_beam(beam.x, 0, beam.y, 1, Direction::South)],
                    Direction::East => vec![
                        self.get_beam(beam.x, 0, beam.y, -1, Direction::North),
                        self.get_beam(beam.x, 0, beam.y, 1, Direction::South),
                    ],
                    Direction::West => vec![
                        self.get_beam(beam.x, 0, beam.y, -1, Direction::North),
                        self.get_beam(beam.x, 0, beam.y, 1, Direction::South),
                    ],
                };
            }
            '-' => {
                return match beam.traveling {
                    Direction::North => vec![
                        self.get_beam(beam.x, 1, beam.y, 0, Direction::East),
                        self.get_beam(beam.x, -1, beam.y, 0, Direction::West),
                    ],
                    Direction::South => vec![
                        self.get_beam(beam.x, 1, beam.y, 0, Direction::East),
                        self.get_beam(beam.x, -1, beam.y, 0, Direction::West),
                    ],
                    Direction::East => vec![self.get_beam(beam.x, 1, beam.y, 0, Direction::East)],
                    Direction::West => vec![self.get_beam(beam.x, -1, beam.y, 0, Direction::West)],
                };
            }
            _ => panic!("unexpected character"),
        }
    }

    pub fn get_beam(
        &self,
        x: usize,
        dx: isize,
        y: usize,
        dy: isize,
        traveling: Direction,
    ) -> Option<Beam> {
        let new_x = x.checked_add_signed(dx);
        let new_y = y.checked_add_signed(dy);
        if new_x.is_some_and(|x| x < self.width) && new_y.is_some_and(|y| y < self.height) {
            return Some(Beam {
                x: new_x.unwrap(),
                y: new_y.unwrap(),
                traveling,
            });
        }

        return None;
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
