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

fn part2(file_path: &str) -> f64 {
    let input = fs::read_to_string(file_path).expect("file input");
    let hail_storm = HailStorm::new(&input);
    return hail_storm.get_magic_stone();
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
                if let Some(p) = hail1.find_intersection(hail2) {
                    if p.x >= min && p.x <= max && p.y >= min && p.y <= max {
                        collisions += 1;
                        //println!("intersects in bounds {:?}, {:?} @ {:?}", hail1, hail2, p);
                    }
                }
            }
        }
        return collisions;
    }

    fn get_magic_stone(&self) -> f64 {
        // Find 3 non-linear stones and solve
        let h1 = self.hail[0].clone();
        let mut h2 = self.hail[0].clone();
        let mut h3 = self.hail[0].clone();
        
        // Search from the start to find a stone that is not linear
        for i in 1..self.hail.len() - 1 {
            if h1.velocity.not_linear(&self.hail[i].velocity) {
                h2 = self.hail[i].clone();
                break;
            }
        }

        // Search from the end to find a stone that is not linear
        // with both existing stone
        for j in (1..self.hail.len()).rev() {
            if h1.velocity.not_linear(&self.hail[j].velocity)
                && h2.velocity.not_linear(&self.hail[j].velocity)
            {
                h3 = self.hail[j].clone();
                break;
            }
        }

        let magic_rock = HailStorm::find_rock(&h1, &h2, &h3);

        return (magic_rock.0.x + magic_rock.0.y + magic_rock.0.z) / magic_rock.1;
    }

    fn find_rock(h1: &Hail, h2: &Hail, h3: &Hail) -> (Point3D, f64) {
        let (a, a_size) = h1.find_plane(h2);
        let (b, b_size) = h1.find_plane(h3);
        let (c, c_size) = h2.find_plane(h3);

        let mut w = HailStorm::lin(
            a_size,
            &b.cross(&c),
            b_size,
            &c.cross(&a),
            c_size,
            &a.cross(&b),
        );
        let t = a.dot(&b.cross(&c));

        w = Point3D {
            x: (w.x / t).round(),
            y: (w.y / t).round(),
            z: (w.z / t).round(),
        };

        let w1 = h1.velocity.sub(&w);
        let w2 = h2.velocity.sub(&w);
        let ww = w1.cross(&w2);

        let e = ww.dot(&h2.position.cross(&w2));
        let f = ww.dot(&h1.position.cross(&w1));
        let g = h1.position.dot(&ww);
        let s = ww.dot(&ww);

        let rock = HailStorm::lin(e, &w1, -f, &w2, g, &ww);
        return (rock, s);
    }

    fn lin(a_scale: f64, a: &Point3D, b_scale: f64, b: &Point3D, c_scale: f64, c: &Point3D) -> Point3D {
        let x = a_scale * a.x + b_scale * b.x + c_scale * c.x;
        let y = a_scale * a.y + b_scale * b.y + c_scale * c.y;
        let z = a_scale * a.z + b_scale * b.z + c_scale * c.z;
        return Point3D { x, y, z };
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

#[derive(Debug, Clone)]
struct Hail {
    position: Point3D,
    velocity: Point3D,
}

impl Hail {
    fn find_plane(&self, other: &Hail) -> (Point3D, f64) {
        let p12 = self.position.sub(&other.position);
        let v12 = self.velocity.sub(&other.velocity);
        let vv = self.velocity.cross(&other.velocity);
        return (p12.cross(&v12), p12.dot(&vv));
    }

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

#[derive(Debug, Clone)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D {
    fn cross(&self, other: &Point3D) -> Point3D {
        return Point3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        };
    }

    fn dot(&self, other: &Point3D) -> f64 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    fn sub(&self, other: &Point3D) -> Point3D {
        return Point3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }

    fn not_linear(&self, other: &Point3D) -> bool {
        let cross = self.cross(other);
        return cross.x != 0.0 || cross.y != 0.0 || cross.z != 0.0;
    }
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
    assert_eq!(ans, 47.0);
}

#[test]
pub fn part2_test2() {
    let ans = part2("input/test2.txt");
    assert_eq!(ans, 641619849766168.0);
}
