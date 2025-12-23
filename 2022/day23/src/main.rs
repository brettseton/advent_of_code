const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

const OFFSET: i32 = 1100;
const GRID_SIZE: usize = 2500;

struct Proposal {
    target_idx: usize,
}

#[derive(Clone, Copy, PartialEq)]
enum ProposalState {
    None,
    Conflict,
    Source(u32),
}

struct Solver {
    elves: Vec<(i32, i32)>,
    grid: Vec<u8>,
    proposals_grid: Vec<ProposalState>,
    successful_proposals: Vec<Proposal>,
    elf_proposals: Vec<Option<usize>>, // Stores target_idx for each elf
}

impl Solver {
    fn new(input: &str) -> Self {
        let mut elves = Vec::new();
        let mut grid = vec![0u8; GRID_SIZE * GRID_SIZE];

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    let px = x as i32 + OFFSET;
                    let py = y as i32 + OFFSET;
                    assert!(
                        (px as usize) < GRID_SIZE && (py as usize) < GRID_SIZE,
                        "Input exceeds grid bounds"
                    );

                    elves.push((px, py));
                    grid[py as usize * GRID_SIZE + px as usize] = 1;
                }
            }
        }

        let elf_count = elves.len();
        Self {
            elves,
            grid,
            proposals_grid: vec![ProposalState::None; GRID_SIZE * GRID_SIZE],
            successful_proposals: Vec::with_capacity(elf_count),
            elf_proposals: vec![None; elf_count],
        }
    }

    #[inline(always)]
    fn has_elf(&self, x: i32, y: i32) -> bool {
        self.grid[y as usize * GRID_SIZE + x as usize] == 1
    }

    #[inline(always)]
    fn get_neighbor_mask(&self, x: i32, y: i32) -> u8 {
        let mut mask = 0u8;
        // [NW, N, NE, W, E, SW, S, SE]
        if self.has_elf(x - 1, y - 1) {
            mask |= 1 << 7;
        }
        if self.has_elf(x, y - 1) {
            mask |= 1 << 6;
        }
        if self.has_elf(x + 1, y - 1) {
            mask |= 1 << 5;
        }
        if self.has_elf(x - 1, y) {
            mask |= 1 << 4;
        }
        if self.has_elf(x + 1, y) {
            mask |= 1 << 3;
        }
        if self.has_elf(x - 1, y + 1) {
            mask |= 1 << 2;
        }
        if self.has_elf(x, y + 1) {
            mask |= 1 << 1;
        }
        if self.has_elf(x + 1, y + 1) {
            mask |= 1 << 0;
        }
        mask
    }

    fn simulate_round(&mut self, round: usize) -> bool {
        self.collect_proposals(round);
        let moved = self.apply_moves();
        self.reset_proposals();
        moved
    }

    fn collect_proposals(&mut self, round: usize) {
        self.successful_proposals.clear();
        let directions = ['N', 'S', 'W', 'E'];
        let start_dir = round % 4;

        for (idx, &(x, y)) in self.elves.iter().enumerate() {
            self.elf_proposals[idx] = None;
            let mask = self.get_neighbor_mask(x, y);

            if mask == 0 {
                continue;
            }

            for i in 0..4 {
                let dir = directions[(start_dir + i) % 4];
                let (is_valid, tx, ty) = match dir {
                    // All N bits NW, N, NE
                    'N' => (mask & 0b11100000 == 0, x, y - 1),
                    // All S bits SW, S, SE
                    'S' => (mask & 0b00000111 == 0, x, y + 1),
                    // All W bits NW, W, SW
                    'W' => (mask & 0b10010100 == 0, x - 1, y),
                    // All E bits NE, E, SE
                    'E' => (mask & 0b00101001 == 0, x + 1, y),
                    _ => unreachable!(),
                };

                if is_valid {
                    let tidx = ty as usize * GRID_SIZE + tx as usize;
                    match self.proposals_grid[tidx] {
                        ProposalState::None => {
                            self.proposals_grid[tidx] = ProposalState::Source(idx as u32);
                            self.successful_proposals
                                .push(Proposal { target_idx: tidx });
                        }
                        ProposalState::Source(_) => {
                            self.proposals_grid[tidx] = ProposalState::Conflict;
                        }
                        ProposalState::Conflict => {}
                    }
                    self.elf_proposals[idx] = Some(tidx);
                    break;
                }
            }
        }
    }

    fn apply_moves(&mut self) -> bool {
        let mut moved = false;
        for (idx, (ox, oy)) in self.elves.iter_mut().enumerate() {
            if let Some(tidx) = self.elf_proposals[idx] {
                if self.proposals_grid[tidx] == ProposalState::Source(idx as u32) {
                    // Update grid
                    self.grid[*oy as usize * GRID_SIZE + *ox as usize] = 0;
                    self.grid[tidx] = 1;

                    // Update Elf position
                    *ox = (tidx % GRID_SIZE) as i32;
                    *oy = (tidx / GRID_SIZE) as i32;
                    moved = true;
                }
            }
        }
        moved
    }

    fn reset_proposals(&mut self) {
        for prop in &self.successful_proposals {
            self.proposals_grid[prop.target_idx] = ProposalState::None;
        }
    }
}

fn part1(input: &str) -> i32 {
    let mut solver = Solver::new(input);
    for round in 0..10 {
        solver.simulate_round(round);
    }

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for &(x, y) in &solver.elves {
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    (width * height) - solver.elves.len() as i32
}

fn part2(input: &str) -> i32 {
    let mut solver = Solver::new(input);
    let mut round = 0;
    while solver.simulate_round(round) {
        round += 1;
    }
    round as i32 + 1
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
        assert_eq!(part1(TEST_INPUT_1), 110);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 3689);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 20);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 965);
    }
}
