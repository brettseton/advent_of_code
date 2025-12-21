use std::collections::HashSet;

const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Interval {
    start: i64,
    end: i64,
}

impl Interval {
    fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }

    fn len(&self) -> i64 {
        (self.end - self.start + 1).max(0)
    }

    fn contains(&self, val: i64) -> bool {
        val >= self.start && val <= self.end
    }

    fn merge_all(mut intervals: Vec<Self>) -> Vec<Self> {
        if intervals.is_empty() {
            return Vec::new();
        }
        intervals.sort_unstable();

        let mut merged = Vec::with_capacity(intervals.len());
        let mut current = intervals[0];

        for interval in intervals.into_iter().skip(1) {
            if interval.start <= current.end {
                current.end = current.end.max(interval.end);
            } else {
                merged.push(current);
                current = interval;
            }
        }
        merged.push(current);
        merged
    }
}

struct Sensor {
    position: Point,
    beacon: Point,
    radius: i64,
}

impl Sensor {
    fn from_line(line: &str) -> Option<Self> {
        let parts: Vec<i64> = line
            .split(|c: char| !c.is_numeric() && c != '-')
            .filter_map(|s| s.parse().ok())
            .collect();

        if parts.len() == 4 {
            let position = Point {
                x: parts[0],
                y: parts[1],
            };
            let beacon = Point {
                x: parts[2],
                y: parts[3],
            };
            let radius = (position.x - beacon.x).abs() + (position.y - beacon.y).abs();
            Some(Self {
                position,
                beacon,
                radius,
            })
        } else {
            None
        }
    }

    fn covers(&self, point: Point) -> bool {
        (self.position.x - point.x).abs() + (self.position.y - point.y).abs() <= self.radius
    }

    fn coverage_at_row(&self, y: i64) -> Option<Interval> {
        let dy = (self.position.y - y).abs();
        if dy <= self.radius {
            let dx = self.radius - dy;
            Some(Interval::new(self.position.x - dx, self.position.x + dx))
        } else {
            None
        }
    }
}

struct SensorSystem {
    sensors: Vec<Sensor>,
}

impl SensorSystem {
    fn new(input: &str) -> Self {
        let sensors = input.lines().filter_map(Sensor::from_line).collect();
        Self { sensors }
    }

    fn count_impossible_positions(&self, target_row: i64) -> i64 {
        let intervals: Vec<_> = self
            .sensors
            .iter()
            .filter_map(|s| s.coverage_at_row(target_row))
            .collect();

        let merged = Interval::merge_all(intervals);
        let total_covered: i64 = merged.iter().map(|i| i.len()).sum();

        let beacons_in_coverage = self
            .sensors
            .iter()
            .filter(|s| s.beacon.y == target_row)
            .map(|s| s.beacon.x)
            .collect::<HashSet<_>>()
            .into_iter()
            .filter(|&x| merged.iter().any(|i| i.contains(x)))
            .count() as i64;

        total_covered - beacons_in_coverage
    }

    fn find_distress_beacon_frequency(&self, max_coord: i64) -> Option<i64> {
        let mut pos_coeffs = Vec::with_capacity(self.sensors.len() * 2);
        let mut neg_coeffs = Vec::with_capacity(self.sensors.len() * 2);

        for s in &self.sensors {
            let d = s.radius + 1;
            pos_coeffs.push(s.position.y - s.position.x + d);
            pos_coeffs.push(s.position.y - s.position.x - d);
            neg_coeffs.push(s.position.y + s.position.x + d);
            neg_coeffs.push(s.position.y + s.position.x - d);
        }

        for &a in &pos_coeffs {
            for &b in &neg_coeffs {
                if (a + b) % 2 == 0 {
                    let x = (b - a) / 2;
                    let y = (a + b) / 2;

                    if x >= 0 && x <= max_coord && y >= 0 && y <= max_coord {
                        let candidate = Point { x, y };
                        if self.sensors.iter().all(|s| !s.covers(candidate)) {
                            return Some(x * 4_000_000 + y);
                        }
                    }
                }
            }
        }
        None
    }
}

fn part1(input: &str) -> i64 {
    let system = SensorSystem::new(input);
    let target_row = if system.sensors.len() < 20 {
        10
    } else {
        2000000
    };
    system.count_impossible_positions(target_row)
}

fn part2(input: &str) -> i64 {
    let system = SensorSystem::new(input);
    let max_coord = if system.sensors.len() < 20 {
        20
    } else {
        4000000
    };
    system
        .find_distress_beacon_frequency(max_coord)
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
        assert_eq!(part1(TEST_INPUT_1), 26);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 4861076);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 56000011);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 10649103160102);
    }
}
