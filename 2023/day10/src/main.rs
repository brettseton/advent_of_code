use std::collections::VecDeque;
use std::fs;
use std::str::FromStr;

fn main() {
    let ans = part1("input/test1.txt");
    println!("part 1 test 1 answer: {}", ans);

    let ans = part1("input/test2.txt");
    println!("part 1 test 2 answer: {}", ans);

    let ans = part2("input/test1.txt");
    println!("part 2 test 1 answer: {}", ans);

    let ans = part2("input/test2.txt");
    println!("part 2 test 2 answer: {}", ans);
}

fn part1(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("Unable to read the input file");
    let pipe_map = PipeMap::new(&input);
    let loop_length = pipe_map.get_loop_length();
    return loop_length;
}

fn part2(file_path: &str) -> i32 {
    let input = fs::read_to_string(file_path).expect("Unable to read the input file");
    let pipe_map = PipeMap::new(&input);
    return pipe_map.get_loop_area();
}

#[derive(Debug)]
struct PipeMap {
    start_location: Point,
    map: Vec<char>,
    map_width: usize,
    map_height: usize,
}

impl PipeMap {
    pub fn new(str: &str) -> PipeMap {
        return PipeMap::from_str(str).expect("Ctor from string failed");
    }

    pub fn get(&self, x: usize, y: usize) -> Option<char> {
        if x < self.map_width && y < self.map_height {
            let index = self.get_1d(x, y);
            Some(self.map[index])
        } else {
            None
        }
    }

    pub fn get_1d(&self, x: usize, y: usize) -> usize {
        return x + y * self.map_width;
    }

    pub fn get_1d_from_point(&self, p: &Point) -> usize {
        return self.get_1d(p.x, p.y);
    }

    pub fn get_loop_length(&self) -> usize {
        return *self
            .get_loop()
            .iter()
            .flat_map(|x| x.iter())
            .max()
            .expect("needs an element") as usize;
    }

    pub fn get_loop_area(&self) -> i32 {
        let mut visited_map = vec![-1; self.map_height * self.map_width];
        visited_map[self.start_location.x + self.start_location.y * self.map_width] = 0;

        let mut search_queue = VecDeque::new();

        let connected_neighbors = self.get_start_neighbors(&self.start_location);

        search_queue.push_back((
            connected_neighbors.first().unwrap().clone(),
            self.start_location.clone(),
        ));

        let mut area: i32 = 0;

        while let Some(point) = search_queue.pop_front() {
            // already visited
            if visited_map[self.get_1d_from_point(&point.0)] != -1 {
                continue;
            }

            // Shoelace formula https://en.wikipedia.org/wiki/Shoelace_formula
            area += (point.1.y as i32 + point.0.y as i32) * (point.1.x as i32 - point.0.x as i32);

            visited_map[self.get_1d_from_point(&point.0)] =
                visited_map[self.get_1d_from_point(&point.1)] + 1;
            let neighbors = self.get_connected_neighbors(&point.0);

            for neighbor in neighbors {
                if visited_map[self.get_1d_from_point(&neighbor)] != -1 {
                    continue;
                }
                search_queue.push_back((
                    neighbor,
                    Point {
                        x: point.0.x,
                        y: point.0.y,
                    },
                ));
            }
        }

        // Pick's theorem https://en.wikipedia.org/wiki/Pick%27s_theorem
        let loop_length = visited_map.iter().max().unwrap() + 1;
        return area.abs() / 2 + 1 - loop_length / 2;
    }

    pub fn get_loop(&self) -> Vec<Vec<i32>> {
        let mut visited_map = vec![-1; self.map_height * self.map_width];
        visited_map[self.start_location.x + self.start_location.y * self.map_width] = 0;

        let mut search_queue = VecDeque::new();

        let connected_neighbors = self.get_start_neighbors(&self.start_location);

        for neighbor in connected_neighbors {
            search_queue.push_back((neighbor, self.start_location.clone()));
        }

        while let Some(point) = search_queue.pop_front() {
            // already visited
            if visited_map[self.get_1d_from_point(&point.0)] != -1 {
                continue;
            }
            visited_map[self.get_1d_from_point(&point.0)] =
                visited_map[self.get_1d_from_point(&point.1)] + 1;
            let neighbors = self.get_connected_neighbors(&point.0);

            for neighbor in neighbors {
                search_queue.push_back((
                    neighbor,
                    Point {
                        x: point.0.x,
                        y: point.0.y,
                    },
                ));
            }
        }

        let loop_map = visited_map
            .chunks(self.map_width)
            .map(|x| x.to_vec())
            .collect::<Vec<Vec<i32>>>();

        return loop_map;
    }

    pub fn get_start_neighbors(&self, start: &Point) -> Vec<Point> {
        let mut start_neighbors = Vec::new();

        // West
        if start.x > 0
            && PipeMap::get_connection_directions(self.get(start.x - 1, start.y).unwrap_or('.'))
                .contains(&Directions::East)
        {
            start_neighbors.push(Point {
                x: start.x - 1,
                y: start.y,
            });
        }
        // East
        if start.x < self.map_width - 1
            && PipeMap::get_connection_directions(self.get(start.x + 1, start.y).unwrap_or('.'))
                .contains(&Directions::West)
        {
            start_neighbors.push(Point {
                x: start.x + 1,
                y: start.y,
            });
        }

        // North
        if start.y > 0
            && PipeMap::get_connection_directions(self.get(start.x, start.y - 1).unwrap_or('.'))
                .contains(&Directions::South)
        {
            start_neighbors.push(Point {
                x: start.x,
                y: start.y - 1,
            });
        }

        // South
        if start.y < self.map_height - 1
            && PipeMap::get_connection_directions(self.get(start.x, start.y + 1).unwrap_or('.'))
                .contains(&Directions::North)
        {
            start_neighbors.push(Point {
                x: start.x,
                y: start.y + 1,
            });
        }

        return start_neighbors;
    }

    pub fn get_connection_directions(pipe: char) -> Vec<Directions> {
        return match pipe {
            'S' => vec![
                Directions::North,
                Directions::East,
                Directions::South,
                Directions::West,
            ],
            '|' => vec![Directions::North, Directions::South],
            '-' => vec![Directions::East, Directions::West],
            'L' => vec![Directions::North, Directions::East],
            'J' => vec![Directions::North, Directions::West],
            '7' => vec![Directions::South, Directions::West],
            'F' => vec![Directions::East, Directions::South],
            '.' => vec![],
            _ => panic!("unexpected character"),
        };
    }

    pub fn get_connected_neighbors(&self, p: &Point) -> Vec<Point> {
        let pipe1_connections =
            PipeMap::get_connection_directions(self.get(p.x, p.y).unwrap_or('.'));
        return self.get_neighbors_from_directions(p, pipe1_connections);
    }

    pub fn get_neighbors_from_directions(
        &self,
        p: &Point,
        directions: Vec<Directions>,
    ) -> Vec<Point> {
        let mut neighbors = Vec::new();
        for direction in directions {
            match direction {
                Directions::North => neighbors.push(Point { x: p.x, y: p.y - 1 }),
                Directions::East => neighbors.push(Point { x: p.x + 1, y: p.y }),
                Directions::South => neighbors.push(Point { x: p.x, y: p.y + 1 }),
                Directions::West => neighbors.push(Point { x: p.x - 1, y: p.y }),
            }
        }
        return neighbors;
    }
}

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(PartialEq)]
enum Directions {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct PipeMapParseError;

impl FromStr for PipeMap {
    type Err = PipeMapParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let map_width = str.lines().nth(0).unwrap_or("").len();
        let map_height = str.lines().count();
        let map: Vec<char> = str.lines().flat_map(|x| x.chars()).collect();
        let start_location = map
            .iter()
            .enumerate()
            .position(|(_y, ch)| *ch == 'S')
            .map(|pos| (pos % map_width, pos / map_width))
            .expect("needs to be a start location");

        return Ok(PipeMap {
            start_location: Point {
                x: start_location.0,
                y: start_location.1,
            },
            map,
            map_width,
            map_height,
        });
    }
}

#[test]
fn part1_test1() {
    let result = part1("input/test1.txt");
    assert_eq!(result, 8);
}

#[test]
fn part1_test2() {
    let result = part1("input/test2.txt");
    assert_eq!(result, 6927);
}

#[test]
fn part2_test1() {
    let result = part2("input/test1.txt");
    assert_eq!(result, 1);
}

#[test]
fn part2_test2() {
    let result = part2("input/test2.txt");
    assert_eq!(result, 467);
}

#[test]
fn part2_test3() {
    let result = part2("input/test3.txt");
    assert_eq!(result, 4);
}

#[test]
fn part2_test4() {
    let result = part2("input/test4.txt");
    assert_eq!(result, 8);
}

#[test]
fn part2_test5() {
    let result = part2("input/test5.txt");
    assert_eq!(result, 10);
}
