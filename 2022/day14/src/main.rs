use std::collections::HashSet;
use std::str::FromStr;

const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn line_to(self, other: Self) -> impl Iterator<Item = Self> {
        let min_x = self.x.min(other.x);
        let max_x = self.x.max(other.x);
        let min_y = self.y.min(other.y);
        let max_y = self.y.max(other.y);

        (min_x..=max_x).flat_map(move |x| (min_y..=max_y).map(move |y| Point::new(x, y)))
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts
            .next()
            .ok_or("Missing X")?
            .parse()
            .map_err(|e| format!("Invalid X: {e}"))?;
        let y = parts
            .next()
            .ok_or("Missing Y")?
            .parse()
            .map_err(|e| format!("Invalid Y: {e}"))?;
        Ok(Point::new(x, y))
    }
}

struct Cave {
    occupied: HashSet<Point>,
    max_y: i32,
    sand_source: Option<Point>,
    floor: Option<i32>,
    path: Vec<Point>,
}

impl Cave {
    fn new(input: &str) -> Self {
        let mut occupied = HashSet::new();
        let mut max_y = 0;

        for line in input.lines() {
            let points: Vec<Point> = line
                .split(" -> ")
                .map(|p| p.parse().expect("Failed to parse point"))
                .collect();

            for window in points.windows(2) {
                for p in window[0].line_to(window[1]) {
                    max_y = max_y.max(p.y);
                    occupied.insert(p);
                }
            }
        }

        Self {
            occupied,
            max_y,
            sand_source: None,
            floor: None,
            path: Vec::new(),
        }
    }

    fn with_source(mut self, source: Point) -> Self {
        self.sand_source = Some(source);
        self
    }

    fn with_floor(mut self, offset: i32) -> Self {
        self.floor = Some(self.max_y + offset);
        self
    }

    #[inline]
    fn is_blocked(&self, x: i32, y: i32) -> bool {
        self.occupied.contains(&Point::new(x, y)) || self.floor == Some(y)
    }

    fn drop_sand(&mut self) -> Option<Point> {
        let source = self.sand_source.expect("Sand source must be configured");

        if self.path.is_empty() {
            if self.is_blocked(source.x, source.y) {
                return None;
            }
            self.path.push(source);
        }

        while let Some(&curr) = self.path.last() {
            if self.floor.is_none() && curr.y >= self.max_y {
                return None;
            }

            let next_y = curr.y + 1;
            let next_x = if !self.is_blocked(curr.x, next_y) {
                Some(curr.x)
            } else if !self.is_blocked(curr.x - 1, next_y) {
                Some(curr.x - 1)
            } else if !self.is_blocked(curr.x + 1, next_y) {
                Some(curr.x + 1)
            } else {
                None
            };

            match next_x {
                Some(x) => {
                    self.path.push(Point::new(x, next_y));
                }
                None => {
                    let settled = self.path.pop().unwrap();
                    self.occupied.insert(settled);
                    return Some(settled);
                }
            }
        }
        None
    }
}

fn part1(input: &str) -> i32 {
    let mut cave = Cave::new(input).with_source(Point::new(500, 0));
    let mut count = 0;
    while cave.drop_sand().is_some() {
        count += 1;
    }
    count
}

fn part2(input: &str) -> i32 {
    let mut cave = Cave::new(input)
        .with_source(Point::new(500, 0))
        .with_floor(2);
    let mut count = 0;
    while cave.drop_sand().is_some() {
        count += 1;
    }
    count
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
        assert_eq!(part1(TEST_INPUT_1), 24);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 578);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 93);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 24377);
    }
}
