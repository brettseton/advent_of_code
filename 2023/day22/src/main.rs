use std::{
    collections::{HashSet, VecDeque},
    fs,
    str::FromStr,
};

fn main() {
    let ans = part1("input/test1.txt");
    println!("part 1 test 1 : {}", ans);

    let ans = part1("input/test2.txt");
    println!("part 1 test 2 : {}", ans);

    let ans = part2("input/test1.txt");
    println!("part 2 test 1 : {}", ans);

    let ans = part2("input/test2.txt");
    println!("part 2 test 2 : {}", ans);
}

fn part1(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let brick_stack = BrickStack::new(&input);
    brick_stack.get_disintegration_count_and_fallen_sum().0
}

fn part2(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let brick_stack = BrickStack::new(&input);
    brick_stack.get_disintegration_count_and_fallen_sum().1
}

#[derive(Debug, Clone)]
struct Brick {
    start_point: Point3D,
    end_point: Point3D,
}

impl Brick {
    fn x_range(&self) -> std::ops::RangeInclusive<usize> {
        self.start_point.x.min(self.end_point.x)..=self.start_point.x.max(self.end_point.x)
    }

    fn y_range(&self) -> std::ops::RangeInclusive<usize> {
        self.start_point.y.min(self.end_point.y)..=self.start_point.y.max(self.end_point.y)
    }

    fn min_z(&self) -> usize {
        self.start_point.z.min(self.end_point.z)
    }

    fn max_z(&self) -> usize {
        self.start_point.z.max(self.end_point.z)
    }

    fn height(&self) -> usize {
        self.max_z() - self.min_z() + 1
    }
}

impl FromStr for Brick {
    type Err = Point3DErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('~');
        let start_point = parts.next().ok_or(Point3DErr)?.parse()?;
        let end_point = parts.next().ok_or(Point3DErr)?.parse()?;
        Ok(Brick {
            start_point,
            end_point,
        })
    }
}

#[derive(Debug, Clone)]
struct Point3D {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug)]
struct Point3DErr;

impl FromStr for Point3D {
    type Err = Point3DErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();

        if coords.len() != 3 {
            return Err(Point3DErr);
        }

        let x = coords[0].trim().parse().unwrap();
        let y = coords[1].trim().parse().unwrap();
        let z = coords[2].trim().parse().unwrap();

        Ok(Point3D { x, y, z })
    }
}

struct BrickStack {
    bricks: Vec<Brick>,
}

impl BrickStack {
    pub fn new(str: &str) -> BrickStack {
        BrickStack::from_str(str).expect("")
    }

    pub fn get_disintegration_count_and_fallen_sum(&self) -> (usize, usize) {
        let mut bricks = self.bricks.clone();
        bricks.sort_by_key(|b| b.min_z());

        let max_x = bricks.iter().map(|b| *b.x_range().end()).max().unwrap_or(0);
        let max_y = bricks.iter().map(|b| *b.y_range().end()).max().unwrap_or(0);

        let mut height_map = vec![vec![(0, None); max_x + 1]; max_y + 1];
        let mut supported_by: Vec<HashSet<usize>> = vec![HashSet::new(); bricks.len()];
        let mut supports: Vec<HashSet<usize>> = vec![HashSet::new(); bricks.len()];

        for i in 0..bricks.len() {
            let b = &bricks[i];
            let x_range = b.x_range();
            let y_range = b.y_range();
            let height = b.height();

            let mut max_h = 0;
            for y in y_range.clone() {
                for x in x_range.clone() {
                    max_h = max_h.max(height_map[y][x].0);
                }
            }

            let new_top_h = max_h + height;
            for y in y_range {
                for x in x_range.clone() {
                    if let (h, Some(prev_idx)) = height_map[y][x] {
                        if h == max_h {
                            supported_by[i].insert(prev_idx);
                            supports[prev_idx].insert(i);
                        }
                    }
                    height_map[y][x] = (new_top_h, Some(i));
                }
            }
        }

        let mut can_disintegrate_count = 0;
        for support in supports.iter().take(bricks.len()) {
            if support
                .iter()
                .all(|&supported_idx| supported_by[supported_idx].len() > 1)
            {
                can_disintegrate_count += 1;
            }
        }

        let mut fallen_sum = 0;
        for i in 0..bricks.len() {
            let mut q = VecDeque::new();
            q.push_back(i);

            let mut removed_supports = vec![0; bricks.len()];
            let mut fallen_count = 0;

            while let Some(curr) = q.pop_front() {
                for &next in &supports[curr] {
                    removed_supports[next] += 1;
                    if removed_supports[next] == supported_by[next].len() {
                        fallen_count += 1;
                        q.push_back(next);
                    }
                }
            }
            fallen_sum += fallen_count;
        }

        (can_disintegrate_count, fallen_sum)
    }
}

#[derive(Debug)]
struct BrickStackError;

impl FromStr for BrickStack {
    type Err = BrickStackError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bricks: Vec<Brick> = s
            .lines()
            .map(|s| s.parse().expect("Failed to parse brick"))
            .collect();

        Ok(BrickStack { bricks })
    }
}

#[test]
pub fn part1_test1() {
    let ans = part1("input/test1.txt");
    assert_eq!(ans, 5);
}

#[test]
pub fn part1_test2() {
    let ans = part1("input/test2.txt");
    assert_eq!(ans, 492);
}

#[test]
pub fn part2_test1() {
    let ans = part2("input/test1.txt");
    assert_eq!(ans, 7);
}

#[test]
pub fn part2_test2() {
    let ans = part2("input/test2.txt");
    assert_eq!(ans, 86556);
}
