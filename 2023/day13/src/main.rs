use std::{cmp::Ordering, fs, str::FromStr};

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
    let puzzle = Puzzle::new(&input);
    return puzzle.get_mirror_location();
}

fn part2(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let puzzle = Puzzle::new(&input);
    return puzzle.get_mirror_location_with_smudge();
}

struct Puzzle {
    rows: Vec<AshRockMap>,
}

impl Puzzle {
    pub fn new(str: &str) -> Puzzle {
        return Puzzle::from_str(str).expect("");
    }

    pub fn get_mirror_location(&self) -> usize {
        return self.rows.iter().map(|x| x.get_mirror_location(0)).sum();
    }

    pub fn get_mirror_location_with_smudge(&self) -> usize {
        let scores: Vec<usize> = self.rows.iter().map(|x| x.get_mirror_location(1)).collect();
        return scores.iter().sum();
    }
}

#[derive(Debug)]
struct PuzzleError;

impl FromStr for Puzzle {
    type Err = PuzzleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s.split("\n\n").map(AshRockMap::new).collect();
        return Ok(Puzzle { rows });
    }
}

struct AshRockMap {
    map: Vec<String>,
}

impl AshRockMap {
    pub fn new(str: &str) -> AshRockMap {
        return AshRockMap::from_str(str).expect("");
    }

    pub fn get_mirror_location(&self, allowed_differences: usize) -> usize {
        // Horizontal mirror
        if let Some(match_index) =
            AshRockMap::get_mirror_location_from_lines(&self.map, allowed_differences)
        {
            return match_index * 100;
        }

        let map_width = self.map.first().unwrap().len();
        let map_height = self.map.len();
        let vertical_map: Vec<String> = (0..map_width)
            .map(|x| {
                (0..map_height)
                    .map(|y| {
                        self.map
                            .get(y)
                            .unwrap_or(&"".to_string())
                            .chars()
                            .nth(x)
                            .unwrap_or(' ')
                    })
                    .collect()
            })
            .collect();

        // Vertical mirror
        if let Some(match_index) =
            AshRockMap::get_mirror_location_from_lines(&vertical_map, allowed_differences)
        {
            return match_index;
        }

        return 0;
    }

    pub fn get_mirror_location_from_lines(
        lines: &[String],
        allowed_differences: usize,
    ) -> Option<usize> {
        let mut prev = lines.first().unwrap();

        for (index, line) in lines.iter().enumerate().skip(1) {
            let mut acc_differences = 0;

            let num_differences = AshRockMap::get_differences(line, prev);

            match num_differences.cmp(&allowed_differences) {
                Ordering::Greater => {
                    prev = line;
                    continue;
                }
                Ordering::Less => (),
                Ordering::Equal => {
                    acc_differences = num_differences;
                }
            }

            for i in 1..lines.len() {
                if i > index - 1 {
                    if acc_differences != allowed_differences {
                        break;
                    }
                    return Some(index);
                }
                if let Some(check_up) = lines.get(index - i - 1) {
                    if let Some(check_down) = lines.get(index + i) {
                        let num_differences = AshRockMap::get_differences(check_up, check_down);

                        if num_differences + acc_differences <= allowed_differences {
                            acc_differences += num_differences;
                            continue;
                        } else {
                            break;
                        }
                    } else {
                        if acc_differences != allowed_differences {
                            break;
                        }
                        return Some(index);
                    }
                } else {
                    if acc_differences != allowed_differences {
                        break;
                    }
                    return Some(index);
                }
            }

            prev = line;
        }

        return None;
    }

    pub fn get_differences(str1: &str, str2: &str) -> usize {
        return str1
            .chars()
            .zip(str2.chars())
            .filter(|&(c1, c2)| c1 != c2)
            .count();
    }
}

#[derive(Debug)]
struct AshRockMapError;

impl FromStr for AshRockMap {
    type Err = AshRockMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(AshRockMap {
            map: s.lines().map(|x| x.to_string()).collect::<Vec<String>>(),
        });
    }
}

#[test]
pub fn part1_test1() {
    let ans = part1("input/test1.txt");
    assert_eq!(ans, 405);
}

#[test]
pub fn part1_test2() {
    let ans = part1("input/test2.txt");
    assert_eq!(ans, 35691);
}

#[test]
pub fn part2_test1() {
    let ans = part2("input/test1.txt");
    assert_eq!(ans, 400);
}

#[test]
pub fn part2_test2() {
    let ans = part2("input/test2.txt");
    assert_eq!(ans, 39037);
}
