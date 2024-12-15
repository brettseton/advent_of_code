#![allow(clippy::needless_return)]
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs,
    str::FromStr,
};

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
    let hiking_trail = HikingTrail::new(&input);
    let (start_x, start_y) = hiking_trail.get_start();
    let start_step = Step {
        count: 0,
        x: start_x,
        y: start_y,
        history: HashSet::new(),
    };
    return hiking_trail.get_reached(&start_step, &HikingTrail::get_connected);
}

fn part2(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let hiking_trail = HikingTrail::new(&input);
    let (start_x, start_y) = hiking_trail.get_start();
    let start_step = Step {
        count: 0,
        x: start_x,
        y: start_y,
        history: HashSet::new(),
    };

    return hiking_trail.dfs(&start_step);
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
            Self::East => 1,
            Self::South => 2,
            Self::West => 3,
        }
    }

    pub fn get_delta(&self) -> (isize, isize) {
        match self {
            Self::North => (0, -1),
            Self::East => (1, 0),
            Self::South => (0, 1),
            Self::West => (-1, 0),
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
    x: usize,
    y: usize,
    history: HashSet<(usize, usize)>,
}

struct HikingTrail {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl HikingTrail {
    pub fn new(str: &str) -> HikingTrail {
        return HikingTrail::from_str(str).expect("");
    }

    fn get_reached(
        &self,
        start: &Step,
        get_neighbors: &dyn Fn(&Self, &Step) -> Vec<Option<Step>>,
    ) -> usize {
        let mut queue = Vec::new();
        queue.push(start.clone());

        let mut current_max = 0;
        let (goal_x, goal_y) = self.get_end();

        while let Some(step) = queue.pop() {
            if step.x == goal_x && step.y == goal_y && current_max < step.count {
                current_max = step.count;
                println!("current_max: {}", current_max);
            }

            let steps = get_neighbors(self, &step);

            for step in steps.into_iter().flatten() {
                queue.push(step);
            }
        }

        return current_max;
    }

    fn dfs(&self, start: &Step) -> usize {
        let graph = self.get_graph();

        return graph.get_longest(start);
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

        let Some(new_x) = previous.x.checked_add_signed(dx) else {
            return None;
        };
        let Some(new_y) = previous.y.checked_add_signed(dy) else {
            return None;
        };

        if previous.history.contains(&(new_x, new_y)) {
            return None;
        }

        if new_x >= self.width || new_y >= self.height || self.grid[new_y][new_x] == '#' {
            return None;
        }

        match self.grid[previous.y][previous.x] {
            '#' => return None,
            '^' => {
                if traveling != Direction::North {
                    return None;
                }
            }
            '>' => {
                if traveling != Direction::East {
                    return None;
                }
            }
            'v' => {
                if traveling != Direction::South {
                    return None;
                }
            }
            '<' => {
                if traveling != Direction::West {
                    return None;
                }
            }
            '.' => (),
            _ => panic!("not a valid hiking trail"),
        }

        let mut history = previous.history.clone();
        history.insert((new_x, new_y));
        return Some(Step {
            x: new_x,
            y: new_y,
            count: previous.count + 1,
            history,
        });
    }

    pub fn get_connected_snow_boots(&self, step: &Step) -> Vec<Option<Step>> {
        return vec![
            self.get_next_snow_boots(step, Direction::North),
            self.get_next_snow_boots(step, Direction::South),
            self.get_next_snow_boots(step, Direction::East),
            self.get_next_snow_boots(step, Direction::West),
        ];
    }

    pub fn get_next_snow_boots(&self, previous: &Step, traveling: Direction) -> Option<Step> {
        let (dx, dy) = traveling.get_delta();

        let Some(new_x) = previous.x.checked_add_signed(dx) else {
            return None;
        };
        let Some(new_y) = previous.y.checked_add_signed(dy) else {
            return None;
        };

        if new_x >= self.width || new_y >= self.height || self.grid[new_y][new_x] == '#' {
            return None;
        }

        if previous.history.contains(&(new_x, new_y)) {
            return None;
        }
        let mut history = previous.history.clone();
        history.insert((new_x, new_y));
        return Some(Step {
            x: new_x,
            y: new_y,
            count: previous.count + 1,
            history,
        });
    }

    pub fn get_start(&self) -> (usize, usize) {
        for x in 0..self.width {
            if self.grid[0][x] == '.' {
                return (x, 0);
            }
        }
        return (0, 0);
    }

    pub fn get_end(&self) -> (usize, usize) {
        for x in 0..self.width {
            if self.grid[self.height - 1][x] == '.' {
                return (x, self.height - 1);
            }
        }
        return (0, 0);
    }

    pub fn get_graph(&self) -> Graph {
        let mut graph: HashMap<Point2D, Vec<Point3D>> = HashMap::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x] != '#' {
                    let e = graph.entry(Point2D { x, y }).or_default();
                    let current = Step {
                        x,
                        y,
                        count: 0,
                        history: HashSet::new(),
                    };
                    for neighbor in self.get_connected_snow_boots(&current).iter() {
                        match neighbor {
                            Some(n) => e.push(Point3D {
                                x: n.x,
                                y: n.y,
                                z: 1,
                            }),
                            None => (),
                        }
                    }
                }
            }
        }

        // remove all corridors
        let corridors = graph
            .iter()
            .filter(|(_k, v)| v.len() == 2)
            .map(|(&k, _)| k)
            .collect::<Vec<_>>();

        for point in corridors {
            let neighbors = graph.remove(&point).unwrap();
            let n0 = &neighbors[0];
            let n1 = &neighbors[1];

            let node1 = graph.get_mut(&n0.as_point2d()).unwrap();

            if let Some(i) = node1.iter().position(|p| (p.x, p.y) == (point.x, point.y)) {
                node1[i] = Point3D {
                    x: n1.x,
                    y: n1.y,
                    z: n0.z + n1.z,
                };
            }

            let node2 = graph.get_mut(&n1.as_point2d()).unwrap();
            if let Some(i) = node2.iter().position(|p| (p.x, p.y) == (point.x, point.y)) {
                node2[i] = Point3D {
                    x: n0.x,
                    y: n0.y,
                    z: n0.z + n1.z,
                };
            }
        }

        return Graph {
            graph,
            width: self.width,
            height: self.height,
        };
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Point2D {
    x: usize,
    y: usize,
}

struct Point3D {
    x: usize,
    y: usize,
    z: usize,
}

impl Point3D {
    pub fn as_point2d(&self) -> Point2D {
        return Point2D {
            x: self.x,
            y: self.y,
        };
    }
}

#[derive(Debug)]
struct HikingTrailError;

impl FromStr for HikingTrail {
    type Err = HikingTrailError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().nth(0).unwrap().len();
        let height = s.lines().count();
        let grid = s
            .lines()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        return Ok(HikingTrail {
            grid,
            width,
            height,
        });
    }
}

struct Graph {
    graph: HashMap<Point2D, Vec<Point3D>>,
    width: usize,
    height: usize,
}

impl Graph {
    pub fn get_longest(&self, start: &Step) -> usize {
        let mut max_length = 0;
        let mut visited = vec![vec![false; self.width]; self.height];

        self.dfs_recursive(
            &Point2D {
                x: start.x,
                y: start.y,
            },
            &mut visited,
            &mut max_length,
            0,
        );

        return max_length;
    }

    fn dfs_recursive(
        &self,
        start: &Point2D,
        visited: &mut Vec<Vec<bool>>,
        max_length: &mut usize,
        length: usize,
    ) {
        visited[start.y][start.x] = true;
        *max_length = (*max_length).max(length);

        if let Some(neighbors) = self.graph.get(start) {
            for neighbor in neighbors {
                if !visited[neighbor.y][neighbor.x] {
                    self.dfs_recursive(
                        &neighbor.as_point2d(),
                        visited,
                        max_length,
                        length + neighbor.z,
                    );
                }
            }
        }
        visited[start.y][start.x] = false; // Backtrack
    }
}

#[test]
pub fn part1_test1() {
    let ans = part1("input/test1.txt");
    assert_eq!(ans, 94);
}

#[test]
pub fn part1_test2() {
    let ans = part1("input/test2.txt");
    assert_eq!(ans, 2246);
}

#[test]
pub fn part2_test1() {
    let ans = part2("input/test1.txt");
    assert_eq!(ans, 154);
}

#[test]
pub fn part2_test2() {
    let ans = part2("input/test2.txt");
    assert_eq!(ans, 6622);
}
