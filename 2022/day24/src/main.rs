const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    r: i32,
    c: i32,
}

impl Point {
    fn new(r: i32, c: i32) -> Self {
        Self { r, c }
    }

    fn neighbors(&self) -> [Point; 5] {
        [
            Point::new(self.r, self.c),     // Wait
            Point::new(self.r - 1, self.c), // Up
            Point::new(self.r + 1, self.c), // Down
            Point::new(self.r, self.c - 1), // Left
            Point::new(self.r, self.c + 1), // Right
        ]
    }
}

struct BlizzardBasin {
    width: usize,
    height: usize,
    cycle: usize,
    /// Precomputed blizzard occupancy: [time % cycle][row][col]
    occupied: Vec<bool>,
}

impl BlizzardBasin {
    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.trim().lines().collect();
        let height = lines.len() - 2;
        let width = lines[0].len() - 2;
        let cycle = lcm(width, height);

        let mut initial_blizzards = Vec::new();
        for (r, line) in lines.iter().enumerate().skip(1).take(height) {
            for (c, char) in line.chars().enumerate().skip(1).take(width) {
                if char != '.' {
                    initial_blizzards.push((r - 1, c - 1, char));
                }
            }
        }

        let mut occupied = vec![false; cycle * height * width];
        for t in 0..cycle {
            for &(r, c, dir) in &initial_blizzards {
                let (nr, nc) = match dir {
                    '>' => (r, (c + t) % width),
                    '<' => (r, (c + width - (t % width)) % width),
                    'v' => ((r + t) % height, c),
                    '^' => ((height + r - (t % height)) % height, c),
                    _ => unreachable!(),
                };
                occupied[t * height * width + nr * width + nc] = true;
            }
        }

        Self {
            width,
            height,
            cycle,
            occupied,
        }
    }

    /// Checks if a blizzard occupies the given coordinate at a specific time.
    fn has_blizzard(&self, p: Point, t: usize) -> bool {
        if p.r < 0 || p.r >= self.height as i32 || p.c < 0 || p.c >= self.width as i32 {
            return false;
        }
        let t_mod = t % self.cycle;
        self.occupied[t_mod * self.height * self.width + p.r as usize * self.width + p.c as usize]
    }

    /// Maps (time % cycle, row, col) to a flat index for the visited set.
    fn visited_index(&self, p: Point, t: usize) -> usize {
        let t_mod = t % self.cycle;
        // row is shifted by 1 to accommodate the start/end points at r=-1 and r=height
        let r_idx = (p.r + 1) as usize;
        t_mod * (self.height + 2) * self.width + r_idx * self.width + p.c as usize
    }

    /// Finds the shortest path from start to end beginning at start_time.
    fn shortest_path(&self, start: Point, end: Point, start_time: usize) -> Option<usize> {
        let mut current_states = vec![start];
        let mut visited = vec![false; self.cycle * (self.height + 2) * self.width];
        visited[self.visited_index(start, start_time)] = true;

        for t in start_time.. {
            let next_t = t + 1;
            let mut next_states = Vec::with_capacity(current_states.len() * 2);

            for p in current_states {
                for next_p in p.neighbors() {
                    if next_p == end {
                        return Some(next_t);
                    }

                    // Valid moves: staying at start/end, or moving into the valley if no blizzard
                    let is_start_or_end = next_p == start;
                    let is_in_valley = next_p.r >= 0
                        && next_p.r < self.height as i32
                        && next_p.c >= 0
                        && next_p.c < self.width as i32;

                    if (is_start_or_end || is_in_valley) && !self.has_blizzard(next_p, next_t) {
                        let v_idx = self.visited_index(next_p, next_t);
                        if !visited[v_idx] {
                            visited[v_idx] = true;
                            next_states.push(next_p);
                        }
                    }
                }
            }
            if next_states.is_empty() {
                return None;
            }
            current_states = next_states;
        }
        None
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

fn part1(input: &str) -> usize {
    let basin = BlizzardBasin::new(input);
    let start = Point::new(-1, 0);
    let end = Point::new(basin.height as i32, (basin.width - 1) as i32);
    basin.shortest_path(start, end, 0).unwrap_or(0)
}

fn part2(input: &str) -> usize {
    let basin = BlizzardBasin::new(input);
    let start = Point::new(-1, 0);
    let end = Point::new(basin.height as i32, (basin.width - 1) as i32);

    let t1 = basin.shortest_path(start, end, 0).expect("No path to goal");
    let t2 = basin
        .shortest_path(end, start, t1)
        .expect("No path back to start");
    basin
        .shortest_path(start, end, t2)
        .expect("No path to goal again")
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
        assert_eq!(part1(TEST_INPUT_1), 18);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 253);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 54);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 794);
    }
}
