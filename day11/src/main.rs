use std::str::FromStr;
use std::{fs, usize};

fn main() {
    let ans = part1("C:/git/advent_of_code/day11/input/test1.txt");
    println!("part 1 test 1 answer: {}", ans);

    let ans = part1("C:/git/advent_of_code/day11/input/test2.txt");
    println!("part 1 test 2 answer: {}", ans);

    let ans = part2("C:/git/advent_of_code/day11/input/test1.txt");
    println!("part 2 test 1 answer: {}", ans);

    let ans = part2("C:/git/advent_of_code/day11/input/test2.txt");
    println!("part 2 test 2 answer: {}", ans);
}

fn part1(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("Unable to read the input file");
    let galaxy_map = GalaxyMap::new(&input);
    let distances = galaxy_map.get_galaxy_distances_after_expansion(2);
    return distances.iter().sum();
}

fn part2(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("Unable to read the input file");
    let galaxy_map = GalaxyMap::new(&input);
    let distances = galaxy_map.get_galaxy_distances_after_expansion(1_000_000);
    return distances.iter().sum();
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct GalaxyMap {
    map: Vec<char>,
    galaxies: Vec<Point>,
    map_width: usize,
    map_height: usize,
    expanded_rows: Vec<usize>,
    expanded_cols: Vec<usize>,
}

impl GalaxyMap {
    pub fn new(str: &str) -> GalaxyMap {
        return GalaxyMap::from_str(str).expect("Ctor from string failed");
    }

    pub fn get_galaxy_distances_after_expansion(&self, expansion_size: usize) -> Vec<usize> {
        return self.galaxies.iter().enumerate().flat_map(|(i, galaxy1)| {
            self.galaxies.iter().skip(i + 1).map(move |galaxy2| {
                let max_x = galaxy1.x.max(galaxy2.x);
                let min_x = galaxy1.x.min(galaxy2.x);
                let max_y = galaxy1.y.max(galaxy2.y);
                let min_y = galaxy1.y.min(galaxy2.y);
    
                let expanded_col_count = self.expanded_cols.iter().filter(|&&p| p > min_x && p < max_x).count();
                let expanded_row_count = self.expanded_rows.iter().filter(|&&p| p > min_y && p < max_y).count();
    
                return (max_x - min_x + max_y - min_y) + expanded_col_count * (expansion_size - 1) + expanded_row_count * (expansion_size - 1);
            })
        }).collect();
    }
}

#[derive(Debug)]
struct GalaxyMapParseError;

impl FromStr for GalaxyMap {
    type Err = GalaxyMapParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let map_width = str.lines().nth(0).unwrap_or("").len();
        let map_height = str.lines().count();
        let map: Vec<char> = str.lines().flat_map(|x| x.chars()).collect();

        let galaxies: Vec<Point> = map
            .iter()
            .enumerate()
            .filter(|(_i, &x)| x == '#')
            .map(|(i, _x)| Point {
                x: i % map_width,
                y: i / map_width,
            })
            .collect();

        let expanded_rows: Vec<usize> = map
            .chunks(map_width)
            .enumerate()
            .filter_map(|(row_index, row)| {
                Some(row_index).filter(|_| row.iter().all(|&x| x == '.'))
            })
            .collect();

        let expanded_cols: Vec<usize> = (0..map_width)
            .filter(|&col_index| {
                (0..map_height).all(|row_index| map[col_index + row_index * map_width] == '.')
            })
            .collect();

        return Ok(GalaxyMap {
            map,
            galaxies,
            map_width,
            map_height,
            expanded_rows,
            expanded_cols,
        });
    }
}

#[test]
fn part1_test1() {
    let result = part1("C:/git/advent_of_code/day11/input/test1.txt");
    assert_eq!(result, 374);
}

#[test]
fn part1_test2() {
    let result = part1("C:/git/advent_of_code/day11/input/test2.txt");
    assert_eq!(result, 9742154);
}

#[test]
fn part2_test1() {
    let result = part2("C:/git/advent_of_code/day11/input/test1.txt");
    assert_eq!(result, 82000210);
}

#[test]
fn part2_test2() {
    let result = part2("C:/git/advent_of_code/day11/input/test2.txt");
    assert_eq!(result, 411142919886);
}
