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
    return sequence.get_hashes();
}

struct Sequence {
    calcs: Vec<String>,
}

impl Sequence {
    pub fn new(str: &str) -> Sequence {
        return Sequence::from_str(str).expect("");
    }

    fn get_hashes(&self) -> usize {
        self.calcs.iter()
            .map(|calc| Sequence::get_hash(calc))
            .sum()
    }

    fn get_hash(s: &str) -> usize {
       let mut sum = 0;

       for c in s.chars() {
           sum += c as usize;
           sum *= 17;
           sum = sum % 256;
       }

       return sum;
    }
}


#[derive(Debug)]
struct SequenceError;

impl FromStr for Sequence {
    type Err = SequenceError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let calcs = s
        .split(',')
        .map(String::from)
        .collect::<Vec<String>>();
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
    assert_eq!(ans, 0);
}

#[test]
pub fn part2_test2() {
    let ans = part2("input/test2.txt");
    assert_eq!(ans, 0);
}
