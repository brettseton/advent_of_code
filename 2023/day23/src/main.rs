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
    return hiking_trail.dfs();
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

    fn dfs(&self) -> usize {
        let graph = self.get_graph();

        return graph.get_longest();
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

        let new_x = previous.x.checked_add_signed(dx)?;
        let new_y = previous.y.checked_add_signed(dy)?;

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
        let (sx, sy) = self.get_start();
        let (ex, ey) = self.get_end();
        let start_pos = Point2D { x: sx, y: sy };
        let end_pos = Point2D { x: ex, y: ey };

        let mut junctions = vec![start_pos, end_pos];
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x] == '#' {
                    continue;
                }
                let p = Point2D { x, y };
                if p == start_pos || p == end_pos {
                    continue;
                }

                let mut neighbors = 0;
                for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx >= 0
                        && nx < self.width as isize
                        && ny >= 0
                        && ny < self.height as isize
                        && self.grid[ny as usize][nx as usize] != '#'
                    {
                        neighbors += 1;
                    }
                }
                if neighbors > 2 {
                    junctions.push(p);
                }
            }
        }

        let node_to_idx: HashMap<Point2D, usize> =
            junctions.iter().enumerate().map(|(i, &p)| (p, i)).collect();

        let mut adj = vec![Vec::new(); junctions.len()];
        let mut adj_masks = vec![0u128; junctions.len()];

        for (i, &start_p) in junctions.iter().enumerate() {
            let mut stack = vec![(start_p, 0)];
            let mut visited = HashSet::new();
            visited.insert(start_p);

            while let Some((curr_p, dist)) = stack.pop() {
                if dist > 0 && node_to_idx.contains_key(&curr_p) {
                    let j_idx = node_to_idx[&curr_p];
                    adj[i].push((j_idx, dist));
                    adj_masks[i] |= 1u128 << j_idx;
                    continue;
                }

                for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let nx = curr_p.x as isize + dx;
                    let ny = curr_p.y as isize + dy;
                    if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                        let np = Point2D {
                            x: nx as usize,
                            y: ny as usize,
                        };
                        if self.grid[np.y][np.x] != '#' && !visited.contains(&np) {
                            visited.insert(np);
                            stack.push((np, dist + 1));
                        }
                    }
                }
            }
            // Sort neighbors by weight descending to find longer paths sooner
            adj[i].sort_by(|a, b| b.1.cmp(&a.1));
        }

        let max_edge_weights = adj
            .iter()
            .map(|neighbors| neighbors.iter().map(|&(_, w)| w).max().unwrap_or(0))
            .collect();

        let start_idx = node_to_idx[&start_pos];
        let end_idx = node_to_idx[&end_pos];

        return Graph {
            adj,
            adj_masks,
            max_edge_weights,
            start_idx,
            end_idx,
        };
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Point2D {
    x: usize,
    y: usize,
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
    adj: Vec<Vec<(usize, usize)>>,
    adj_masks: Vec<u128>,
    max_edge_weights: Vec<usize>,
    start_idx: usize,
    end_idx: usize,
}

impl Graph {
    pub fn get_longest(&self) -> usize {
        let mut max_length = 0;
        assert!(self.adj.len() <= 128, "Graph too large for u128 bitmask");

        // Precompute total potential to avoid redundant calculations
        let total_potential: usize = self.max_edge_weights.iter().sum();

        self.dfs_recursive(self.start_idx, 0, 0, total_potential, &mut max_length);

        return max_length;
    }

    fn get_reachable_potential(&self, current: usize, visited: u128) -> (bool, usize) {
        let target = 1u128 << self.end_idx;
        let mut seen = visited | (1u128 << current);
        let mut queue = self.adj_masks[current] & !seen;

        let mut potential = self.max_edge_weights[current];
        let mut reached_target = (self.adj_masks[current] & !visited) & target != 0;

        seen |= queue;
        let mut all_reachable = queue;
        while queue != 0 {
            let u = queue.trailing_zeros() as usize;
            queue &= !(1u128 << u);

            let neighbors = self.adj_masks[u] & !seen;
            if neighbors & target != 0 {
                reached_target = true;
            }
            queue |= neighbors;
            seen |= neighbors;
            all_reachable |= neighbors;
        }

        // Sum weights of all nodes in the reachable component
        let mut temp = all_reachable;
        while temp != 0 {
            let u = temp.trailing_zeros() as usize;
            temp &= !(1u128 << u);
            potential += self.max_edge_weights[u];
        }

        (reached_target, potential)
    }

    fn dfs_recursive(
        &self,
        current: usize,
        visited: u128,
        length: usize,
        remaining_potential: usize,
        max_length: &mut usize,
    ) {
        if current == self.end_idx {
            *max_length = (*max_length).max(length);
            return;
        }

        // Pruning: if current length + theoretical max remaining < current max, stop
        if length + remaining_potential <= *max_length {
            return;
        }

        let mut available_neighbors: u128 = 0;
        let mut num_neighbors = 0;
        for &(next, _) in &self.adj[current] {
            if (visited & (1u128 << next)) == 0 {
                available_neighbors |= 1u128 << next;
                num_neighbors += 1;
            }
        }

        if num_neighbors == 0 {
            return;
        }

        // Reachability and potential pruning - only if we have choices or to check dead ends
        if num_neighbors > 1 {
            let (reachable, potential) = self.get_reachable_potential(current, visited);
            if !reachable || length + potential <= *max_length {
                return;
            }
        }

        let mask = 1u128 << current;
        let new_visited = visited | mask;
        let next_potential = remaining_potential - self.max_edge_weights[current];

        for &(next, dist) in &self.adj[current] {
            if (available_neighbors & (1u128 << next)) != 0 {
                self.dfs_recursive(next, new_visited, length + dist, next_potential, max_length);
            }
        }
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
