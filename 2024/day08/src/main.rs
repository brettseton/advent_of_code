use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<char>>,
    antennas: HashMap<char, Vec<Point>>,
    width: i32,
    height: i32,
}

impl Grid {
    fn from_str(input: &str) -> Self {
        let cells: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = cells.len() as i32;
        let width = cells[0].len() as i32;
        let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();

        for (y, row) in cells.iter().enumerate() {
            for (x, &ch) in row.iter().enumerate() {
                if ch != '.' {
                    antennas
                        .entry(ch)
                        .or_default()
                        .push(Point::new(x as i32, y as i32));
                }
            }
        }

        Grid {
            cells,
            antennas,
            width,
            height,
        }
    }

    fn calculate_antinodes(&self, num_steps: i32) -> HashSet<Point> {
        let mut antinodes = HashSet::new();

        // For each frequency
        for (_freq, positions) in &self.antennas {
            // For each pair of antennas with the same frequency
            for i in 0..positions.len() - 1 {
                for j in (i + 1)..positions.len() {
                    let a1 = &positions[i];
                    let a2 = &positions[j];

                    // Calculate distance between antennas
                    let dx = a2.x - a1.x;
                    let dy = a2.y - a1.y;

                    if dx >= 0 && dy >= 0 {
                        let points1 = self.generate_points_in_direction(a1, -dx, -dy, num_steps);
                        antinodes.extend(points1);

                        let points2 = self.generate_points_in_direction(a2, dx, dy, num_steps);
                        antinodes.extend(points2);
                    } else if dx >= 0 && dy < 0 {
                        panic!("dx >= 0 && dy < 0");
                    } else if dx < 0 && dy >= 0 {
                        let points1 = self.generate_points_in_direction(a1, -dx, -dy, num_steps);
                        antinodes.extend(points1);

                        let points2 = self.generate_points_in_direction(a2, dx, dy, num_steps);
                        antinodes.extend(points2);
                    } else if dx < 0 && dy < 0 {
                        panic!("dx >= 0 && dy < 0");
                    }
                }
            }
        }

        antinodes
    }

    fn generate_points_in_direction(
        &self,
        start: &Point,
        dx: i32,
        dy: i32,
        num_steps: i32,
    ) -> Vec<Point> {
        let mut points = Vec::new();
        let mut step = 1;

        while start.x + dx * step >= 0
            && start.x + dx * step < self.width
            && start.y + dy * step >= 0
            && start.y + dy * step < self.height
            && step <= num_steps
        {
            points.push(Point::new(start.x + dx * step, start.y + dy * step));
            step += 1;
        }

        points
    }
}

fn part1(input: &str) -> i32 {
    let grid = Grid::from_str(input);
    let antinodes = grid.calculate_antinodes(1);
    antinodes.len() as i32
}

fn part2(input: &str) -> usize {
    let grid = Grid::from_str(input);
    let antinodes = grid.calculate_antinodes(100);

    let antennas: HashSet<Point> = grid
        .antennas
        .values()
        .flat_map(|v| v.iter().cloned())
        .collect();

    antinodes.len() + antennas.difference(&antinodes).count()
}

fn main() {
    let input1 =
        fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
    let input2 =
        fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");

    println!("Part 1 test 1: {}", part1(&input1));
    println!("Part 1 test 2: {}", part1(&input2));

    println!("Part 2 test 1: {}", part2(&input1));
    println!("Part 2 test 2: {}", part2(&input2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 14);
    }

    #[test]
    fn test2_part1() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 313);
    }

    #[test]
    fn test1_part2() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 34);
    }

    #[test]
    fn test2_part2() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 1064);
    }
}
