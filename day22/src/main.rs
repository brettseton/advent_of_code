use std::{fs, str::FromStr, usize, collections::{HashSet, HashMap}};

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
    return brick_stack.get_disintegration_count();
}

fn part2(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let brick_stack = BrickStack::new(&input);
    return brick_stack.get_disintegration_count();
}

#[derive(Debug, Clone)]
struct Brick {
    id: usize,
    start_point: Point3D,
    end_point: Point3D,
    resting_on: HashSet<usize>,
}

impl Brick {
    fn drop(&mut self, occupancy_grid: &mut Vec<Vec<Vec<Option<usize>>>>) {
        let min_x = self.start_point.x.min(self.end_point.x);
        let max_x = self.start_point.x.max(self.end_point.x);
        let min_y = self.start_point.y.min(self.end_point.y);
        let max_y = self.start_point.y.max(self.end_point.y);
        let min_z = self.start_point.z.min(self.end_point.z);
        let max_z = self.start_point.z.max(self.end_point.z);
        let block_height = max_z - min_z + 1;

        for z in (1..self.start_point.z).rev() {
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    match occupancy_grid[z][y][x] {
                        Some(id) => {
                            println!("collided with {} resting of {:?} at height: {}", id, self, z+1);
                            self.resting_on.insert(id);

                            // Find any other blocks this is resting on
                            for y in min_y..=max_y {
                                for x in min_x..=max_x {
                                    match occupancy_grid[z][y][x] {
                                        Some(id) => {
                                            self.resting_on.insert(id);
                                        }
                                        None => (),
                                    }
                                }
                            }

                            // Fill the occupancy grid for the other blocks to use
                            for dz in 1..=block_height {
                                for y in min_y..=max_y {
                                    for x in min_x..=max_x {
                                        occupancy_grid[z + dz][y][x] = Some(self.id);
                                    }
                                }
                            }
                            return;
                        }
                        None => (),
                    }
                }
            }
        }

        // no collisions and resting on the ground
        println!("resting on the ground {:?}", self);
        for dz in 1..=block_height {
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    occupancy_grid[dz][y][x] = Some(self.id);
                }
            }
        }
        return;
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
        return BrickStack::from_str(str).expect("");
    }

    pub fn get_disintegration_count(&self) -> usize {
        let mut bricks_ordered = self.bricks.clone();
        bricks_ordered.sort_by_key(|b| {
            b.start_point.z.min(b.end_point.z)
        });

        let mut occupancy_grid: Vec<Vec<Vec<Option<usize>>>> = vec![vec![vec![None; 10]; 10]; bricks_ordered.last().unwrap().end_point.z];
        let mut parent_of: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();

        for brick in bricks_ordered.iter_mut() {
            brick.drop(&mut occupancy_grid);

            parent_of.insert(brick.id, vec![]);
            for &resting_on in brick.resting_on.iter() {
                parent_of
                    .entry(resting_on)
                    .or_insert(vec![])
                    .push((brick.id, brick.resting_on.len()));
            }
        }

        let mut disintegrate = vec![];
        for (k, v) in parent_of.iter() {
            if v.is_empty() || v.iter().all(|x| x.1 > 1) {
                disintegrate.push(k);
            }
        }

        // for (z, plane) in occupancy_grid.iter().enumerate() {
        //     println!("height {} grid:", z);
        //     for (y, line) in plane.iter().enumerate() {
        //         let mut out = "".to_string();
        //         for (x, brick) in line.iter().enumerate() {
        //             match brick {
        //                 None => out.push('.'),
        //                 Some(id) => out.push(((((id % 74)) as u8) + 48) as char)
        //             }
        //         }
        //         println!("{}", out);
        //     }
        //     println!("----------------------");
        // }
        return disintegrate.len();
    }
}

#[derive(Debug)]
struct BrickStackError;

impl FromStr for BrickStack {
    type Err = BrickStackError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bricks: Vec<Brick> = s
            .lines()
            .enumerate()
            .map(|(i, s)| {
                let [start_str, end_str] =
                    &s.split('~').map(String::from).collect::<Vec<String>>()[..]
                else {
                    panic!()
                };
                return Brick {
                    id: i,
                    start_point: start_str.parse::<Point3D>().expect(""),
                    end_point: end_str.parse::<Point3D>().expect(""),
                    resting_on: HashSet::new(),
                };
            })
            .collect();

        return Ok(BrickStack { bricks });
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
    assert_eq!(ans, 0);
}

#[test]
pub fn part2_test2() {
    let ans = part2("input/test2.txt");
    assert_eq!(ans, 0);
}
