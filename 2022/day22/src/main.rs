const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Facing {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Facing {
    fn turn_left(self) -> Self {
        match self {
            Facing::Right => Facing::Up,
            Facing::Down => Facing::Right,
            Facing::Left => Facing::Down,
            Facing::Up => Facing::Left,
        }
    }
    fn turn_right(self) -> Self {
        match self {
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
            Facing::Up => Facing::Right,
        }
    }
    fn delta(self) -> (i32, i32) {
        match self {
            Facing::Right => (0, 1),
            Facing::Down => (1, 0),
            Facing::Left => (0, -1),
            Facing::Up => (-1, 0),
        }
    }
}

enum Instruction {
    Move(i32),
    Left,
    Right,
}

impl Instruction {
    fn parse_all(input: &str) -> Vec<Self> {
        let mut result = Vec::new();
        let mut num = 0;
        for c in input.chars() {
            if let Some(digit) = c.to_digit(10) {
                num = num * 10 + digit as i32;
            } else {
                if num > 0 {
                    result.push(Instruction::Move(num));
                    num = 0;
                }
                match c {
                    'L' => result.push(Instruction::Left),
                    'R' => result.push(Instruction::Right),
                    _ => {}
                }
            }
        }
        if num > 0 {
            result.push(Instruction::Move(num));
        }
        result
    }
}

struct Board {
    grid: Vec<Vec<char>>,
    height: usize,
}

impl Board {
    fn parse(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = grid.len();
        Board { grid, height }
    }

    fn get(&self, r: usize, c: usize) -> char {
        self.grid
            .get(r)
            .and_then(|row| row.get(c))
            .copied()
            .unwrap_or(' ')
    }

    fn is_valid(&self, r: i32, c: i32) -> bool {
        if r < 0 || r >= self.height as i32 {
            return false;
        }
        let row = &self.grid[r as usize];
        if c < 0 || c >= row.len() as i32 {
            return false;
        }
        row[c as usize] != ' '
    }
}

trait WrappingStrategy {
    fn next_pos(&self, board: &Board, r: usize, c: usize, f: Facing) -> (usize, usize, Facing);
}

trait FromBoard: WrappingStrategy {
    fn from_board(board: &Board) -> Self;
}

struct FlatWrapping;
impl WrappingStrategy for FlatWrapping {
    fn next_pos(&self, board: &Board, r: usize, c: usize, f: Facing) -> (usize, usize, Facing) {
        let (dr, dc) = f.delta();
        let nr = r as i32 + dr;
        let nc = c as i32 + dc;

        if board.is_valid(nr, nc) {
            return (nr as usize, nc as usize, f);
        }

        match f {
            Facing::Right => {
                let nc = board.grid[r].iter().position(|&ch| ch != ' ').unwrap();
                (r, nc, f)
            }
            Facing::Left => {
                let nc = board.grid[r].iter().rposition(|&ch| ch != ' ').unwrap();
                (r, nc, f)
            }
            Facing::Down => {
                let nr = (0..board.height)
                    .find(|&row_idx| board.get(row_idx, c) != ' ')
                    .unwrap();
                (nr, c, f)
            }
            Facing::Up => {
                let nr = (0..board.height)
                    .rev()
                    .find(|&row_idx| board.get(row_idx, c) != ' ')
                    .unwrap();
                (nr, c, f)
            }
        }
    }
}

impl FromBoard for FlatWrapping {
    fn from_board(_: &Board) -> Self {
        FlatWrapping
    }
}

struct CubeWrapping {
    face_size: usize,
}

impl FromBoard for CubeWrapping {
    fn from_board(board: &Board) -> Self {
        let total_tiles: usize = board
            .grid
            .iter()
            .map(|row| row.iter().filter(|&&ch| ch == '.' || ch == '#').count())
            .sum();
        let face_size = ((total_tiles / 6) as f64).sqrt() as usize;
        CubeWrapping { face_size }
    }
}

impl CubeWrapping {
    fn wrap_example(
        &self,
        _board: &Board,
        r: usize,
        c: usize,
        f: Facing,
    ) -> (usize, usize, Facing) {
        let s = self.face_size;
        let face_r = r / s;
        let face_c = c / s;
        let y = r % s;
        let x = c % s;

        let (nr, nc, nf) = match (face_r, face_c, f) {
            // Face 1: (0,2)
            (0, 2, Facing::Up) => (s, s - 1 - x, Facing::Down),
            (0, 2, Facing::Left) => (s, s + y, Facing::Down),
            (0, 2, Facing::Right) => (3 * s - 1 - y, 4 * s - 1, Facing::Left),
            // Face 2: (1,0)
            (1, 0, Facing::Up) => (0, 3 * s - 1 - x, Facing::Down),
            (1, 0, Facing::Down) => (3 * s - 1, 3 * s - 1 - x, Facing::Up),
            (1, 0, Facing::Left) => (3 * s - 1, 4 * s - 1 - y, Facing::Up),
            // Face 3: (1,1)
            (1, 1, Facing::Up) => (x, 2 * s, Facing::Right),
            (1, 1, Facing::Down) => (3 * s - 1 - x, 2 * s, Facing::Right),
            // Face 4: (1,2)
            (1, 2, Facing::Right) => (2 * s, 4 * s - 1 - y, Facing::Down),
            // Face 5: (2,2)
            (2, 2, Facing::Down) => (2 * s - 1, s - 1 - x, Facing::Up),
            (2, 2, Facing::Left) => (2 * s - 1, 2 * s - 1 - y, Facing::Up),
            // Face 6: (2,3)
            (2, 3, Facing::Up) => (2 * s - 1 - x, 3 * s - 1, Facing::Left),
            (2, 3, Facing::Down) => (2 * s - 1 - x, 0, Facing::Right),
            (2, 3, Facing::Right) => (s - 1 - y, 3 * s - 1, Facing::Left),
            _ => unreachable!("Invalid wrap for example: ({}, {}, {:?})", r, c, f),
        };
        (nr, nc, nf)
    }

    fn wrap_input(&self, _board: &Board, r: usize, c: usize, f: Facing) -> (usize, usize, Facing) {
        let s = self.face_size;
        let face_r = r / s;
        let face_c = c / s;
        let y = r % s;
        let x = c % s;

        let (nr, nc, nf) = match (face_r, face_c, f) {
            // Face 1: (0,1)
            (0, 1, Facing::Up) => (3 * s + x, 0, Facing::Right),
            (0, 1, Facing::Left) => (3 * s - 1 - y, 0, Facing::Right),
            // Face 2: (0,2)
            (0, 2, Facing::Up) => (4 * s - 1, x, Facing::Up),
            (0, 2, Facing::Down) => (s + x, 2 * s - 1, Facing::Left),
            (0, 2, Facing::Right) => (3 * s - 1 - y, 2 * s - 1, Facing::Left),
            // Face 3: (1,1)
            (1, 1, Facing::Left) => (2 * s, y, Facing::Down),
            (1, 1, Facing::Right) => (s - 1, 2 * s + y, Facing::Up),
            // Face 4: (2,1)
            (2, 1, Facing::Down) => (3 * s + x, s - 1, Facing::Left),
            (2, 1, Facing::Right) => (s - 1 - y, 3 * s - 1, Facing::Left),
            // Face 5: (2,0)
            (2, 0, Facing::Up) => (s + x, s, Facing::Right),
            (2, 0, Facing::Left) => (s - 1 - y, s, Facing::Right),
            // Face 6: (3,0)
            (3, 0, Facing::Down) => (0, 2 * s + x, Facing::Down),
            (3, 0, Facing::Left) => (0, s + y, Facing::Down),
            (3, 0, Facing::Right) => (3 * s - 1, s + y, Facing::Up),
            _ => unreachable!("Invalid wrap for input: ({}, {}, {:?})", r, c, f),
        };
        (nr, nc, nf)
    }
}

impl WrappingStrategy for CubeWrapping {
    fn next_pos(&self, board: &Board, r: usize, c: usize, f: Facing) -> (usize, usize, Facing) {
        let (dr, dc) = f.delta();
        let nr = r as i32 + dr;
        let nc = c as i32 + dc;

        if board.is_valid(nr, nc) {
            return (nr as usize, nc as usize, f);
        }

        if self.face_size == 4 {
            self.wrap_example(board, r, c, f)
        } else {
            self.wrap_input(board, r, c, f)
        }
    }
}

struct Navigator {
    r: usize,
    c: usize,
    f: Facing,
}

impl Navigator {
    fn new(board: &Board) -> Self {
        let r = 0;
        let c = board.grid[0].iter().position(|&ch| ch == '.').unwrap();
        let f = Facing::Right;
        Navigator { r, c, f }
    }

    fn follow_instructions(
        &mut self,
        board: &Board,
        instructions: &[Instruction],
        strategy: &impl WrappingStrategy,
    ) {
        for inst in instructions {
            match inst {
                Instruction::Left => self.f = self.f.turn_left(),
                Instruction::Right => self.f = self.f.turn_right(),
                Instruction::Move(steps) => {
                    for _ in 0..*steps {
                        let (nr, nc, nf) = strategy.next_pos(board, self.r, self.c, self.f);
                        if board.grid[nr][nc] == '#' {
                            break;
                        }
                        self.r = nr;
                        self.c = nc;
                        self.f = nf;
                    }
                }
            }
        }
    }

    fn password(&self) -> i32 {
        1000 * (self.r as i32 + 1) + 4 * (self.c as i32 + 1) + self.f as i32
    }
}

fn solve<S: FromBoard>(input: &str) -> i32 {
    let (map_part, path_part) = input.split_once("\n\n").unwrap();
    let board = Board::parse(map_part);
    let instructions = Instruction::parse_all(path_part);
    let strategy = S::from_board(&board);

    let mut navigator = Navigator::new(&board);
    navigator.follow_instructions(&board, &instructions, &strategy);

    navigator.password()
}

fn part1(input: &str) -> i32 {
    solve::<FlatWrapping>(input)
}

fn part2(input: &str) -> i32 {
    solve::<CubeWrapping>(input)
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
        assert_eq!(part1(TEST_INPUT_1), 6032);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 191010);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 5031);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 55364);
    }
}
