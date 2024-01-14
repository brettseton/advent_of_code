use std::{fs, str::FromStr};

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
    let sequence = Sequence::new(&input);
    return sequence.get_hashes();
}

fn part2(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let sequence = Sequence::new(&input);
    return sequence.get_box_score();
}

struct Sequence {
    calcs: Vec<String>,
}

#[derive(Clone)]
struct Lens {
    label: String,
    focal_length: usize
}

impl Sequence {
    pub fn new(str: &str) -> Sequence {
        return Sequence::from_str(str).expect("");
    }

    fn get_hashes(&self) -> usize {
        self.calcs.iter().map(|calc| Sequence::get_hash(calc)).sum()
    }

    fn get_box_score(&self) -> usize {
        let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
        for calc in self.calcs.iter() {
            if calc.contains('-') {
                if let Some((label, _operation)) = calc.split_once('-') {
                    let hash = Sequence::get_hash(label);
                    
                    match boxes[hash].iter().enumerate().find(|(_i, x)| x.label == label) {
                        Some((index, _)) => {
                            boxes[hash].remove(index);
                        },
                        None => continue,
                    }
                }
            } else {
                if let Some((label, focal_length_str)) = calc.split_once('=') {
                    let hash = Sequence::get_hash(label);
                    let lens = Lens { label: label.to_string(), focal_length: focal_length_str.parse::<usize>().unwrap() };
                    match boxes[hash].iter().enumerate().find(|(_i, x)| x.label == label) {
                        Some((index, _)) => {
                            boxes[hash][index].focal_length = lens.focal_length;
                        },
                        None => { boxes[hash].push(lens); },
                    }
                }
            }
        }

        return boxes.iter().enumerate().map(|(box_id, x)| x.iter().enumerate().map(|(slot_id, lens)| (box_id + 1) * (slot_id + 1) * lens.focal_length).sum::<usize>()).sum();
    }

    fn get_hash(s: &str) -> usize {
        let mut sum = 0;

        for c in s.chars() {
            sum = (sum + c as usize) * 17 % 256;
        }

        return sum;
    }
}

#[derive(Debug)]
struct SequenceError;

impl FromStr for Sequence {
    type Err = SequenceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let calcs = s.split(',').map(String::from).collect::<Vec<String>>();
        return Ok(Sequence { calcs });
    }
}

#[test]
pub fn part1_test1() {
    let ans = part1("input/test1.txt");
    assert_eq!(ans, 1320);
}

#[test]
pub fn part1_test2() {
    let ans = part1("input/test2.txt");
    assert_eq!(ans, 512283);
}

#[test]
pub fn part2_test1() {
    let ans = part2("input/test1.txt");
    assert_eq!(ans, 145);
}

#[test]
pub fn part2_test2() {
    let ans = part2("input/test2.txt");
    assert_eq!(ans, 215827);
}
