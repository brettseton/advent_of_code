use std::{fs, str::FromStr};

fn main() {
    let ans = part1("input/test1.txt", 7.0, 27.0);
    println!("part 1 test 1 : {}", ans);

    let ans = part1("input/test2.txt", 200000000000000.0, 400000000000000.0);
    println!("part 1 test 2 : {}", ans);

    let ans = part2("input/test1.txt");
    println!("part 2 test 1 : {}", ans);

    let ans = part2("input/test2.txt");
    println!("part 2 test 2 : {}", ans);
}

fn part1(file_path: &str, min_bound: f64, max_bound: f64) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let hail_storm = HailStorm::new(&input);
    return hail_storm.get_collision_count(min_bound, max_bound);
}

fn part2(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let hail_storm = HailStorm::new(&input);
    return hail_storm.get_collision_count(7.0, 27.0);
}

struct HailStorm {
    hail: Vec<Hail>,
}

impl HailStorm {
    pub fn new(str: &str) -> HailStorm {
        return HailStorm::from_str(str).expect("");
    }

    fn get_collision_count(&self, min: f64, max: f64) -> usize {
        let mut collisions = 0;
        for i in 0..self.hail.len() - 1 {
            for j in i + 1..self.hail.len() {
                let hail1 = &self.hail[i];
                let hail2 = &self.hail[j];
                if let Some(p) = hail1.find_intersection(&hail2) {
                    if p.x >= min && p.x <= max && p.y >= min && p.y <= max {
                        collisions += 1;
                        //println!("intersects in bounds {:?}, {:?} @ {:?}", hail1, hail2, p);
                    }
                }
            }
        }
        return collisions;
    }
}

#[derive(Debug)]
struct HailStormError;

impl FromStr for HailStorm {
    type Err = HailStormError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hail = s
            .lines()
            .map(|s| s.parse::<Hail>().expect(""))
            .collect::<Vec<Hail>>();
        return Ok(HailStorm { hail });
    }
}

#[derive(Debug)]
struct Hail {
    position: Point3D,
    velocity: Point3D,
}

impl Hail {
    // Get a point on the line at a given parameter t
    fn point_at_parameter(&self, t: f64) -> Point3D {
        Point3D {
            x: self.position.x + t * self.velocity.x,
            y: self.position.y + t * self.velocity.y,
            z: self.position.z + t * self.velocity.z,
        }
    }

    // Find the intersection point with another line
    fn find_intersection(&self, other: &Hail) -> Option<Point3D> {
        // Solve for t values at the intersection point
        let det = self.velocity.x * other.velocity.y - self.velocity.y * other.velocity.x;

        if det.abs() < 1e-10 {
            // Lines are parallel
            None
        } else {
            let t1 = ((other.position.x - self.position.x) * other.velocity.y - (other.position.y - self.position.y) * other.velocity.x)/ det;
            let t2 = ((other.position.x - self.position.x) * self.velocity.y - (other.position.y - self.position.y) * self.velocity.x)/ det;

            // Check if the intersection point is within the line segments
            if t1 >= 0.0 && t2 >= 0.0 {
                Some(self.point_at_parameter(t1))
            } else {
                None
            }
        }
    }
}

#[derive(Debug)]
struct HailError;

impl FromStr for Hail {
    type Err = HailError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [position_str, velocity_str] =
            &s.split('@').map(String::from).collect::<Vec<String>>()[..]
        else {
            panic!()
        };

        return Ok(Hail {
            position: position_str.parse::<Point3D>().expect(""),
            velocity: velocity_str.parse::<Point3D>().expect(""),
        });
    }
}

#[derive(Debug)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
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

#[test]
pub fn part1_test1() {
    let ans = part1("input/test1.txt", 7.0, 27.0);
    assert_eq!(ans, 2);
}

#[test]
pub fn part1_test2() {
    let ans = part1("input/test2.txt", 200000000000000.0, 400000000000000.0);
    assert_eq!(ans, 27732);
}

#[test]
pub fn part2_test1() {
    let ans = part2("input/test1.txt");
    assert_eq!(ans, 47);
}

#[test]
pub fn part2_test2() {
    let ans = part2("input/test2.txt");
    assert_eq!(ans, 0);
}
