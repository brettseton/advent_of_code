use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(Direction::Left),
            ">" => Ok(Direction::Right),
            _ => Err(format!("Invalid direction: {}", s)),
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
    relative_heights: [i64; 7],
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

    fn simulate(&mut self, jets: &[Direction], num_rocks: i64) -> i64 {
        let mut i = 0;
        while i < num_rocks {
            let rock_type = ROCKS[(i % 5) as usize];
            let rock_coords = rock_type.coords();

            if self.extra_height == 0 {
                let mut rel_h = [0i64; 7];
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
                    let remaining_rocks = num_rocks - i;
                    let num_cycles = remaining_rocks / cycle_len;

                    self.extra_height = num_cycles * cycle_h;
                    i += num_cycles * cycle_len;
                } else {
                    self.history.insert(state, (i, self.max_height));
                }
            }

            if i >= num_rocks {
                break;
            }

            let mut curr_pos = Point::new(2, self.max_height + 4);

            loop {
                let jet = jets[self.jet_idx];
                self.jet_idx = (self.jet_idx + 1) % jets.len();

                let next_x = match jet {
                    Direction::Left => curr_pos.x - 1,
                    Direction::Right => curr_pos.x + 1,
                };

                let mut can_move_x = true;
                for delta in rock_coords {
                    let x = next_x + delta.x;
                    let y = curr_pos.y + delta.y;
                    if !(0..7).contains(&x) || self.stopped_rocks.contains(&Point::new(x, y)) {
                        can_move_x = false;
                        break;
                    }
                }
                if can_move_x {
                    curr_pos.x = next_x;
                }

                let next_y = curr_pos.y - 1;
                let mut can_fall = true;
                if next_y <= 0 {
                    can_fall = false;
                } else {
                    for delta in rock_coords {
                        let x = curr_pos.x + delta.x;
                        let y = next_y + delta.y;
                        if self.stopped_rocks.contains(&Point::new(x, y)) {
                            can_fall = false;
                            break;
                        }
                    }
                }

                if can_fall {
                    curr_pos.y = next_y;
                } else {
                    for delta in rock_coords {
                        let p = Point::new(curr_pos.x + delta.x, curr_pos.y + delta.y);
                        self.stopped_rocks.insert(p);
                        if p.y > self.max_height {
                            self.max_height = p.y;
                        }
                    }
                    break;
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
        .filter_map(|c| Direction::from_str(&c.to_string()).ok())
        .collect()
}

fn part1(input: &str) -> i64 {
    let jets = parse_input(input);
    Chamber::new().simulate(&jets, 2022)
}

fn part2(input: &str) -> i64 {
    let jets = parse_input(input);
    Chamber::new().simulate(&jets, 1000000000000)
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
