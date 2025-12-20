const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

const SCREEN_WIDTH: i32 = 40;
const SCREEN_HEIGHT: i32 = 6;
const SPRITE_RADIUS: i32 = 1;

const FIRST_SIGNAL_CYCLE: i32 = 20;
const SIGNAL_CYCLE_STEP: i32 = 40;
const MAX_SIGNAL_CYCLE: i32 = 220;

use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        match parts.next() {
            Some("noop") => Ok(Instruction::Noop),
            Some("addx") => {
                let v = parts
                    .next()
                    .and_then(|p| p.parse().ok())
                    .ok_or("Invalid addx value")?;
                Ok(Instruction::Addx(v))
            }
            _ => Err("Unknown instruction"),
        }
    }
}

struct Cpu<'a> {
    input: &'a str,
}

impl<'a> Cpu<'a> {
    fn new(input: &'a str) -> Self {
        Self { input }
    }

    fn get_register_history(&self) -> Vec<i32> {
        let mut x = 1;
        let mut history = Vec::with_capacity(240);

        for line in self.input.lines() {
            if let Ok(ins) = line.trim().parse::<Instruction>() {
                match ins {
                    Instruction::Noop => {
                        history.push(x);
                    }
                    Instruction::Addx(v) => {
                        history.push(x);
                        history.push(x);
                        x += v;
                    }
                }
            }
        }
        history
    }
}

struct Screen;

impl Screen {
    fn render(history: &[i32]) -> String {
        let capacity = (SCREEN_WIDTH * SCREEN_HEIGHT + SCREEN_HEIGHT) as usize;
        let mut output = String::with_capacity(capacity);

        for (i, &x) in history.iter().enumerate() {
            let pos = (i as i32) % SCREEN_WIDTH;

            let is_sprite_visible = (pos - x).abs() <= SPRITE_RADIUS;
            output.push(if is_sprite_visible { '#' } else { '.' });

            if pos == SCREEN_WIDTH - 1 {
                output.push('\n');
            }

            if i as i32 + 1 >= SCREEN_WIDTH * SCREEN_HEIGHT {
                break;
            }
        }
        output
    }
}

fn part1(input: &str) -> i32 {
    Cpu::new(input)
        .get_register_history()
        .iter()
        .enumerate()
        .map(|(i, &x)| (i as i32 + 1, x))
        .filter(|(cycle, _)| {
            (*cycle - FIRST_SIGNAL_CYCLE) % SIGNAL_CYCLE_STEP == 0 && *cycle <= MAX_SIGNAL_CYCLE
        })
        .map(|(cycle, x)| cycle * x)
        .sum()
}

fn part2(input: &str) -> String {
    let history = Cpu::new(input).get_register_history();
    Screen::render(&history)
}

fn main() {
    println!("Part 1 test 1: {}", part1(TEST_INPUT_1));
    println!("Part 1 test 2: {}", part1(TEST_INPUT_2));

    println!("Part 2 test 1:\n{}", part2(TEST_INPUT_1));
    println!("Part 2 test 2:\n{}", part2(TEST_INPUT_2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() {
        assert_eq!(part1(TEST_INPUT_1), 13140);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 12880);
    }

    #[test]
    fn test1_part2() {
        let expected = [
            "##..##..##..##..##..##..##..##..##..##..",
            "###...###...###...###...###...###...###.",
            "####....####....####....####....####....",
            "#####.....#####.....#####.....#####.....",
            "######......######......######......####",
            "#######.......#######.......#######.....",
            "",
        ]
        .join("\n");
        assert_eq!(part2(TEST_INPUT_1), expected);
    }

    #[test]
    fn test2_part2() {
        //FCJAPJRE
        let expected = [
            "####..##....##..##..###....##.###..####.",
            "#....#..#....#.#..#.#..#....#.#..#.#....",
            "###..#.......#.#..#.#..#....#.#..#.###..",
            "#....#.......#.####.###.....#.###..#....",
            "#....#..#.#..#.#..#.#....#..#.#.#..#....",
            "#.....##...##..#..#.#.....##..#..#.####.",
            "",
        ]
        .join("\n");
        assert_eq!(part2(TEST_INPUT_2), expected);
    }
}
