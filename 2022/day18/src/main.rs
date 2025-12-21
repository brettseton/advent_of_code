use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

trait Neighborhood {
    fn neighbors(&self, p: Point) -> Box<dyn Iterator<Item = Point>>;
}

#[derive(Default)]
struct Orthogonal;
impl Neighborhood for Orthogonal {
    fn neighbors(&self, p: Point) -> Box<dyn Iterator<Item = Point>> {
        let directions = [
            Point::new(1, 0, 0),
            Point::new(-1, 0, 0),
            Point::new(0, 1, 0),
            Point::new(0, -1, 0),
            Point::new(0, 0, 1),
            Point::new(0, 0, -1),
        ];
        Box::new(
            directions
                .into_iter()
                .map(move |d| Point::new(p.x + d.x, p.y + d.y, p.z + d.z)),
        )
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let mut next_part = || {
            parts
                .next()
                .ok_or_else(|| format!("Invalid point format: {}", s))
        };

        let x = next_part()?
            .parse()
            .map_err(|e| format!("Invalid x: {}", e))?;
        let y = next_part()?
            .parse()
            .map_err(|e| format!("Invalid y: {}", e))?;
        let z = next_part()?
            .parse()
            .map_err(|e| format!("Invalid z: {}", e))?;

        Ok(Point::new(x, y, z))
    }
}

struct BoundingBox {
    min: Point,
    max: Point,
}

impl BoundingBox {
    fn contains(&self, p: Point) -> bool {
        p.x >= self.min.x
            && p.x <= self.max.x
            && p.y >= self.min.y
            && p.y <= self.max.y
            && p.z >= self.min.z
            && p.z <= self.max.z
    }
}

struct Droplet<N: Neighborhood> {
    cubes: HashSet<Point>,
    neighborhood: N,
}

impl<N: Neighborhood + Default> FromStr for Droplet<N> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cubes = s
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(str::parse)
            .collect::<Result<HashSet<Point>, _>>()?;
        Ok(Self {
            cubes,
            neighborhood: N::default(),
        })
    }
}

impl<N: Neighborhood> Droplet<N> {
    fn total_surface_area(&self) -> usize {
        self.count_faces(|n| !self.cubes.contains(&n))
    }

    fn exterior_surface_area(&self) -> usize {
        let bounds = match self.get_flood_bounds() {
            Some(b) => b,
            None => return 0,
        };

        let mut outside = HashSet::new();
        let mut queue = VecDeque::new();

        outside.insert(bounds.min);
        queue.push_back(bounds.min);

        while let Some(curr) = queue.pop_front() {
            for next in self.neighborhood.neighbors(curr) {
                if bounds.contains(next) && !self.cubes.contains(&next) && outside.insert(next) {
                    queue.push_back(next);
                }
            }
        }

        self.count_faces(|n| outside.contains(&n))
    }

    fn count_faces<F>(&self, predicate: F) -> usize
    where
        F: Fn(Point) -> bool,
    {
        self.cubes
            .iter()
            .flat_map(|c| self.neighborhood.neighbors(*c))
            .filter(|&n| predicate(n))
            .count()
    }

    fn get_flood_bounds(&self) -> Option<BoundingBox> {
        if self.cubes.is_empty() {
            return None;
        }

        let min_x = self.cubes.iter().map(|c| c.x).min()?;
        let max_x = self.cubes.iter().map(|c| c.x).max()?;
        let min_y = self.cubes.iter().map(|c| c.y).min()?;
        let max_y = self.cubes.iter().map(|c| c.y).max()?;
        let min_z = self.cubes.iter().map(|c| c.z).min()?;
        let max_z = self.cubes.iter().map(|c| c.z).max()?;

        Some(BoundingBox {
            min: Point::new(min_x - 1, min_y - 1, min_z - 1),
            max: Point::new(max_x + 1, max_y + 1, max_z + 1),
        })
    }
}

fn part1(input: &str) -> usize {
    input
        .parse::<Droplet<Orthogonal>>()
        .map(|d| d.total_surface_area())
        .unwrap_or(0)
}

fn part2(input: &str) -> usize {
    input
        .parse::<Droplet<Orthogonal>>()
        .map(|d| d.exterior_surface_area())
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
        assert_eq!(part1(TEST_INPUT_1), 64);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 4244);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 58);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 2460);
    }
}
