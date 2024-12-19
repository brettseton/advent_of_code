use std::collections::HashSet;
use std::fs;

type Grid = Vec<Vec<char>>;
type Region = HashSet<Point>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn neighbor(&self, offset: Point) -> Self {
        Self::new(self.x + offset.x, self.y + offset.y)
    }

    fn in_bounds(&self, rows: i32, cols: i32) -> bool {
        self.y >= 0 && self.y < rows && self.x >= 0 && self.x < cols
    }
}

fn parse_input(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_neighbors(point: Point, grid: &Grid) -> Vec<Point> {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    Direction::all_offsets()
        .map(|offset| point.neighbor(offset))
        .filter(|p| p.in_bounds(rows, cols))
        .collect()
}

fn find_region(start: Point, grid: &Grid) -> Region {
    let mut region = Region::new();
    let mut stack = vec![start];
    let target_char = grid[start.y as usize][start.x as usize];

    while let Some(point) = stack.pop() {
        if !region.insert(point) {
            continue;
        }

        stack.extend(get_neighbors(point, grid).into_iter().filter(|&neighbor| {
            grid[neighbor.y as usize][neighbor.x as usize] == target_char
                && !region.contains(&neighbor)
        }));
    }

    region
}

fn calculate_perimeter(region: &Region) -> i32 {
    let mut perimeter = 0;

    for &point in region.iter() {
        for offset in Direction::all_offsets() {
            let neighbor = point.neighbor(offset);

            if !region.contains(&neighbor) {
                perimeter += 1;
            }
        }
    }

    perimeter
}

fn count_sides(region: &Region) -> i32 {
    let mut continuous_sides = 0;

    // Check horizontal sides
    for &point in region.iter() {
        // Check if this point has no region point above it
        let above = Point {
            x: point.x,
            y: point.y - 1,
        };
        if !region.contains(&above) {
            // Check if this is the start of a new horizontal side
            let left = Point {
                x: point.x - 1,
                y: point.y,
            };
            if !region.contains(&left) || !is_exposed_edge(left, Direction::Top, region) {
                continuous_sides += 1;
            }
        }

        // Check if this point has no region point below it
        let below = Point {
            x: point.x,
            y: point.y + 1,
        };
        if !region.contains(&below) {
            // Check if this is the start of a new horizontal side
            let left = Point {
                x: point.x - 1,
                y: point.y,
            };
            if !region.contains(&left) || !is_exposed_edge(left, Direction::Bottom, region) {
                continuous_sides += 1;
            }
        }
    }

    // Check vertical sides
    for &point in region.iter() {
        // Check if this point has no region point to the left
        let left = Point {
            x: point.x - 1,
            y: point.y,
        };
        if !region.contains(&left) {
            // Check if this is the start of a new vertical side
            let above = Point {
                x: point.x,
                y: point.y - 1,
            };
            if !region.contains(&above) || !is_exposed_edge(above, Direction::Left, region) {
                continuous_sides += 1;
            }
        }

        // Check if this point has no region point to the right
        let right = Point {
            x: point.x + 1,
            y: point.y,
        };
        if !region.contains(&right) {
            // Check if this is the start of a new vertical side
            let above = Point {
                x: point.x,
                y: point.y - 1,
            };
            if !region.contains(&above) || !is_exposed_edge(above, Direction::Right, region) {
                continuous_sides += 1;
            }
        }
    }

    continuous_sides
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

impl Direction {
    fn offset(&self) -> Point {
        match self {
            Direction::Top => Point::new(0, -1),
            Direction::Bottom => Point::new(0, 1),
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0),
        }
    }

    fn neighbor(&self, point: Point) -> Point {
        point.neighbor(self.offset())
    }

    fn all_offsets() -> impl Iterator<Item = Point> {
        [
            Direction::Top,
            Direction::Bottom,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        .map(|dir| dir.offset())
    }
}

fn is_exposed_edge(point: Point, direction: Direction, region: &Region) -> bool {
    if !region.contains(&point) {
        return false;
    }

    !region.contains(&direction.neighbor(point))
}

fn get_regions(grid: &Grid) -> Vec<Region> {
    let mut visited = Region::new();
    let mut regions = Vec::new();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let point = Point::new(col as i32, row as i32);

            if !visited.contains(&point) {
                let region = find_region(point, grid);
                visited.extend(&region);
                regions.push(region);
            }
        }
    }

    regions
}

fn part1(input: &str) -> i32 {
    let grid = parse_input(input);
    get_regions(&grid)
        .iter()
        .map(|region| {
            let area = region.len() as i32;
            let perimeter = calculate_perimeter(region);
            area * perimeter
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    let grid = parse_input(input);
    get_regions(&grid)
        .iter()
        .map(|region| {
            let area = region.len() as i32;
            let sides = count_sides(region);
            area * sides
        })
        .sum()
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
        assert_eq!(part1(&test_input), 1930);
    }

    #[test]
    fn test2_part1() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 1573474);
    }

    #[test]
    fn test1_part2() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 1206);
    }

    #[test]
    fn test2_part2() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 966476);
    }
}
