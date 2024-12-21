use std::fs;

fn parse_input(input: &str) -> Computer {
    let register_a = input
        .lines()
        .nth(0)
        .unwrap()
        .split(" ")
        .nth(2)
        .unwrap()
        .parse()
        .unwrap();

    let register_b = input
        .lines()
        .nth(1)
        .unwrap()
        .split(" ")
        .nth(2)
        .unwrap()
        .parse()
        .unwrap();

    let register_c = input
        .lines()
        .nth(2)
        .unwrap()
        .split(" ")
        .nth(2)
        .unwrap()
        .parse()
        .unwrap();

    let program = input
        .lines()
        .nth(4)
        .unwrap()
        .split(" ")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    Computer {
        register_a,
        register_b,
        register_c,
        program: program,
    }
}

#[derive(Clone)]
struct Computer {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    program: Vec<u64>,
}

impl Computer {
    fn run_program(&mut self) -> Vec<u64> {
        let mut output = Vec::new();
        let mut instruction_pointer = 0;

        while instruction_pointer < self.program.len() as u64 {
            let opcode = self.program[instruction_pointer as usize];
            let literal_operand = self.program[instruction_pointer as usize + 1];
            let combo_operand = match literal_operand {
                0..=3 => literal_operand,
                4 => self.register_a,
                5 => self.register_b,
                6 => self.register_c,
                _ => panic!("Invalid opcode: {}", opcode),
            };

            match opcode {
                0 => {
                    // adv
                    let denominator = 2_u64.pow(combo_operand as u32);
                    self.register_a /= denominator; // A
                    instruction_pointer += 2;
                }
                1 => {
                    // bxl
                    self.register_b ^= literal_operand; // B
                    instruction_pointer += 2;
                }
                2 => {
                    // bst
                    self.register_b = combo_operand % 8; // B
                    instruction_pointer += 2;
                }
                3 => {
                    // jnz
                    if self.register_a != 0 {
                        instruction_pointer = literal_operand; // Jump
                    } else {
                        instruction_pointer += 2;
                    }
                }
                4 => {
                    // bxc
                    self.register_b ^= self.register_c; // B
                    instruction_pointer += 2;
                }
                5 => {
                    // out
                    output.push(combo_operand % 8); // Output
                    instruction_pointer += 2;
                }
                6 => {
                    // bdv
                    let denominator = 2_u64.pow(combo_operand as u32);
                    self.register_b = self.register_a / denominator; // B
                    instruction_pointer += 2;
                }
                7 => {
                    // cdv
                    let denominator = 2_u64.pow(combo_operand as u32);
                    self.register_c = self.register_a / denominator; // C
                    instruction_pointer += 2;
                }
                _ => panic!("Invalid opcode: {}", opcode), // Halt on invalid opcode
            }
        }

        output
    }
}

fn part1(input: &str) -> String {
    let mut computer = parse_input(input);

    computer
        .run_program()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn part2(input: &str) -> u64 {
    let computer = parse_input(input);
    let mut x: u64;

    let mut new_a = vec![0; 1];
    while new_a.len() <= computer.program.len() {
        x = 0;
        for j in &new_a {
            x <<= 3;
            x += j;
        }

        let mut altered_computer = computer.clone();
        altered_computer.register_a = x;

        let out = altered_computer.run_program();
        // Check if all the elements of the output is the same as the computer.program
        if out.len() == new_a.len()
            && out[0] == computer.program[computer.program.len() - out.len()]
        {
            new_a.push(0);
        } else {
            let mut last_value = new_a.last_mut().unwrap();
            *last_value += 1;
            while *last_value == 8 {
                new_a.pop();
                if let Some(new_last) = new_a.last_mut() {
                    *new_last += 1;
                    last_value = new_last;
                } else {
                    break;
                }
            }
        }

        let mut altered_computer = computer.clone();
        altered_computer.register_a = x;

        let out = altered_computer.run_program();
        // Check if all the elements of the output is the same as the computer.program
        if computer.program.len() == out.len()
            && out
                .iter()
                .enumerate()
                .all(|(i, x)| computer.program[i] == *x)
        {
            return x;
        }
    }

    return 0;
}

fn main() {
    let input1 =
        fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
    let input2 =
        fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
    let input3 =
        fs::read_to_string("input/test3.txt").expect("Should have been able to read the file");
    let input4 =
        fs::read_to_string("input/test4.txt").expect("Should have been able to read the file");
    let input5 =
        fs::read_to_string("input/test5.txt").expect("Should have been able to read the file");

    println!("Part 1 test 1: {}", part1(&input1));
    println!("Part 1 test 2: {}", part1(&input2));
    println!("Part 1 test 2: {}", part1(&input3));
    println!("Part 1 test 2: {}", part1(&input4));

    println!("Part 2 test 5: {}", part2(&input5));
    println!("Part 2 test 2: {}", part2(&input2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test2_part1() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), "6,2,7,2,3,1,6,0,5");
    }

    #[test]
    fn test3_part1() {
        let test_input =
            fs::read_to_string("input/test3.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), "0,1,2");
    }

    #[test]
    fn test4_part1() {
        let test_input =
            fs::read_to_string("input/test4.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), "4,2,5,6,7,7,7,7,3,1,0");
    }

    #[test]
    fn test5_part2() {
        let test_input =
            fs::read_to_string("input/test5.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 117440);
    }

    #[test]
    fn test2_part2() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 236548287712877);
    }
}
