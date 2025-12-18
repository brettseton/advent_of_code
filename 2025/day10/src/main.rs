const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");
const IMPOSSIBLE_COST: usize = 1_000_000;
use std::str::FromStr;

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

impl Machine {
    fn parse_buttons(tokens: &[&str]) -> Vec<Vec<usize>> {
        tokens
            .iter()
            .map(|s| {
                s.trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect()
    }
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let target_str = parts[0].trim_matches(|c| c == '[' || c == ']');
        let target: Vec<bool> = target_str.chars().map(|c| c == '#').collect();
        let buttons = Self::parse_buttons(&parts[1..parts.len() - 1]);

        Ok(Machine { target, buttons })
    }
}

impl FromStr for JoltageMachine {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let buttons = Machine::parse_buttons(&parts[1..parts.len() - 1]);
        let joltage_str = parts.last().unwrap().trim_matches(|c| c == '{' || c == '}');
        let target: Vec<usize> = joltage_str.split(',').map(|s| s.parse().unwrap()).collect();

        Ok(JoltageMachine { buttons, target })
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
                if is_bit_set(mask, i) {
                    toggle_lights(&mut lights, button);
                }
            }

            if lights == machine.target {
                min_presses = min_presses.min(mask.count_ones() as usize);
            }
        }

        min_presses
    }
}

fn is_bit_set<T: Into<u64>>(mask: T, index: usize) -> bool {
    (mask.into() >> index) & 1 == 1
}

fn toggle_lights(lights: &mut [bool], button: &[usize]) {
    for &light_idx in button {
        if light_idx < lights.len() {
            lights[light_idx] = !lights[light_idx];
        }
    }
}

struct GF2Solver {
    t_rows: Vec<u64>,
    pivots: Vec<Option<usize>>,
    kernel_basis: Vec<u64>,
    num_rows: usize,
}

impl GF2Solver {
    fn new(buttons: &[Vec<usize>], num_rows: usize) -> Self {
        assert!(num_rows <= 64, "Too many joltages for u64 bitmask");
        let num_buttons = buttons.len();
        let mut r_rows = Self::build_button_matrix(buttons, num_rows);
        let mut t_rows = Self::build_identity_matrix(num_rows);
        let pivots = Self::gaussian_elimination(&mut r_rows, &mut t_rows, num_buttons);
        let kernel_basis = Self::compute_kernel_basis(&r_rows, &pivots, num_buttons, num_rows);

        Self {
            t_rows,
            pivots,
            kernel_basis,
            num_rows,
        }
    }

    fn build_button_matrix(buttons: &[Vec<usize>], num_rows: usize) -> Vec<u64> {
        let mut matrix = vec![0u64; num_rows];
        for (j, indices) in buttons.iter().enumerate() {
            for &i in indices {
                if i < num_rows {
                    matrix[i] |= 1 << j;
                }
            }
        }
        matrix
    }

    fn build_identity_matrix(num_rows: usize) -> Vec<u64> {
        let mut matrix = vec![0u64; num_rows];
        for (i, row) in matrix.iter_mut().enumerate() {
            *row = 1 << i;
        }
        matrix
    }

    fn gaussian_elimination(
        r_rows: &mut [u64],
        t_rows: &mut [u64],
        num_buttons: usize,
    ) -> Vec<Option<usize>> {
        let num_rows = r_rows.len();
        let mut pivots = vec![None; num_rows];
        let mut next_row = 0;

        for j in 0..num_buttons {
            if let Some(pivot_row) = Self::find_pivot_row(r_rows, next_row, j) {
                r_rows.swap(next_row, pivot_row);
                t_rows.swap(next_row, pivot_row);
                Self::eliminate_column(r_rows, t_rows, next_row, j);
                pivots[next_row] = Some(j);
                next_row += 1;
            }
        }

        pivots
    }

    fn find_pivot_row(r_rows: &[u64], start_row: usize, column: usize) -> Option<usize> {
        r_rows
            .iter()
            .enumerate()
            .skip(start_row)
            .find(|(_, row)| is_bit_set(**row, column))
            .map(|(i, _)| i)
    }

    fn eliminate_column(r_rows: &mut [u64], t_rows: &mut [u64], pivot_row: usize, column: usize) {
        let num_rows = r_rows.len();
        for i in 0..num_rows {
            if i != pivot_row && is_bit_set(r_rows[i], column) {
                r_rows[i] ^= r_rows[pivot_row];
                t_rows[i] ^= t_rows[pivot_row];
            }
        }
    }

    fn compute_kernel_basis(
        r_rows: &[u64],
        pivots: &[Option<usize>],
        num_buttons: usize,
        num_rows: usize,
    ) -> Vec<u64> {
        let is_pivot = Self::mark_pivot_columns(pivots, num_buttons);
        let mut kernel_basis = Vec::new();

        for (f, &pivot) in is_pivot.iter().enumerate() {
            if !pivot {
                let mut v = 1 << f;
                for i in 0..num_rows {
                    if let Some(p) = pivots[i] {
                        if is_bit_set(r_rows[i], f) {
                            v |= 1 << p;
                        }
                    }
                }
                kernel_basis.push(v);
            }
        }

        kernel_basis
    }

    fn mark_pivot_columns(pivots: &[Option<usize>], num_buttons: usize) -> Vec<bool> {
        let mut is_pivot = vec![false; num_buttons];
        for p in pivots.iter().flatten() {
            is_pivot[*p] = true;
        }
        is_pivot
    }

    fn for_each_solution<F>(&self, target_parity: u64, mut f: F) -> bool
    where
        F: FnMut(u64),
    {
        let b_prime = self.compute_transformed_target(target_parity);

        if !self.is_system_consistent(b_prime) {
            return false;
        }

        let x_p = self.find_particular_solution(b_prime);
        let num_k = self.kernel_basis.len();

        for mask in 0u64..(1 << num_k) {
            let mut x = x_p;
            for (i, &basis) in self.kernel_basis.iter().enumerate() {
                if is_bit_set(mask, i) {
                    x ^= basis;
                }
            }
            f(x);
        }
        true
    }

    fn compute_transformed_target(&self, target_parity: u64) -> u64 {
        let mut b_prime = 0u64;
        for i in 0..self.num_rows {
            if (self.t_rows[i] & target_parity).count_ones() % 2 == 1 {
                b_prime |= 1 << i;
            }
        }
        b_prime
    }

    fn is_system_consistent(&self, b_prime: u64) -> bool {
        (0..self.num_rows).all(|i| self.pivots[i].is_some() || !is_bit_set(b_prime, i))
    }

    fn find_particular_solution(&self, b_prime: u64) -> u64 {
        let mut x_p = 0u64;
        for i in 0..self.num_rows {
            if let Some(p) = self.pivots[i] {
                if is_bit_set(b_prime, i) {
                    x_p |= 1 << p;
                }
            }
        }
        x_p
    }
}

struct LinearSolver {
    memo: std::collections::HashMap<Vec<usize>, usize>,
    row_map: Vec<Vec<usize>>,
    gf2_solver: Option<std::rc::Rc<GF2Solver>>,
}

impl LinearSolver {
    fn new(buttons: Vec<Vec<usize>>, num_joltages: usize) -> Self {
        let gf2_solver = if num_joltages > 0 {
            Some(std::rc::Rc::new(GF2Solver::new(&buttons, num_joltages)))
        } else {
            None
        };

        let mut row_map = vec![vec![]; num_joltages];
        for (btn_idx, indices) in buttons.iter().enumerate() {
            for &row_idx in indices {
                if row_idx < num_joltages {
                    row_map[row_idx].push(btn_idx);
                }
            }
        }

        Self {
            memo: std::collections::HashMap::new(),
            row_map,
            gf2_solver,
        }
    }

    fn solve_recursive(&mut self, joltages: &[usize]) -> usize {
        if self.is_solved(joltages) {
            return 0;
        }

        if self.is_impossible(joltages) {
            return IMPOSSIBLE_COST;
        }

        if let Some(&result) = self.memo.get(joltages) {
            return result;
        }

        let min_presses = self.compute_min_presses(joltages);
        self.memo.insert(joltages.to_vec(), min_presses);
        min_presses
    }

    fn is_solved(&self, joltages: &[usize]) -> bool {
        joltages.iter().all(|&x| x == 0)
    }

    fn is_impossible(&self, joltages: &[usize]) -> bool {
        joltages.iter().any(|&x| x > IMPOSSIBLE_COST)
    }

    fn compute_min_presses(&mut self, joltages: &[usize]) -> usize {
        let target_parity = self.joltages_to_parity(joltages);
        let mut min_total = IMPOSSIBLE_COST;

        let solver = self
            .gf2_solver
            .as_ref()
            .expect("Solver should be initialized")
            .clone();

        let found = solver.for_each_solution(target_parity, |mask| {
            if let Some(cost) = self.evaluate_solution(mask, joltages) {
                if cost < min_total {
                    min_total = cost;
                }
            }
        });

        if !found {
            return IMPOSSIBLE_COST;
        }

        min_total
    }

    fn joltages_to_parity(&self, joltages: &[usize]) -> u64 {
        let mut parity = 0u64;
        for (i, &val) in joltages.iter().enumerate() {
            if val % 2 == 1 {
                parity |= 1 << i;
            }
        }
        parity
    }

    fn evaluate_solution(&mut self, mask: u64, joltages: &[usize]) -> Option<usize> {
        let next_joltages = self.compute_next_state(mask, joltages)?;
        let recursive_cost = self.solve_recursive(&next_joltages);

        if recursive_cost < IMPOSSIBLE_COST {
            let button_presses = mask.count_ones() as usize;
            Some(button_presses + 2 * recursive_cost)
        } else {
            None
        }
    }

    fn compute_next_state(&self, mask: u64, joltages: &[usize]) -> Option<Vec<usize>> {
        let mut next_joltages = Vec::with_capacity(joltages.len());
        for (i, &joltage) in joltages.iter().enumerate() {
            let mut press_count = 0;
            for &btn_idx in &self.row_map[i] {
                if is_bit_set(mask, btn_idx) {
                    press_count += 1;
                }
            }

            if press_count > joltage {
                return None;
            }
            let rem = joltage - press_count;
            if rem % 2 != 0 {
                return None;
            }
            next_joltages.push(rem / 2);
        }
        Some(next_joltages)
    }
}

impl JoltageSolver for LinearSolver {
    fn solve(&self, machine: &JoltageMachine) -> usize {
        let mut solver = LinearSolver::new(machine.buttons.clone(), machine.target.len());
        solver.solve_recursive(&machine.target)
    }
}

fn solve_puzzle<T, S>(input: &str, solver: S) -> i32
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
    S: for<'a> Fn(&'a T) -> usize,
{
    input
        .lines()
        .map(|line| line.parse::<T>().unwrap())
        .map(|machine| solver(&machine))
        .sum::<usize>() as i32
}

fn part1(input: &str) -> i32 {
    solve_puzzle(input, |machine| BruteForceSolver.solve(machine))
}

fn part2(input: &str) -> i32 {
    solve_puzzle(input, |machine| LinearSolver::new(vec![], 0).solve(machine))
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
