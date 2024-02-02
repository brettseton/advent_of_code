use std::{cmp::Ordering, collections::BinaryHeap, fs, str::FromStr};

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
    let heat_map = Grid::new(&input);
    return heat_map.get_path(vec![
        State{ x: 0, y: 0, traveling: Direction::East,  current_cost: 0, num_step: 0 },
        State{ x: 0, y: 0, traveling: Direction::South, current_cost: 0, num_step: 0 },
        State{ x: 0, y: 0, traveling: Direction::North, current_cost: 0, num_step: 0 },
        State{ x: 0, y: 0, traveling: Direction::West,  current_cost: 0, num_step: 0 },
        ], 0, 3);
}

fn part2(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let heat_map = Grid::new(&input);
    return heat_map.get_path(vec![
        State{ x: 0, y: 0, traveling: Direction::East,  current_cost: 0, num_step: 0 },
        State{ x: 0, y: 0, traveling: Direction::South, current_cost: 0, num_step: 0 },
        State{ x: 0, y: 0, traveling: Direction::North, current_cost: 0, num_step: 0 },
        State{ x: 0, y: 0, traveling: Direction::West,  current_cost: 0, num_step: 0 },
        ], 4, 10);
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
struct State {
    x: usize,
    y: usize,
    traveling: Direction,
    current_cost: usize,
    num_step: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .current_cost
            .cmp(&self.current_cost)
            .then_with(|| other.x.cmp(&self.x))
            .then_with(|| other.y.cmp(&self.y))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Grid {
    grid: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(str: &str) -> Grid {
        return Grid::from_str(str).expect("");
    }

    fn get_path(&self, start_states: Vec<State>, min_steps: usize, max_steps: usize) -> usize {
        let mut heap = BinaryHeap::new();
        for start_state in start_states.iter() {
            heap.push(State{ x: start_state.x, y: start_state.y, traveling: start_state.traveling.clone(),  current_cost: 0, num_step: 0 });
        }

        let mut visited_map: Vec<Vec<Vec<Vec<usize>>>> =
            vec![vec![vec![vec![usize::MAX; max_steps + 1]; 4]; self.width]; self.height];

        while let Some(state) = heap.pop() {
            if state.x == self.width - 1 && state.y == self.height - 1 {
                return state.current_cost;
            }
            if state.current_cost > visited_map[state.y][state.x][state.traveling.to_usize()][state.num_step] {
                continue;
            }

            let states = self.get_connected_states(&state, min_steps, max_steps);

            for new_state in states.into_iter().flatten() {
                if new_state.current_cost < visited_map[new_state.y][new_state.x][new_state.traveling.to_usize()][new_state.num_step] {
                    visited_map[new_state.y][new_state.x][new_state.traveling.to_usize()][new_state.num_step] = new_state.current_cost;
                    heap.push(new_state);
                }
            }
        }

        return 0;
    }

    pub fn get_connected_states(&self, state: &State, min_steps: usize, max_steps: usize) -> Vec<Option<State>> {
        return match state.traveling {
            Direction::North => vec![
                self.get_state(state, Direction::North, min_steps, max_steps),
                self.get_state(state, Direction::West, min_steps, max_steps),
                self.get_state(state, Direction::East, min_steps, max_steps),
            ],
            Direction::South => vec![
                self.get_state(state, Direction::South, min_steps, max_steps),
                self.get_state(state, Direction::West, min_steps, max_steps),
                self.get_state(state, Direction::East, min_steps, max_steps),
            ],
            Direction::East => vec![
                self.get_state(state, Direction::East, min_steps, max_steps),
                self.get_state(state, Direction::North, min_steps, max_steps),
                self.get_state(state, Direction::South, min_steps, max_steps),
            ],
            Direction::West => vec![
                self.get_state(state, Direction::West, min_steps, max_steps),
                self.get_state(state, Direction::North, min_steps, max_steps),
                self.get_state(state, Direction::South, min_steps, max_steps),
            ],
        };
    }

    pub fn get_state(&self, b: &State, traveling: Direction, min_steps: usize, max_steps: usize) -> Option<State> {

        let (dx, dy) = traveling.get_delta();
        let new_x = b.x.checked_add_signed(dx);
        let new_y = b.y.checked_add_signed(dy);

        // Still in bounds
        if !(new_x.is_some_and(|x| x < self.width) && new_y.is_some_and(|y| y < self.height)) {
            return None;
        }

        // Transition conditions are met
        if (traveling == b.traveling && b.num_step == max_steps)
        || (traveling != b.traveling && b.num_step < min_steps) {
            return None;
        }

        let mut num_step = 1;

        if b.traveling == traveling {
            num_step += b.num_step;
        }

        return Some(State {
            x: new_x.unwrap(),
            y: new_y.unwrap(),
            traveling,
            current_cost: b.current_cost + self.grid[new_y.unwrap()][new_x.unwrap()],
            num_step,
        });
    }
}

#[derive(Debug)]
struct GridError;

impl FromStr for Grid {
    type Err = GridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().nth(0).unwrap().len();
        let height = s.lines().count();
        let grid = s
            .lines()
            .map(|s| {
                s.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();
        return Ok(Grid {
            grid,
            width,
            height,
        });
    }
}

#[test]
pub fn part1_test1() {
    let ans = part1("input/test1.txt");
    assert_eq!(ans, 102);
}

#[test]
pub fn part1_test2() {
    let ans = part1("input/test2.txt");
    assert_eq!(ans, 1260);
}

#[test]
pub fn part2_test1() {
    let ans = part2("input/test1.txt");
    assert_eq!(ans, 94);
}

#[test]
pub fn part2_test2() {
    let ans = part2("input/test2.txt");
    assert_eq!(ans, 1416);
}
