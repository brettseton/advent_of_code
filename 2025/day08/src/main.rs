const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

type Pair = (usize, usize);

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn from_str(s: &str) -> Self {
        let parts: Vec<&str> = s.split(',').collect();
        Point {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
            z: parts[2].parse().unwrap(),
        }
    }

    fn distance_squared(&self, other: &Point) -> i64 {
        let dx = (self.x - other.x) as i64;
        let dy = (self.y - other.y) as i64;
        let dz = (self.z - other.z) as i64;
        dx * dx + dy * dy + dz * dz
    }
}

struct Boxes {
    points: Vec<Point>,
}

impl Boxes {
    fn from_str(input: &str) -> Self {
        let points: Vec<Point> = input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(Point::from_str)
            .collect();
        Boxes { points }
    }

    fn len(&self) -> usize {
        self.points.len()
    }

    fn get_point(&self, index: usize) -> Point {
        self.points[index]
    }

    fn sorted_pairs_by_distance(&self) -> Vec<Pair> {
        let n = self.points.len();
        let pair_count = n.saturating_sub(1) * n / 2;
        let mut pairs_with_distance: Vec<(Pair, i64)> = Vec::with_capacity(pair_count);

        for i in 0..n {
            for j in (i + 1)..n {
                let dist_sq = self.points[i].distance_squared(&self.points[j]);
                pairs_with_distance.push(((i, j), dist_sq));
            }
        }

        pairs_with_distance.sort_by_key(|&(_, dist)| dist);
        pairs_with_distance
            .into_iter()
            .map(|(pair, _)| pair)
            .collect()
    }
}

struct WiringPlan {
    boxes: Boxes,
    ordered_pairs: Vec<Pair>,
    uf: UnionFind,
}

impl WiringPlan {
    fn build_with_strategy<F>(input: &str, mut uf_factory: F) -> Self
    where
        F: FnMut(usize) -> UnionFind,
    {
        let boxes = Boxes::from_str(input);
        let ordered_pairs = boxes.sorted_pairs_by_distance();
        let uf = uf_factory(boxes.len());

        WiringPlan {
            boxes,
            ordered_pairs,
            uf,
        }
    }

    fn connect_first_n(&mut self, count: usize) {
        self.uf
            .connect_pairs(self.ordered_pairs.iter().copied(), count);
    }

    fn connect_until_single_circuit(&mut self) -> Vec<Pair> {
        self.uf
            .connect_until_single_circuit(self.ordered_pairs.iter().copied())
    }

    fn top_n_circuits(&mut self, n: usize) -> Option<Vec<usize>> {
        self.uf.top_n_circuits(n)
    }

    fn point(&self, index: usize) -> Point {
        self.boxes.get_point(index)
    }
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        // Union by size to keep trees shallow.
        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }
        true
    }

    fn get_circuit_sizes(&mut self) -> Vec<usize> {
        let n = self.parent.len();
        let mut seen_roots = std::collections::HashSet::new();
        let mut sizes = Vec::new();

        for i in 0..n {
            let root = self.find(i);
            if seen_roots.insert(root) {
                sizes.push(self.size[root]);
            }
        }

        sizes
    }

    fn top_n_circuits(&mut self, n: usize) -> Option<Vec<usize>> {
        let mut sizes = self.get_circuit_sizes();
        if sizes.len() < n {
            return None;
        }
        sizes.sort_unstable_by(|a, b| b.cmp(a));
        sizes.truncate(n);
        Some(sizes)
    }

    fn is_single_circuit(&mut self) -> bool {
        let n = self.parent.len();
        if n == 0 {
            return true;
        }

        let first_root = self.find(0);
        for i in 1..n {
            if self.find(i) != first_root {
                return false;
            }
        }

        true
    }

    fn connect_pairs<I>(&mut self, pairs: I, count: usize)
    where
        I: Iterator<Item = Pair>,
    {
        for (x, y) in pairs.take(count) {
            self.union(x, y);
        }
    }

    fn connect_until_single_circuit<I>(&mut self, pairs: I) -> Vec<Pair>
    where
        I: Iterator<Item = Pair>,
    {
        let mut connected_pairs = Vec::new();
        for (i, j) in pairs {
            if self.union(i, j) {
                connected_pairs.push((i, j));
                if self.is_single_circuit() {
                    break;
                }
            }
        }
        connected_pairs
    }
}

fn part1(input: &str, target_connections: usize) -> Option<i32> {
    let mut plan = WiringPlan::build_with_strategy(input, UnionFind::new);
    plan.connect_first_n(target_connections);
    plan.top_n_circuits(3)
        .map(|sizes| sizes.iter().product::<usize>() as i32)
}

fn part2(input: &str) -> Option<i32> {
    let mut plan = WiringPlan::build_with_strategy(input, UnionFind::new);
    let connected_pairs = plan.connect_until_single_circuit();

    connected_pairs
        .last()
        .map(|&(i, j)| plan.point(i).x * plan.point(j).x)
}

fn main() {
    println!("Part 1 test 1: {}", part1(TEST_INPUT_1, 10).unwrap_or(0));
    println!("Part 1 test 2: {}", part1(TEST_INPUT_2, 1000).unwrap_or(0));

    println!("Part 2 test 1: {}", part2(TEST_INPUT_1).unwrap_or(0));
    println!("Part 2 test 2: {}", part2(TEST_INPUT_2).unwrap_or(0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() {
        assert_eq!(part1(TEST_INPUT_1, 10), Some(40));
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2, 1000), Some(68112));
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), Some(25272));
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), Some(44543856));
    }
}
