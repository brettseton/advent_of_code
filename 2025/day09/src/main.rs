use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn rectangle_area(self, other: Self) -> i128 {
        let width = i128::from((self.x - other.x).abs()) + 1;
        let height = i128::from((self.y - other.y).abs()) + 1;
        width * height
    }
}

impl FromStr for Point {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = s.trim().split_once(',').ok_or("expected format 'x,y'")?;
        let x = x_str
            .trim()
            .parse()
            .map_err(|_| "failed to parse x coordinate")?;
        let y = y_str
            .trim()
            .parse()
            .map_err(|_| "failed to parse y coordinate")?;
        Ok(Point::new(x, y))
    }
}

fn parse_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter_map(|line| line.trim().parse::<Point>().ok())
        .collect()
}

trait PointsExt {
    fn max_rectangle_area(&self) -> i64;
}

impl PointsExt for [Point] {
    fn max_rectangle_area(&self) -> i64 {
        (0..self.len())
            .flat_map(|i| (i + 1..self.len()).map(move |j| (self[i], self[j])))
            .map(|(p1, p2)| p1.rectangle_area(p2))
            .max()
            .unwrap_or(0)
            .try_into()
            .expect("area should fit in i64")
    }
}

trait BoundaryTracer {
    fn trace_boundary(
        &self,
        points: &[Point],
        mapper: &CoordinateMapper,
    ) -> Result<Vec<Vec<bool>>, &'static str>;
}

trait FloodFiller {
    fn flood_fill_outside(
        &self,
        boundary: &[Vec<bool>],
        mapper: &CoordinateMapper,
    ) -> Vec<Vec<bool>>;
}

trait AreaCalculator {
    fn cell_area(&self, mapper: &CoordinateMapper, x: usize, y: usize) -> i128;

    fn rectangle_area(
        &self,
        mapper: &CoordinateMapper,
        x1: usize,
        x2: usize,
        y1: usize,
        y2: usize,
    ) -> i128;

    fn build_prefix_sums(&self, mapper: &CoordinateMapper, green: &[Vec<bool>]) -> Vec<Vec<i128>>;

    fn rectangle_sum(prefix_sums: &[Vec<i128>], x1: usize, x2: usize, y1: usize, y2: usize)
        -> i128;
}

struct DefaultAreaCalculator;

impl AreaCalculator for DefaultAreaCalculator {
    fn cell_area(&self, mapper: &CoordinateMapper, x: usize, y: usize) -> i128 {
        i128::from(mapper.xs[x + 1] - mapper.xs[x]) * i128::from(mapper.ys[y + 1] - mapper.ys[y])
    }

    fn rectangle_area(
        &self,
        mapper: &CoordinateMapper,
        x1: usize,
        x2: usize,
        y1: usize,
        y2: usize,
    ) -> i128 {
        let (x_min, x_max) = (x1.min(x2), x1.max(x2));
        let (y_min, y_max) = (y1.min(y2), y1.max(y2));
        i128::from(mapper.xs[x_max + 1] - mapper.xs[x_min])
            * i128::from(mapper.ys[y_max + 1] - mapper.ys[y_min])
    }

    fn build_prefix_sums(&self, mapper: &CoordinateMapper, green: &[Vec<bool>]) -> Vec<Vec<i128>> {
        let mut prefix_sums = vec![vec![0i128; mapper.width + 1]; mapper.height + 1];

        for y in 0..mapper.height {
            for x in 0..mapper.width {
                let cell_area = green[y][x]
                    .then(|| self.cell_area(mapper, x, y))
                    .unwrap_or(0);
                prefix_sums[y + 1][x + 1] =
                    prefix_sums[y][x + 1] + prefix_sums[y + 1][x] - prefix_sums[y][x] + cell_area;
            }
        }

        prefix_sums
    }

    fn rectangle_sum(
        prefix_sums: &[Vec<i128>],
        x1: usize,
        x2: usize,
        y1: usize,
        y2: usize,
    ) -> i128 {
        let (x_min, x_max) = (x1.min(x2), x1.max(x2));
        let (y_min, y_max) = (y1.min(y2), y1.max(y2));
        prefix_sums[y_max + 1][x_max + 1]
            - prefix_sums[y_min][x_max + 1]
            - prefix_sums[y_max + 1][x_min]
            + prefix_sums[y_min][x_min]
    }
}

trait RectangleValidator<V: AreaCalculator> {
    fn max_rectangle_area_in_green(
        &self,
        corner_cells: &[(usize, usize)],
        prefix_sums: &[Vec<i128>],
        area_calculator: &V,
        mapper: &CoordinateMapper,
    ) -> i128;
}

struct DefaultRectangleValidator;

impl<V: AreaCalculator> RectangleValidator<V> for DefaultRectangleValidator {
    fn max_rectangle_area_in_green(
        &self,
        corner_cells: &[(usize, usize)],
        prefix_sums: &[Vec<i128>],
        area_calculator: &V,
        mapper: &CoordinateMapper,
    ) -> i128 {
        let mut max_area = 0i128;
        for i in 0..corner_cells.len() {
            for j in (i + 1)..corner_cells.len() {
                let (x1, y1) = corner_cells[i];
                let (x2, y2) = corner_cells[j];
                let green_area = V::rectangle_sum(prefix_sums, x1, x2, y1, y2);
                let full_area = area_calculator.rectangle_area(mapper, x1, x2, y1, y2);
                if green_area == full_area {
                    max_area = max_area.max(green_area);
                }
            }
        }
        max_area
    }
}

struct DefaultFloodFiller;

impl FloodFiller for DefaultFloodFiller {
    fn flood_fill_outside(
        &self,
        boundary: &[Vec<bool>],
        mapper: &CoordinateMapper,
    ) -> Vec<Vec<bool>> {
        let mut outside = vec![vec![false; mapper.width]; mapper.height];
        let mut queue = VecDeque::from([(0, 0)]);
        outside[0][0] = true;

        while let Some((y, x)) = queue.pop_front() {
            for &(dy, dx) in &DIRECTIONS {
                let ny = y as isize + dy;
                let nx = x as isize + dx;
                if CoordinateMapper::in_bounds(ny, nx, mapper.height, mapper.width) {
                    let (nyu, nxu) = (ny as usize, nx as usize);
                    if !boundary[nyu][nxu] && !outside[nyu][nxu] {
                        outside[nyu][nxu] = true;
                        queue.push_back((nyu, nxu));
                    }
                }
            }
        }

        outside
    }
}

struct DefaultBoundaryTracer;

impl BoundaryTracer for DefaultBoundaryTracer {
    fn trace_boundary(
        &self,
        points: &[Point],
        mapper: &CoordinateMapper,
    ) -> Result<Vec<Vec<bool>>, &'static str> {
        let mut boundary = vec![vec![false; mapper.width]; mapper.height];

        for i in 0..points.len() {
            let p1 = points[i];
            let p2 = points[(i + 1) % points.len()];

            if p1.x == p2.x {
                let ix = mapper.get_x_index(p1.x).ok_or("invalid x coordinate")?;
                let y_start = mapper
                    .get_y_index(p1.y.min(p2.y))
                    .ok_or("invalid y coordinate")?;
                let y_end = mapper
                    .get_y_index(p1.y.max(p2.y))
                    .ok_or("invalid y coordinate")?;
                for row in boundary[y_start..=y_end].iter_mut() {
                    row[ix] = true;
                }
            } else if p1.y == p2.y {
                let iy = mapper.get_y_index(p1.y).ok_or("invalid y coordinate")?;
                let x_start = mapper
                    .get_x_index(p1.x.min(p2.x))
                    .ok_or("invalid x coordinate")?;
                let x_end = mapper
                    .get_x_index(p1.x.max(p2.x))
                    .ok_or("invalid x coordinate")?;
                for cell in boundary[iy][x_start..=x_end].iter_mut() {
                    *cell = true;
                }
            } else {
                return Err("segment must be axis-aligned (horizontal or vertical)");
            }
        }

        Ok(boundary)
    }
}

#[derive(Clone, Debug)]
struct CoordinateMapper {
    xs: Vec<i64>,
    ys: Vec<i64>,
    x_index: HashMap<i64, usize>,
    y_index: HashMap<i64, usize>,
    width: usize,
    height: usize,
}

impl CoordinateMapper {
    fn from_points(points: &[Point]) -> Self {
        let ((min_x, max_x), (min_y, max_y)) = points.iter().fold(
            ((i64::MAX, i64::MIN), (i64::MAX, i64::MIN)),
            |((min_x, max_x), (min_y, max_y)), p| {
                (
                    (min_x.min(p.x), max_x.max(p.x)),
                    (min_y.min(p.y), max_y.max(p.y)),
                )
            },
        );

        let mut xs_set: HashSet<i64> = points.iter().flat_map(|p| [p.x, p.x + 1]).collect();
        let mut ys_set: HashSet<i64> = points.iter().flat_map(|p| [p.y, p.y + 1]).collect();

        xs_set.insert(min_x.saturating_sub(1));
        xs_set.insert(max_x.saturating_add(2));
        ys_set.insert(min_y.saturating_sub(1));
        ys_set.insert(max_y.saturating_add(2));

        let mut xs: Vec<i64> = xs_set.into_iter().collect();
        let mut ys: Vec<i64> = ys_set.into_iter().collect();
        xs.sort_unstable();
        ys.sort_unstable();

        let x_index: HashMap<i64, usize> = xs.iter().enumerate().map(|(i, &x)| (x, i)).collect();
        let y_index: HashMap<i64, usize> = ys.iter().enumerate().map(|(i, &y)| (y, i)).collect();

        let width = xs.len() - 1;
        let height = ys.len() - 1;

        Self {
            xs,
            ys,
            x_index,
            y_index,
            width,
            height,
        }
    }

    fn to_grid_coords(&self, p: Point) -> Option<(usize, usize)> {
        Some((*self.x_index.get(&p.x)?, *self.y_index.get(&p.y)?))
    }

    fn get_x_index(&self, x: i64) -> Option<usize> {
        self.x_index.get(&x).copied()
    }

    fn get_y_index(&self, y: i64) -> Option<usize> {
        self.y_index.get(&y).copied()
    }

    fn in_bounds(y: isize, x: isize, height: usize, width: usize) -> bool {
        y >= 0 && x >= 0 && (y as usize) < height && (x as usize) < width
    }
}

struct CompressedGrid<
    T: BoundaryTracer,
    U: FloodFiller,
    V: AreaCalculator,
    W: RectangleValidator<V>,
> {
    points: Vec<Point>,
    mapper: CoordinateMapper,
    boundary_tracer: T,
    flood_filler: U,
    area_calculator: V,
    rectangle_validator: W,
}

impl<T: BoundaryTracer, U: FloodFiller, V: AreaCalculator, W: RectangleValidator<V>>
    CompressedGrid<T, U, V, W>
{
    fn from_points(
        points: &[Point],
        boundary_tracer: T,
        flood_filler: U,
        area_calculator: V,
        rectangle_validator: W,
    ) -> Self {
        Self {
            points: points.to_vec(),
            mapper: CoordinateMapper::from_points(points),
            boundary_tracer,
            flood_filler,
            area_calculator,
            rectangle_validator,
        }
    }

    fn max_valid_rectangle_area(&self) -> i128 {
        let boundary = self
            .boundary_tracer
            .trace_boundary(&self.points, &self.mapper)
            .expect("failed to trace boundary");
        let outside = self
            .flood_filler
            .flood_fill_outside(&boundary, &self.mapper);

        let green: Vec<Vec<bool>> = boundary
            .iter()
            .zip(&outside)
            .map(|(boundary_row, outside_row)| {
                boundary_row
                    .iter()
                    .zip(outside_row.iter())
                    .map(|(&is_boundary, &is_outside)| is_boundary || !is_outside)
                    .collect()
            })
            .collect();

        let prefix_sums = self.area_calculator.build_prefix_sums(&self.mapper, &green);
        let corner_cells: Vec<(usize, usize)> = self
            .points
            .iter()
            .filter_map(|&p| self.mapper.to_grid_coords(p))
            .collect();

        self.rectangle_validator.max_rectangle_area_in_green(
            &corner_cells,
            &prefix_sums,
            &self.area_calculator,
            &self.mapper,
        )
    }
}

fn part1(input: &str) -> i64 {
    let points = parse_points(input);
    points.max_rectangle_area()
}

fn part2(input: &str) -> i64 {
    let points = parse_points(input);

    let grid = CompressedGrid::from_points(
        &points,
        DefaultBoundaryTracer,
        DefaultFloodFiller,
        DefaultAreaCalculator,
        DefaultRectangleValidator,
    );
    grid.max_valid_rectangle_area()
        .try_into()
        .expect("area should fit in i64")
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
        assert_eq!(part1(TEST_INPUT_1), 50);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 4754955192);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 24);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 1568849600);
    }
}
