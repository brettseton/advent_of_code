const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

const SHAPE_WIDTH: usize = 3;
const SHAPE_HEIGHT: usize = 3;
const MAX_PRESENTS_TOLERANCE: usize = 2;
const REGION_DELIMITER: char = 'x';
const SHAPE_HEADER_SUFFIX: char = ':';
const TILE_CHAR: char = '#';

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

impl PuzzleInput {
    fn count_fitting_regions(&self) -> usize {
        self.regions
            .iter()
            .filter(|region| RegionValidator::can_fit(region, &self.shapes))
            .count()
    }
}

impl Shape {
    fn parse_from_lines(lines: &[&str], index: &mut usize) -> Option<Self> {
        let grid: Vec<&str> = lines
            .iter()
            .skip(*index + 1)
            .map(|s| s.trim())
            .take_while(|line| {
                !line.is_empty()
                    && !line.ends_with(SHAPE_HEADER_SUFFIX)
                    && !line.contains(REGION_DELIMITER)
            })
            .collect();

        if grid.len() != SHAPE_HEIGHT || !grid.iter().all(|row| row.len() == SHAPE_WIDTH) {
            *index += grid.len() + 1;
            return None;
        }

        *index += grid.len() + 1;

        let tile_count = grid
            .iter()
            .flat_map(|row| row.chars())
            .filter(|&ch| ch == TILE_CHAR)
            .count();

        Some(Shape { tile_count })
    }
}

impl Region {
    fn parse_from_line(line: &str) -> Option<Self> {
        if !line.contains(REGION_DELIMITER) {
            return None;
        }

        let numbers = Self::extract_numbers(line);
        if numbers.len() < 2 {
            return None;
        }

        Some(Region {
            width: numbers[0],
            height: numbers[1],
            shape_quantities: numbers[2..].to_vec(),
        })
    }

    fn extract_numbers(line: &str) -> Vec<usize> {
        line.split(|c: char| !c.is_ascii_digit())
            .filter_map(|s| s.parse().ok())
            .collect()
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

    fn total_tile_count(&self, shapes: &[Shape]) -> usize {
        shapes
            .iter()
            .zip(self.shape_quantities.iter())
            .map(|(shape, &quantity)| shape.tile_count * quantity)
            .sum()
    }
}

struct ShapeParser;

impl ShapeParser {
    fn parse_all(lines: &[&str]) -> Vec<Shape> {
        let mut shapes = Vec::new();
        let mut index = 0;

        while index < lines.len() {
            let line = lines[index].trim();

            if line.is_empty() {
                index += 1;
                continue;
            }

            if line.contains(REGION_DELIMITER) {
                break;
            }

            if line.ends_with(SHAPE_HEADER_SUFFIX) {
                if let Some(shape) = Shape::parse_from_lines(lines, &mut index) {
                    shapes.push(shape);
                }
            } else {
                index += 1;
            }
        }

        shapes
    }
}

struct RegionParser;

impl RegionParser {
    fn parse_all(lines: &[&str]) -> Vec<Region> {
        lines
            .iter()
            .filter_map(|line| Region::parse_from_line(line.trim()))
            .collect()
    }
}

struct PuzzleParser;

impl PuzzleParser {
    fn parse(input: &str) -> PuzzleInput {
        let lines: Vec<&str> = input.lines().collect();
        let shapes = ShapeParser::parse_all(&lines);
        let regions = RegionParser::parse_all(&lines);

        PuzzleInput { shapes, regions }
    }
}

struct RegionValidator;

impl RegionValidator {
    fn can_fit(region: &Region, shapes: &[Shape]) -> bool {
        Self::check_all_conditions(region, shapes)
    }

    fn check_all_conditions(region: &Region, shapes: &[Shape]) -> bool {
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
        region.total_tile_count(shapes) <= region.area()
    }

    fn fits_by_presents_tolerance(region: &Region) -> bool {
        region.total_presents() <= region.max_presents_lower_bound() + MAX_PRESENTS_TOLERANCE
    }
}

fn part1(input: &str) -> i32 {
    PuzzleParser::parse(input).count_fitting_regions() as i32
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
