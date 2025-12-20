use std::collections::VecDeque;
use std::str::FromStr;

const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Direction {
    dx: i32,
    dy: i32,
}

impl Direction {
    const UP: Self = Self { dx: 0, dy: -1 };
    const DOWN: Self = Self { dx: 0, dy: 1 };
    const LEFT: Self = Self { dx: -1, dy: 0 };
    const RIGHT: Self = Self { dx: 1, dy: 0 };

    const ALL: [Self; 4] = [Self::UP, Self::DOWN, Self::LEFT, Self::RIGHT];
}

trait SearchGoal {
    fn is_reached(&self, pos: Pos, hm: &HeightMap) -> bool;
}

trait MoveConstraint {
    fn can_move(&self, current: u8, next: u8) -> bool;
}

struct ReachTargetPos(Pos);
impl SearchGoal for ReachTargetPos {
    fn is_reached(&self, pos: Pos, _hm: &HeightMap) -> bool {
        pos == self.0
    }
}

struct ReachElevation(u8);
impl SearchGoal for ReachElevation {
    fn is_reached(&self, pos: Pos, hm: &HeightMap) -> bool {
        hm.elevations[pos.y][pos.x] == self.0
    }
}

struct ForwardMove;
impl MoveConstraint for ForwardMove {
    fn can_move(&self, current: u8, next: u8) -> bool {
        next <= current + 1
    }
}

struct BackwardMove;
impl MoveConstraint for BackwardMove {
    fn can_move(&self, current: u8, next: u8) -> bool {
        next >= current.saturating_sub(1)
    }
}

struct HeightMap {
    elevations: Vec<Vec<u8>>,
    start: Pos,
    end: Pos,
    rows: usize,
    cols: usize,
}

impl FromStr for HeightMap {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
        if grid.is_empty() || grid[0].is_empty() {
            return Err("Empty grid");
        }
        let rows = grid.len();
        let cols = grid[0].len();

        let mut start = None;
        let mut end = None;
        let mut elevations = vec![vec![0u8; cols]; rows];

        for (y, row) in grid.iter().enumerate() {
            if row.len() != cols {
                return Err("Inconsistent row lengths");
            }
            for (x, &ch) in row.iter().enumerate() {
                match ch {
                    'S' => {
                        start = Some(Pos { x, y });
                        elevations[y][x] = 0;
                    }
                    'E' => {
                        end = Some(Pos { x, y });
                        elevations[y][x] = 25;
                    }
                    'a'..='z' => {
                        elevations[y][x] = (ch as u8) - b'a';
                    }
                    _ => return Err("Invalid character in heightmap"),
                }
            }
        }

        Ok(Self {
            elevations,
            start: start.ok_or("Missing start (S)")?,
            end: end.ok_or("Missing end (E)")?,
            rows,
            cols,
        })
    }
}

impl HeightMap {
    fn neighbors(&self, pos: Pos) -> impl Iterator<Item = Pos> + '_ {
        let x = pos.x as i32;
        let y = pos.y as i32;

        Direction::ALL.into_iter().filter_map(move |dir| {
            let nx = x + dir.dx;
            let ny = y + dir.dy;
            if nx >= 0 && nx < self.cols as i32 && ny >= 0 && ny < self.rows as i32 {
                Some(Pos {
                    x: nx as usize,
                    y: ny as usize,
                })
            } else {
                None
            }
        })
    }

    fn shortest_path<G, M>(&self, start_pos: Pos, goal: G, constraint: M) -> Option<usize>
    where
        G: SearchGoal,
        M: MoveConstraint,
    {
        let mut queue = VecDeque::new();
        queue.push_back((start_pos, 0));

        let mut visited = vec![false; self.rows * self.cols];
        visited[start_pos.y * self.cols + start_pos.x] = true;

        while let Some((pos, dist)) = queue.pop_front() {
            if goal.is_reached(pos, self) {
                return Some(dist);
            }

            let current_elevation = self.elevations[pos.y][pos.x];

            for next in self.neighbors(pos) {
                let idx = next.y * self.cols + next.x;
                if !visited[idx]
                    && constraint.can_move(current_elevation, self.elevations[next.y][next.x])
                {
                    visited[idx] = true;
                    queue.push_back((next, dist + 1));
                }
            }
        }

        None
    }
}

fn part1(input: &str) -> usize {
    let hm: HeightMap = input.parse().expect("Invalid heightmap");
    hm.shortest_path(hm.start, ReachTargetPos(hm.end), ForwardMove)
        .unwrap_or(0)
}

fn part2(input: &str) -> usize {
    let hm: HeightMap = input.parse().expect("Invalid heightmap");
    hm.shortest_path(hm.end, ReachElevation(0), BackwardMove)
        .unwrap_or(0)
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
        assert_eq!(part1(TEST_INPUT_1), 31);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 449);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 29);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 443);
    }
}
