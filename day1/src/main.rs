use std::io::{self, BufRead};
use std::io::Write;

fn main() {
  day1(&mut io::stdin().lock(), &mut io::stdout());
}

fn day1(input: &mut impl BufRead,
    output: &mut impl Write,) {
  let mut line = String::new();
  let mut sum = 0;
  loop {
    match input.read_line(&mut line) {
      Ok(0) => break, // EOF
      Ok(_) => {
        let c1 = line.chars().find(|ch| ch.is_digit(10));
        let c2 = line.chars().rev().find(|ch| ch.is_digit(10));
        
        let d1 = match c1 {
            Some(c)=> c.to_digit(10).unwrap(),
            None => 0
        };

        let d2 = match c2 {
            Some(c)=> c.to_digit(10).unwrap(),
            None => 0
        };

        sum += d1*10 + d2;
        line.clear();
      },
      Err(e) => {
        println!("Error: {}", e);
        break;
      },
    }
  }
  write!(output, "{}", sum).expect("Unable to write");
}

#[test]
fn writes_upcased_input_to_output() {
    let mut output: Vec<u8> = Vec::new();
    day1(&mut "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet".as_bytes(), &mut output);
    assert_eq!(&output, b"142");
}