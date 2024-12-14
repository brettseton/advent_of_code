use std::fs;

fn main() {
  let ans = part1("input/test1.txt");
  println!("part 1 test 1 answer: {}", ans);

  let ans = part1("input/test2.txt");
  println!("part 1 test 2 answer: {}", ans);

  let ans = part2("input/test3.txt");
  println!("part 2 test 1 answer: {}", ans);

  let ans = part2("input/test2.txt");
  println!("part 2 test 2 answer: {}", ans);
}

fn part1(file_path: &str) -> u32 {
  let input = fs::read_to_string(file_path).expect("Unable to read the input file");
  let mut sum = 0;

  input.lines().for_each(|line| {
    let c1 = line.chars().find(|ch| ch.is_ascii_digit());
    let c2 = line.chars().rev().find(|ch| ch.is_ascii_digit());
    
    let d1 = match c1 {
        Some(c)=> c.to_digit(10).unwrap(),
        None => 0
    };
  
    let d2 = match c2 {
        Some(c)=> c.to_digit(10).unwrap(),
        None => 0
    };
  
    sum += d1*10 + d2;
  });

  return sum;
}

fn part2(file_path: &str) -> u32  {
  let input = fs::read_to_string(file_path).expect("Unable to read the input file");
  let mut sum = 0;
  input.lines().for_each(|line| {
      sum += get_number_from_string(line);
  });
  return sum;
}

fn check_pattern(mut iter: impl Iterator<Item = char>, pattern: &str, value: u32) -> Option<u32> {
  for expected_char in pattern.chars() {
      if let Some(ch) = iter.next() {
          if ch != expected_char {
              return None;
          }
      } else {
          return None;
      }
  }

  iter.next(); // Consume the last character of the pattern
  return Some(value);
}

fn check_pattern_rev(iter: &mut std::iter::Peekable<std::iter::Rev<std::str::Chars>>, pattern: &str, value: u32) -> Option<u32> {
  let mut iter_clone = iter.clone();
  for expected_char in pattern.chars() {
      if let Some(ch) = iter_clone.next() {
          if ch != expected_char {
              return None;
          }
      } else {
          return None;
      }
  }

  iter_clone.next(); // Consume the last character of the pattern
  return Some(value);
}

fn get_number_from_string(line: &str) -> u32 {
  let mut d1 = 0;
  let mut chars = line.chars().peekable();

  while let Some(ch) = chars.next() {
      if let Some(value) = match ch {
          'o' => check_pattern(&mut chars.clone(), "ne", 1),
          't' => check_pattern(&mut chars.clone(), "wo", 2).or_else(|| check_pattern(&mut chars.clone(), "hree", 3)),
          'f' => check_pattern(&mut chars.clone(), "our", 4).or_else(|| check_pattern(&mut chars.clone(), "ive", 5)),
          's' => check_pattern(&mut chars.clone(), "ix", 6).or_else(|| check_pattern(&mut chars.clone(), "even", 7)),
          'e' => check_pattern(&mut chars.clone(), "ight", 8),
          'n' => check_pattern(&mut chars.clone(), "ine", 9),
          _ => None,
      } {
          d1 = value;
          break;
      }

      if ch.is_ascii_digit() {
          d1 = ch.to_digit(10).expect("couldn't get number");
          break;
      }
  }

  let mut d2 = 0;
  let mut chars_rev = line.chars().rev().peekable();

  while let Some(ch) = chars_rev.next() {
      if let Some(value) = match ch {
          'e' => check_pattern_rev(&mut chars_rev, "no", 1).or_else(|| check_pattern_rev(&mut chars_rev, "erht", 3)).or_else(|| check_pattern_rev(&mut chars_rev, "vif", 5)).or_else(|| check_pattern_rev(&mut chars_rev, "nin", 9)),
          'o' => check_pattern_rev(&mut chars_rev, "wt", 2),
          'r' => check_pattern_rev(&mut chars_rev, "uof", 4),
          'x' => check_pattern_rev(&mut chars_rev, "is", 6),
          'n' => check_pattern_rev(&mut chars_rev, "eves", 7),
          't' => check_pattern_rev(&mut chars_rev, "hgie", 8),
          _ => None,
      } {
          d2 = value;
          break;
      }

      if ch.is_ascii_digit() {
          d2 = ch.to_digit(10).expect("couldn't get number");
          break;
      }
  }

  return d1 * 10 + d2;
}



#[test]
fn part1_test1() {
    let ans = part1("input/test1.txt");
    assert_eq!(ans, 142);
}

#[test]
fn part1_test2() {
  let ans = part1("input/test2.txt");
  assert_eq!(ans, 54953);
}

#[test]
fn part2_test1() {
    let ans = part2("input/test3.txt");
    assert_eq!(ans, 281);
}

#[test]
fn part2_test2() {
  let ans = part2("input/test2.txt");
  assert_eq!(ans, 53868);
}