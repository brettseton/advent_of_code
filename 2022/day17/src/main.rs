use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

const CHAMBER_WIDTH: i64 = 7;
const INITIAL_X_OFFSET: i64 = 2;
const INITIAL_Y_OFFSET: i64 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    const fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err(format!("Invalid direction: {}", c)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RockType {
    HorizontalBar,
    Plus,
    L,
    VerticalBar,
    Square,
}

const ROCKS: [RockType; 5] = [
    RockType::HorizontalBar,
    RockType::Plus,
    RockType::L,
    RockType::VerticalBar,
    RockType::Square,
];

impl RockType {
    fn coords(&self) -> &'static [Point] {
        match self {
            RockType::HorizontalBar => &[
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
                Point { x: 3, y: 0 },
            ],
            RockType::Plus => &[
                Point { x: 1, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 1 },
                Point { x: 2, y: 1 },
                Point { x: 1, y: 2 },
            ],
            RockType::L => &[
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
                Point { x: 2, y: 1 },
                Point { x: 2, y: 2 },
            ],
            RockType::VerticalBar => &[
                Point { x: 0, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 0, y: 2 },
                Point { x: 0, y: 3 },
            ],
            RockType::Square => &[
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 1 },
            ],
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct State {
    rock_type: RockType,
    jet_idx: usize,
    relative_heights: [i64; CHAMBER_WIDTH as usize],
}

struct Chamber {
    stopped_rocks: HashSet<Point>,
    max_height: i64,
    jet_idx: usize,
    history: HashMap<State, (i64, i64)>,
    extra_height: i64,
}

impl Chamber {
    fn new() -> Self {
        Self {
            stopped_rocks: HashSet::new(),
            max_height: 0,
            jet_idx: 0,
            history: HashMap::new(),
            extra_height: 0,
        }
    }

    fn is_colliding(&self, pos: Point, rock_coords: &[Point]) -> bool {
        rock_coords.iter().any(|&delta| {
            let p = pos + delta;
            !(0..CHAMBER_WIDTH).contains(&p.x) || p.y <= 0 || self.stopped_rocks.contains(&p)
        })
    }

    fn simulate(&mut self, jets: &[Direction], num_rocks: i64) -> i64 {
        let mut i = 0;
        while i < num_rocks {
            let rock_type = ROCKS[(i % 5) as usize];
            let rock_coords = rock_type.coords();

            if self.extra_height == 0 {
                let mut rel_h = [0i64; CHAMBER_WIDTH as usize];
                for (x, h) in rel_h.iter_mut().enumerate() {
                    let mut depth = 0;
                    while depth < 100 {
                        if self
                            .stopped_rocks
                            .contains(&Point::new(x as i64, self.max_height - depth))
                        {
                            break;
                        }
                        depth += 1;
                    }
                    *h = depth;
                }

                let state = State {
                    rock_type,
                    jet_idx: self.jet_idx,
                    relative_heights: rel_h,
                };

                if let Some(&(old_i, old_h)) = self.history.get(&state) {
                    let cycle_len = i - old_i;
                    let cycle_h = self.max_height - old_h;
                    let num_cycles = (num_rocks - i) / cycle_len;

                    self.extra_height = num_cycles * cycle_h;
                    i += num_cycles * cycle_len;
                } else {
                    self.history.insert(state, (i, self.max_height));
                }
            }

            if i >= num_rocks {
                break;
            }

            let mut curr_pos = Point::new(INITIAL_X_OFFSET, self.max_height + INITIAL_Y_OFFSET);

            loop {
                let jet = jets[self.jet_idx];
                self.jet_idx = (self.jet_idx + 1) % jets.len();

                let next_x = match jet {
                    Direction::Left => curr_pos.x - 1,
                    Direction::Right => curr_pos.x + 1,
                };

                if !self.is_colliding(Point::new(next_x, curr_pos.y), rock_coords) {
                    curr_pos.x = next_x;
                }

                let next_y = curr_pos.y - 1;
                if self.is_colliding(Point::new(curr_pos.x, next_y), rock_coords) {
                    for &delta in rock_coords {
                        let p = curr_pos + delta;
                        self.stopped_rocks.insert(p);
                        self.max_height = self.max_height.max(p.y);
                    }
                    break;
                } else {
                    curr_pos.y = next_y;
                }
            }
            i += 1;
        }

        self.max_height + self.extra_height
    }
}

fn parse_input(input: &str) -> Vec<Direction> {
    input
        .trim()
        .chars()
        .filter_map(|c| Direction::try_from(c).ok())
        .collect()
}

fn part1(input: &str) -> i64 {
    let jets = parse_input(input);
    Chamber::new().simulate(&jets, 2022)
}

fn part2(input: &str) -> i64 {
    let jets = parse_input(input);
    Chamber::new().simulate(&jets, 1_000_000_000_000)
}

fn main() {
    println!("Part 1 test 1: {}", part1(TEST_INPUT_1));
    println!("Part 1 test 2: {}", part1(TEST_INPUT_2));

    println!("Part 2 test 1: {}", part2(TEST_INPUT_1));
    println!("Part 2 test 2: {}", part2(TEST_INPUT_2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() {
        assert_eq!(part1(TEST_INPUT_1), 3068);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 3191);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 1514285714288);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 1572093023267);
    }
}
