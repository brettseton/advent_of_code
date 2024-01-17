use std::{fs, str::FromStr, collections::BinaryHeap, cmp::Ordering};

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
    let heat_map = Grid::new(&input);
    return heat_map.get_path(vec![
        Beam{ x: 0, y: 0, traveling: Direction::East,  current_cost: 0, num_step: 0 },
        Beam{ x: 0, y: 0, traveling: Direction::South, current_cost: 0, num_step: 0 },
        Beam{ x: 0, y: 0, traveling: Direction::North, current_cost: 0, num_step: 0 },
        Beam{ x: 0, y: 0, traveling: Direction::West,  current_cost: 0, num_step: 0 },
        ]);
}

fn part2(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let heat_map = Grid::new(&input);
    return heat_map.get_path(vec![]);
}

#[derive(PartialEq, Eq, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn to_usize(&self) -> usize {
        match self {
            Self::North => 0,
            Self::East  => 1,
            Self::South => 2,
            Self::West  => 3,
        }
    }
}

impl Ord for Direction {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        self.to_usize().cmp(&other.to_usize())
    }
}

impl PartialOrd for Direction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Clone, Eq, PartialEq)]
struct Beam {
    x: usize,
    y: usize,
    traveling: Direction,
    current_cost: usize,
    //history: Vec<Direction>,
    num_step:usize
}

impl Ord for Beam {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.current_cost.cmp(&self.current_cost)
            .then_with(|| other.x.cmp(&self.x))
            .then_with(|| other.y.cmp(&self.y))
    }
}

impl PartialOrd for Beam {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Grid {
    grid: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}


impl Grid {
    pub fn new(str: &str) -> Grid {
        return Grid::from_str(str).expect("");
    }

    fn get_path(&self, start_beams: Vec<Beam>) -> usize {
        //let mut queue = start_beams;
        let mut heap = BinaryHeap::new();
        for start_beam in start_beams.iter() {
            heap.push(Beam{x: start_beam.x, y: start_beam.y, traveling: start_beam.traveling.clone(),  current_cost: 0, num_step: 0 });
        }

        let mut visited_map: Vec<Vec<Vec<Vec<usize>>>> = vec![vec![vec![vec![usize::MAX; 4]; 4]; self.width]; self.height];

        while let Some(beam) = heap.pop() {

            if beam.x == self.width - 1 && beam.y == self.height - 1 {
                return beam.current_cost;
            }
            if beam.current_cost > visited_map[beam.y][beam.x][beam.traveling.to_usize()][beam.num_step] {
                continue;
            }

            let beams = self.get_connected_beams(&beam);

            for new_beam in beams {
                match new_beam {
                    Some(b) => {
                        if b.current_cost < visited_map[b.y][b.x][b.traveling.to_usize()][b.num_step] {
                            visited_map[b.y][b.x][b.traveling.to_usize()][b.num_step] = b.current_cost;
                            heap.push(b);
                        }
                    },
                    None => (),
                }
            }
        }
        
        return 0;
    }

    pub fn get_connected_beams(&self, beam: &Beam) -> Vec<Option<Beam>> {
        return match beam.traveling {
            Direction::North => vec![self.get_beam(beam,  0, -1, Direction::North), self.get_beam(beam, -1,  0, Direction::West),  self.get_beam(beam, 1, 0, Direction::East)],
            Direction::South => vec![self.get_beam(beam,  0,  1, Direction::South), self.get_beam(beam, -1,  0, Direction::West),  self.get_beam(beam, 1, 0, Direction::East)],
            Direction::East  => vec![self.get_beam(beam,  1,  0, Direction::East ), self.get_beam(beam,  0, -1, Direction::North), self.get_beam(beam, 0, 1, Direction::South)],
            Direction::West  => vec![self.get_beam(beam, -1,  0, Direction::West ), self.get_beam(beam,  0, -1, Direction::North), self.get_beam(beam, 0, 1, Direction::South)],
        };
    }

    pub fn get_beam(&self, b: &Beam, dx: isize, dy: isize, traveling: Direction) -> Option<Beam> {

        let new_x = b.x.checked_add_signed(dx);
        let new_y = b.y.checked_add_signed(dy);

        if new_x.is_some_and(|x| x <self.width)
          && new_y.is_some_and(|y| y <self.height)
          && (b.num_step < 3 || b.traveling != traveling) {
            //let mut history = b.history.clone();
            //history.push(b.traveling.clone());
            let mut num_step = 1;
            
            if b.traveling == traveling {
                num_step += b.num_step;
            }

            return Some(Beam { 
                x: new_x.unwrap(),
                y: new_y.unwrap(),
                traveling, 
                current_cost: b.current_cost + self.grid[new_y.unwrap()][new_x.unwrap()],
                //history,
                num_step });
          }

        return None;
    }

}

#[derive(Debug)]
struct GridError;

impl FromStr for Grid {
    type Err = GridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().nth(0).unwrap().len();
        let height = s.lines().count();
        let grid = s
            .lines()
            .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>())
            .collect::<Vec<Vec<usize>>>();
        return Ok(Grid {
            grid,
            width,
            height,
        });
    }
}

#[test]
pub fn part1_test1() {
    let ans = part1("input/test1.txt");
    assert_eq!(ans, 102);
}

#[test]
pub fn part1_test2() {
    let ans = part1("input/test2.txt");
    assert_eq!(ans, 1260);
}

#[test]
pub fn part2_test1() {
    let ans = part2("input/test1.txt");
    assert_eq!(ans, 94);
}

#[test]
pub fn part2_test2() {
    let ans = part2("input/test2.txt");
    assert_eq!(ans, 0);
}
