const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");
const IMPOSSIBLE_COST: usize = 1_000_000;

#[derive(Debug, Clone)]
struct Machine {
    target: Vec<bool>,
    buttons: Vec<Vec<usize>>,
}

#[derive(Debug, Clone)]
struct JoltageMachine {
    buttons: Vec<Vec<usize>>,
    target: Vec<usize>,
}

trait Parser<T> {
    fn parse(&self, line: &str) -> T;
}

struct MachineParser;

impl Parser<Machine> for MachineParser {
    fn parse(&self, line: &str) -> Machine {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let target_str = parts[0].trim_matches(|c| c == '[' || c == ']');
        let target: Vec<bool> = target_str.chars().map(|c| c == '#').collect();

        let buttons: Vec<Vec<usize>> = parts[1..parts.len() - 1]
            .iter()
            .map(|s| {
                s.trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect();

        Machine { target, buttons }
    }
}

struct JoltageMachineParser;

impl Parser<JoltageMachine> for JoltageMachineParser {
    fn parse(&self, line: &str) -> JoltageMachine {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let buttons: Vec<Vec<usize>> = parts[1..parts.len() - 1]
            .iter()
            .map(|s| {
                s.trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect();

        let joltage_str = parts.last().unwrap().trim_matches(|c| c == '{' || c == '}');
        let target: Vec<usize> = joltage_str.split(',').map(|s| s.parse().unwrap()).collect();

        JoltageMachine { buttons, target }
    }
}

trait MachineSolver {
    fn solve(&self, machine: &Machine) -> usize;
}

trait JoltageSolver {
    fn solve(&self, machine: &JoltageMachine) -> usize;
}

struct BruteForceSolver;

impl MachineSolver for BruteForceSolver {
    fn solve(&self, machine: &Machine) -> usize {
        let num_buttons = machine.buttons.len();
        let num_lights = machine.target.len();

        let mut min_presses = usize::MAX;

        for mask in 0u32..(1 << num_buttons) {
            let mut lights = vec![false; num_lights];

            for (i, button) in machine.buttons.iter().enumerate() {
                if (mask & (1 << i)) != 0 {
                    for &light_idx in button {
                        if light_idx < num_lights {
                            lights[light_idx] = !lights[light_idx];
                        }
                    }
                }
            }

            if lights == machine.target {
                let presses = mask.count_ones() as usize;
                min_presses = min_presses.min(presses);
            }
        }

        min_presses
    }
}

struct LinearSolver {
    memo: std::collections::HashMap<Vec<usize>, usize>,
    buttons: Vec<Vec<usize>>,
}

impl LinearSolver {
    fn new(buttons: Vec<Vec<usize>>) -> Self {
        Self {
            memo: std::collections::HashMap::new(),
            buttons,
        }
    }

    fn solve_recursive(&mut self, joltages: &[usize]) -> usize {
        if joltages.iter().all(|&x| x == 0) {
            return 0;
        }

        if joltages.iter().any(|&x| x > IMPOSSIBLE_COST) {
            return IMPOSSIBLE_COST;
        }

        let key = joltages.to_vec();
        if let Some(&result) = self.memo.get(&key) {
            return result;
        }

        let num_buttons = self.buttons.len();
        let num_joltages = joltages.len();
        let mut min_presses = IMPOSSIBLE_COST;

        let target_parity: Vec<bool> = joltages.iter().map(|&x| x % 2 == 1).collect();

        for mask in 0u32..(1 << num_buttons) {
            let mut parity = vec![false; num_joltages];

            for i in 0..num_buttons {
                if (mask & (1 << i)) != 0 {
                    for &joltage_idx in &self.buttons[i] {
                        if joltage_idx < num_joltages {
                            parity[joltage_idx] = !parity[joltage_idx];
                        }
                    }
                }
            }

            if parity == target_parity {
                let mut presses = vec![0; num_joltages];
                for i in 0..num_buttons {
                    if (mask & (1 << i)) != 0 {
                        for &joltage_idx in &self.buttons[i] {
                            if joltage_idx < num_joltages {
                                presses[joltage_idx] += 1;
                            }
                        }
                    }
                }

                let mut remaining = vec![0; num_joltages];
                let mut valid = true;
                for i in 0..num_joltages {
                    if presses[i] > joltages[i] {
                        valid = false;
                        break;
                    }
                    remaining[i] = joltages[i] - presses[i];
                }

                if valid && remaining.iter().all(|&x| x % 2 == 0) {
                    let halved: Vec<usize> = remaining.iter().map(|&x| x / 2).collect();
                    let recursive_cost = self.solve_recursive(&halved);
                    if recursive_cost < IMPOSSIBLE_COST {
                        let button_presses = mask.count_ones() as usize;
                        let total_cost = button_presses + 2 * recursive_cost;
                        min_presses = min_presses.min(total_cost);
                    }
                }
            }
        }

        self.memo.insert(key, min_presses);
        min_presses
    }
}

impl JoltageSolver for LinearSolver {
    fn solve(&self, machine: &JoltageMachine) -> usize {
        let mut solver = LinearSolver::new(machine.buttons.clone());
        solver.solve_recursive(&machine.target)
    }
}

fn part1(input: &str) -> i32 {
    let parser = MachineParser;
    let solver = BruteForceSolver;

    input
        .lines()
        .map(|line| parser.parse(line))
        .map(|machine| solver.solve(&machine))
        .sum::<usize>() as i32
}

fn part2(input: &str) -> i32 {
    let parser = JoltageMachineParser;
    let solver = LinearSolver::new(vec![]);

    input
        .lines()
        .map(|line| parser.parse(line))
        .map(|machine| solver.solve(&machine))
        .sum::<usize>() as i32
}

fn main() {
    println!("Part 1 test 1: {}", part1(TEST_INPUT_1));
    println!("Part 1 test 2: {}", part1(TEST_INPUT_2));

    println!("Part 2 test 1: {}", part2(TEST_INPUT_1));
    println!("Part 2 test 2: {}", part2(TEST_INPUT_2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() {
        assert_eq!(part1(TEST_INPUT_1), 7);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 545);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 33);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 22430);
    }
}
