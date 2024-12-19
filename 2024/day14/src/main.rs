use std::fs;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Robot {
    pos: Point,
    vel: Point,
}

fn parse_input(input: &str) -> (Vec<Robot>, Point) {
    let mut lines = input.lines();
    // Parse grid dimensions from first line
    let dims: Vec<i32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    let bounds = Point {
        x: dims[0],
        y: dims[1],
    };

    // Parse robots
    let robots = lines
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();
            let pos: Vec<i32> = parts[0][2..]
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect();
            let vel: Vec<i32> = parts[1][2..]
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect();
            Robot {
                pos: Point {
                    x: pos[0],
                    y: pos[1],
                },
                vel: Point {
                    x: vel[0],
                    y: vel[1],
                },
            }
        })
        .collect();

    (robots, bounds)
}

fn simulate_step(robot: &mut Robot, bounds: Point) {
    robot.pos.x = (robot.pos.x + robot.vel.x).rem_euclid(bounds.x);
    robot.pos.y = (robot.pos.y + robot.vel.y).rem_euclid(bounds.y);
}

fn count_robots_in_quadrants(robots: &[Robot], bounds: Point) -> Vec<i32> {
    let mut quadrants = vec![0; 4];
    let mid_x = bounds.x / 2;
    let mid_y = bounds.y / 2;

    for robot in robots {
        if robot.pos.x == mid_x || robot.pos.y == mid_y {
            continue; // Skip robots on the middle lines
        }

        let quadrant = match (robot.pos.x < mid_x, robot.pos.y < mid_y) {
            (true, true) => 0,   // Top-left
            (false, true) => 1,  // Top-right
            (true, false) => 2,  // Bottom-left
            (false, false) => 3, // Bottom-right
        };
        quadrants[quadrant] += 1;
    }
    quadrants
}

/// Detects if the robots have formed a tree-like pattern by checking for rows with 21 or more consecutive robots.
/// This pattern emerges when robots align in a way that resembles a Christmas tree shape.
///
/// # Arguments
/// * `robots` - A slice containing all robot positions
/// * `bounds` - The boundaries of the grid (width and height)
///
/// # Returns
/// * `true` if a tree pattern is detected (21+ consecutive robots in any row)
/// * `false` otherwise
fn detect_tree_pattern(robots: &[Robot], bounds: Point) -> bool {
    // Create a grid to mark robot positions
    let mut grid = vec![vec![false; bounds.x as usize]; bounds.y as usize];

    // Mark robot positions in grid
    for robot in robots {
        grid[robot.pos.y as usize][robot.pos.x as usize] = true;
    }

    // Check each row for consecutive robots
    for row in grid {
        let mut current_consecutive = 0;
        for has_robot in row {
            if has_robot {
                current_consecutive += 1;
                if current_consecutive >= 21 {
                    return true;
                }
            } else {
                current_consecutive = 0;
            }
        }
    }

    false
}

fn part1(input: &str) -> i32 {
    let (mut robots, bounds) = parse_input(input);

    // Simulate 100 seconds
    for _ in 0..100 {
        for robot in robots.iter_mut() {
            simulate_step(robot, bounds);
        }
    }

    // Calculate safety factor
    let quadrants = count_robots_in_quadrants(&robots, bounds);
    quadrants.iter().product()
}

// output the iterations to a png and sort on file size to help find the tree
fn part2(input: &str) -> i32 {
    let (mut robots, bounds) = parse_input(input);

    // Simulate until we find the tree pattern
    for i in 1..10000 {
        for robot in robots.iter_mut() {
            simulate_step(robot, bounds);
        }

        let like_tree = detect_tree_pattern(&robots, bounds);
        if like_tree {
            //Create an image of the current state
            let mut img = image::RgbImage::new(bounds.x as u32, bounds.y as u32);

            // Fill background with black
            for pixel in img.pixels_mut() {
                *pixel = image::Rgb([0, 0, 0]);
            }

            // Draw robots as white pixels
            for robot in &robots {
                if robot.pos.x >= 0
                    && robot.pos.x < bounds.x
                    && robot.pos.y >= 0
                    && robot.pos.y < bounds.y
                {
                    img.put_pixel(
                        robot.pos.x as u32,
                        robot.pos.y as u32,
                        image::Rgb([255, 255, 255]),
                    );
                }
            }

            //Save the image to visualise the pattern
            img.save(format!("output/iteration_{}.png", i))
                .expect("Failed to save image");
            return i;
        }
    }

    -1 // Pattern not found within limit
}

fn main() {
    let input1 =
        fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
    let input2 =
        fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");

    println!("Part 1 test 1: {}", part1(&input1));
    println!("Part 1 test 2: {}", part1(&input2));

    println!("Part 2 test 2: {}", part2(&input2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 12);
    }

    #[test]
    fn test2_part1() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 217132650);
    }

    #[test]
    fn test2_part2() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 6516);
    }
}
