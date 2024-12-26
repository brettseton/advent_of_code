use std::collections::{HashMap, VecDeque};
use std::fs;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Keys {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
    KeyUp,
    KeyRight,
    KeyDown,
    KeyLeft,
    Empty,
}

use Keys::*;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Keypad {
    shortest_paths: HashMap<Keys, HashMap<Keys, Vec<Vec<Keys>>>>,
}

impl Keypad {
    fn new(layout: Vec<(Point, Keys)>) -> Self {
        let layout = layout.into_iter().collect();
        let shortest_paths = Self::calculate_shortest_paths(&layout);
        Self { shortest_paths }
    }

    fn calculate_shortest_paths(
        layout: &HashMap<Point, Keys>,
    ) -> HashMap<Keys, HashMap<Keys, Vec<Vec<Keys>>>> {
        layout
            .iter()
            .map(|(coord, key)| (*key, Self::find_shortest_paths(*coord, layout)))
            .collect()
    }

    fn find_shortest_paths(
        start: Point,
        layout: &HashMap<Point, Keys>,
    ) -> HashMap<Keys, Vec<Vec<Keys>>> {
        let mut paths: HashMap<Keys, Vec<Vec<Keys>>> = HashMap::new();

        let mut to_do: VecDeque<(Point, Vec<Keys>)> = vec![(start, vec![])].into();

        let start_key = layout.get(&start).unwrap();
        paths.insert(*start_key, vec![]);

        while let Some((coordinate, path)) = to_do.pop_front() {
            let current_key = layout.get(&coordinate).unwrap();

            let shortest = paths.entry(*current_key).or_default();

            let mut deduped_shortest = shortest.last().unwrap_or(&vec![]).clone();
            deduped_shortest.dedup();
            let mut deduped_path = path.clone();
            deduped_path.dedup();

            if deduped_shortest.is_empty() || deduped_shortest.len() > deduped_path.len() {
                *shortest = vec![path.clone()];
            } else if deduped_shortest.len() == deduped_path.len() {
                (*shortest).push(path.clone());
            } else {
                continue;
            }

            let neighbors = [
                (
                    Point {
                        x: coordinate.x,
                        y: coordinate.y.wrapping_sub(1),
                    },
                    KeyUp,
                ),
                (
                    Point {
                        x: coordinate.x + 1,
                        y: coordinate.y,
                    },
                    KeyRight,
                ),
                (
                    Point {
                        x: coordinate.x,
                        y: coordinate.y + 1,
                    },
                    KeyDown,
                ),
                (
                    Point {
                        x: coordinate.x.wrapping_sub(1),
                        y: coordinate.y,
                    },
                    KeyLeft,
                ),
            ];

            for (neighbor, direction) in neighbors {
                if let Some(key) = layout.get(&neighbor) {
                    if *key != Empty && key != start_key {
                        let mut next_path = path.clone();
                        next_path.push(direction);
                        to_do.push_back((neighbor, next_path));
                    }
                }
            }
        }
        for sub_path in paths.values_mut() {
            for path in sub_path.iter_mut() {
                path.push(KeyA);
            }
        }
        paths
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct PathPair {
    from: Keys,
    to: Keys,
}

struct ComplexityCalculator {
    numpad: Keypad,
    dpad: Keypad,
}

impl ComplexityCalculator {
    fn new() -> Self {
        let numpad = Keypad::new(vec![
            (Point { x: 0, y: 0 }, Key7),
            (Point { x: 1, y: 0 }, Key8),
            (Point { x: 2, y: 0 }, Key9),
            (Point { x: 0, y: 1 }, Key4),
            (Point { x: 1, y: 1 }, Key5),
            (Point { x: 2, y: 1 }, Key6),
            (Point { x: 0, y: 2 }, Key1),
            (Point { x: 1, y: 2 }, Key2),
            (Point { x: 2, y: 2 }, Key3),
            (Point { x: 0, y: 3 }, Empty),
            (Point { x: 1, y: 3 }, Key0),
            (Point { x: 2, y: 3 }, KeyA),
        ]);

        let dpad = Keypad::new(vec![
            (Point { x: 0, y: 0 }, Empty),
            (Point { x: 1, y: 0 }, KeyUp),
            (Point { x: 2, y: 0 }, KeyA),
            (Point { x: 0, y: 1 }, KeyLeft),
            (Point { x: 1, y: 1 }, KeyDown),
            (Point { x: 2, y: 1 }, KeyRight),
        ]);

        Self { numpad, dpad }
    }

    fn calculate_complexity(&self, code: &str, num_levels: usize) -> i64 {
        let (sequence, value) = parse_input(code);
        let mut total_len = 0;
        let mut previous_key = KeyA;

        for &key in &sequence {
            total_len += self.find_min_moves(previous_key, key, num_levels);
            previous_key = key;
        }

        total_len as i64 * value as i64
    }

    fn compute_cost(
        &self,
        path: PathPair,
        max: usize,
        current: usize,
        memo: &mut HashMap<PathPair, HashMap<usize, usize>>,
        last_at_level: &mut HashMap<usize, Keys>,
    ) -> usize {
        if current == max {
            1
        } else {
            if let Some(path_cost) = memo.get(&path) {
                if let Some(cost) = path_cost.get(&current) {
                    return *cost;
                }
            }

            let next_path = (if current == 0 {
                &self.numpad
            } else {
                &self.dpad
            })
            .shortest_paths
            .get(&path.from)
            .unwrap()
            .get(&path.to)
            .unwrap();

            let last = *last_at_level.entry(current).or_insert(KeyA);
            let mut next_last = last;
            let mut total = usize::MAX;

            for possible_paths in next_path {
                let mut sub_total = 0;
                let mut previous = last;
                for part in possible_paths {
                    let fragment_cost = self.compute_cost(
                        PathPair {
                            from: previous,
                            to: *part,
                        },
                        max,
                        current + 1,
                        memo,
                        last_at_level,
                    );
                    previous = *part;
                    sub_total += fragment_cost;
                }
                if sub_total < total {
                    total = sub_total;
                    next_last = *possible_paths.last().unwrap();
                }
            }

            last_at_level.insert(current, next_last);

            let saved_path = memo.entry(path).or_default();
            saved_path.insert(current, total);

            total
        }
    }

    fn find_min_moves(&self, from: Keys, to: Keys, levels: usize) -> usize {
        let mut memo = HashMap::new();
        let mut last_at_level = HashMap::new();
        self.compute_cost(
            PathPair { from, to },
            levels,
            0,
            &mut memo,
            &mut last_at_level,
        )
    }
}

fn parse_input(input: &str) -> (Vec<Keys>, usize) {
    let line = input.lines().next().unwrap();
    (
        line.chars().map(char_to_key).collect(),
        line[0..3].parse().unwrap(),
    )
}

fn char_to_key(c: char) -> Keys {
    match c {
        '0' => Key0,
        '1' => Key1,
        '2' => Key2,
        '3' => Key3,
        '4' => Key4,
        '5' => Key5,
        '6' => Key6,
        '7' => Key7,
        '8' => Key8,
        '9' => Key9,
        'A' => KeyA,
        _ => unreachable!(),
    }
}

fn part1(input: &str) -> i64 {
    let calculator = ComplexityCalculator::new();
    input
        .lines()
        .map(|code| calculator.calculate_complexity(code, 3))
        .sum()
}

fn part2(input: &str) -> i64 {
    let calculator = ComplexityCalculator::new();
    input
        .lines()
        .map(|code| calculator.calculate_complexity(code, 26))
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
        assert_eq!(part1(&test_input), 126384);
    }

    #[test]
    fn test2_part1() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 174124);
    }

    #[test]
    fn test1_part2() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 154115708116294);
    }

    #[test]
    fn test2_part2() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 216668579770346);
    }
}
