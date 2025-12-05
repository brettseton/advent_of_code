use std::fmt;
use std::ops::Index;

const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Cell {
    PaperRoll,
    #[default]
    Empty,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '@' => Cell::PaperRoll,
            _ => Cell::Empty,
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::PaperRoll => write!(f, "@"),
            Cell::Empty => write!(f, "."),
        }
    }
}

impl Cell {
    const fn is_paper_roll(self) -> bool {
        matches!(self, Cell::PaperRoll)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    row: isize,
    col: isize,
}

impl Coord {
    const fn new(row: usize, col: usize) -> Self {
        Self {
            row: row as isize,
            col: col as isize,
        }
    }

    fn neighbors(self) -> impl Iterator<Item = Coord> {
        const DELTAS: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        DELTAS.into_iter().map(move |(dr, dc)| Coord {
            row: self.row + dr,
            col: self.col + dc,
        })
    }
}

trait StabilityRule {
    fn is_stable(&self, grid: &Grid, coord: Coord) -> bool;
}

#[derive(Debug)]
struct NeighborCountRule {
    threshold: usize,
}

impl Default for NeighborCountRule {
    fn default() -> Self {
        Self { threshold: 4 }
    }
}

impl StabilityRule for NeighborCountRule {
    fn is_stable(&self, grid: &Grid, coord: Coord) -> bool {
        match grid.get(coord) {
            Some(cell) if cell.is_paper_roll() => {
                grid.count_paper_roll_neighbors(coord) >= self.threshold
            }
            _ => true,
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let cells: Vec<Vec<Cell>> = input
            .lines()
            .map(|line| line.chars().map(Cell::from).collect())
            .collect();
        Self { cells }
    }

    fn rows(&self) -> usize {
        self.cells.len()
    }

    fn cols(&self) -> usize {
        self.cells.first().map_or(0, Vec::len)
    }

    fn contains(&self, coord: Coord) -> bool {
        coord.row >= 0
            && coord.row < self.rows() as isize
            && coord.col >= 0
            && coord.col < self.cols() as isize
    }

    fn get(&self, coord: Coord) -> Option<Cell> {
        self.contains(coord)
            .then(|| self.cells[coord.row as usize][coord.col as usize])
    }

    fn set(&mut self, coord: Coord, value: Cell) {
        debug_assert!(self.contains(coord), "Coord out of bounds: {:?}", coord);
        if self.contains(coord) {
            self.cells[coord.row as usize][coord.col as usize] = value;
        }
    }

    fn count_paper_roll_neighbors(&self, coord: Coord) -> usize {
        coord
            .neighbors()
            .filter_map(|n| self.get(n))
            .filter(|cell| cell.is_paper_roll())
            .count()
    }
}

impl Index<Coord> for Grid {
    type Output = Cell;

    fn index(&self, coord: Coord) -> &Self::Output {
        &self.cells[coord.row as usize][coord.col as usize]
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.cells {
            for cell in row {
                write!(f, "{cell}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct GridIter<'a> {
    grid: &'a Grid,
    row: usize,
    col: usize,
}

impl<'a> Iterator for GridIter<'a> {
    type Item = (Coord, Cell);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.grid.rows() {
            return None;
        }
        let coord = Coord::new(self.row, self.col);
        let cell = self.grid[coord];
        self.col += 1;
        if self.col >= self.grid.cols() {
            self.col = 0;
            self.row += 1;
        }
        Some((coord, cell))
    }
}

impl<'a> IntoIterator for &'a Grid {
    type Item = (Coord, Cell);
    type IntoIter = GridIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        GridIter {
            grid: self,
            row: 0,
            col: 0,
        }
    }
}

struct Simulator<R: StabilityRule> {
    grid: Grid,
    stability_rule: R,
}

impl<R: StabilityRule> Simulator<R> {
    fn new(grid: Grid, stability_rule: R) -> Self {
        Self {
            grid,
            stability_rule,
        }
    }

    fn find_unstable(&self) -> Vec<Coord> {
        self.grid
            .into_iter()
            .filter(|(coord, cell)| {
                cell.is_paper_roll() && !self.stability_rule.is_stable(&self.grid, *coord)
            })
            .map(|(coord, _)| coord)
            .collect()
    }

    fn count_unstable(&self) -> usize {
        self.find_unstable().len()
    }

    fn remove_unstable_cells(&mut self) -> usize {
        let unstable = self.find_unstable();
        let count = unstable.len();
        for coord in unstable {
            self.grid.set(coord, Cell::Empty);
        }
        count
    }

    fn count_removable_cells(&mut self) -> usize {
        std::iter::from_fn(|| {
            let removed = self.remove_unstable_cells();
            (removed > 0).then_some(removed)
        })
        .sum()
    }
}

fn part1(input: &str) -> usize {
    let simulator = Simulator::new(Grid::new(input), NeighborCountRule::default());
    simulator.count_unstable()
}

fn part2(input: &str) -> usize {
    let mut simulator = Simulator::new(Grid::new(input), NeighborCountRule::default());
    simulator.count_removable_cells()
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
        assert_eq!(part1(TEST_INPUT_1), 13);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 1449);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 43);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 8746);
    }
}
