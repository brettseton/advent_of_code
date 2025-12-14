const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

const SHAPE_WIDTH: usize = 3;
const SHAPE_HEIGHT: usize = 3;
const MAX_PRESENTS_TOLERANCE: usize = 2;

struct Shape {
    tile_count: usize,
}

struct Region {
    width: usize,
    height: usize,
    shape_quantities: Vec<usize>,
}

struct PuzzleInput {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

impl Shape {
    fn parse_from_lines(lines: &[&str], index: &mut usize) -> Option<Self> {
        let mut grid = Vec::new();
        *index += 1;

        while *index < lines.len() {
            let shape_line = lines[*index].trim();
            if shape_line.is_empty() || shape_line.ends_with(':') || shape_line.contains('x') {
                break;
            }
            grid.push(shape_line);
            *index += 1;
        }

        if grid.len() != SHAPE_HEIGHT || !grid.iter().all(|row| row.len() == SHAPE_WIDTH) {
            return None;
        }

        let tile_count = grid
            .iter()
            .flat_map(|row| row.chars())
            .filter(|&ch| ch == '#')
            .count();

        Some(Shape { tile_count })
    }
}

impl Region {
    fn parse_from_line(line: &str) -> Option<Self> {
        if !line.contains('x') {
            return None;
        }

        let numbers: Vec<usize> = line
            .split(|c: char| !c.is_ascii_digit())
            .filter_map(|s| s.parse().ok())
            .collect();

        if numbers.len() < 2 {
            return None;
        }

        Some(Region {
            width: numbers[0],
            height: numbers[1],
            shape_quantities: numbers[2..].to_vec(),
        })
    }

    fn total_presents(&self) -> usize {
        self.shape_quantities.iter().sum()
    }

    fn area(&self) -> usize {
        self.width * self.height
    }

    fn max_presents_lower_bound(&self) -> usize {
        (self.width / SHAPE_WIDTH) * (self.height / SHAPE_HEIGHT)
    }
}

struct PuzzleParser;

impl PuzzleParser {
    fn parse(input: &str) -> PuzzleInput {
        let lines: Vec<&str> = input.lines().collect();
        let shapes = Self::parse_shapes(&lines);
        let regions = Self::parse_regions(&lines);

        PuzzleInput { shapes, regions }
    }

    fn parse_shapes(lines: &[&str]) -> Vec<Shape> {
        let mut shapes = Vec::new();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();
            if line.is_empty() {
                i += 1;
                continue;
            }

            if line.contains('x') {
                break;
            }

            if line.ends_with(':') {
                if let Some(shape) = Shape::parse_from_lines(lines, &mut i) {
                    shapes.push(shape);
                }
            } else {
                i += 1;
            }
        }

        shapes
    }

    fn parse_regions(lines: &[&str]) -> Vec<Region> {
        lines
            .iter()
            .filter_map(|line| Region::parse_from_line(line.trim()))
            .collect()
    }
}

struct RegionValidator;

impl RegionValidator {
    fn can_fit(region: &Region, shapes: &[Shape]) -> bool {
        if Self::fits_by_presents_bound(region) {
            return true;
        }

        if !Self::fits_by_tile_count(region, shapes) {
            return false;
        }

        Self::fits_by_presents_tolerance(region)
    }

    fn fits_by_presents_bound(region: &Region) -> bool {
        region.total_presents() <= region.max_presents_lower_bound()
    }

    fn fits_by_tile_count(region: &Region, shapes: &[Shape]) -> bool {
        let total_tiles: usize = shapes
            .iter()
            .zip(region.shape_quantities.iter())
            .map(|(shape, &quantity)| shape.tile_count * quantity)
            .sum();

        total_tiles <= region.area()
    }

    fn fits_by_presents_tolerance(region: &Region) -> bool {
        region.total_presents() <= region.max_presents_lower_bound() + MAX_PRESENTS_TOLERANCE
    }
}

fn part1(input: &str) -> i32 {
    let puzzle = PuzzleParser::parse(input);

    puzzle
        .regions
        .iter()
        .filter(|region| RegionValidator::can_fit(region, &puzzle.shapes))
        .count() as i32
}

fn main() {
    println!("Part 1 test 1: {}", part1(TEST_INPUT_1));
    println!("Part 1 test 2: {}", part1(TEST_INPUT_2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() {
        assert_eq!(part1(TEST_INPUT_1), 2);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 599);
    }
}
