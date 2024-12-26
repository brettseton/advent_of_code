use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug)]
struct Graph {
    adjacency_list: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn new(input: &str) -> Self {
        let mut adjacency_list: HashMap<String, HashSet<String>> = HashMap::new();

        for line in input.lines() {
            let (a, b) = line.split_once('-').unwrap();
            adjacency_list
                .entry(a.to_string())
                .or_default()
                .insert(b.to_string());
            adjacency_list
                .entry(b.to_string())
                .or_default()
                .insert(a.to_string());
        }

        Self { adjacency_list }
    }

    fn find_triangles(&self) -> Vec<Vec<String>> {
        let mut triangles = Vec::new();
        let nodes: Vec<&String> = self.adjacency_list.keys().collect();

        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                if !self.adjacency_list[nodes[i]].contains(nodes[j]) {
                    continue;
                }

                for k in (j + 1)..nodes.len() {
                    if self.adjacency_list[nodes[i]].contains(nodes[k])
                        && self.adjacency_list[nodes[j]].contains(nodes[k])
                    {
                        let mut triangle =
                            vec![nodes[i].clone(), nodes[j].clone(), nodes[k].clone()];
                        triangle.sort();
                        triangles.push(triangle);
                    }
                }
            }
        }

        triangles
    }

    fn create_adjacency_matrix(&self) -> (Vec<Vec<bool>>, Vec<String>) {
        let nodes: Vec<String> = self.adjacency_list.keys().cloned().collect();
        let n = nodes.len();
        let mut matrix = vec![vec![false; n]; n];

        for i in 0..n {
            for j in 0..n {
                if i != j {
                    matrix[i][j] = self.adjacency_list[&nodes[i]].contains(&nodes[j]);
                }
            }
        }

        (matrix, nodes)
    }

    fn branch_and_bound(
        pos: usize,
        current: &mut Vec<usize>,
        max_clique: &mut Vec<usize>,
        adj_matrix: &[Vec<bool>],
        n: usize,
    ) {
        if current.len() + (n - pos) <= max_clique.len() {
            return;
        }

        if pos == n {
            if current.len() > max_clique.len() {
                max_clique.clear();
                max_clique.extend(current.iter());
            }
            return;
        }

        if current.iter().all(|&i| adj_matrix[i][pos]) {
            current.push(pos);
            Self::branch_and_bound(pos + 1, current, max_clique, adj_matrix, n);
            current.pop();
        }

        Self::branch_and_bound(pos + 1, current, max_clique, adj_matrix, n);
    }

    fn find_largest_clique(&self) -> Vec<String> {
        let (adj_matrix, nodes) = self.create_adjacency_matrix();
        let mut current = Vec::new();
        let mut max_indices = Vec::new();
        let n = nodes.len();

        Self::branch_and_bound(0, &mut current, &mut max_indices, &adj_matrix, n);

        max_indices.iter().map(|&i| nodes[i].clone()).collect()
    }
}

fn part1(input: &str) -> i32 {
    let graph = Graph::new(input);
    let triangles = graph.find_triangles();

    triangles
        .iter()
        .filter(|triangle| triangle.iter().any(|node| node.starts_with('t')))
        .count() as i32
}

fn part2(input: &str) -> String {
    let graph = Graph::new(input);
    let mut clique = graph.find_largest_clique();
    clique.sort();
    clique.join(",")
}

fn main() {
    let input1 =
        fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
    let input2 =
        fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");

    println!("Part 1 test 1: {}", part1(&input1));
    println!("Part 1 test 2: {}", part1(&input2));

    println!("Part 2 test 1: {}", part2(&input1));
    println!("Part 2 test 2: {}", part2(&input2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 7);
    }

    #[test]
    fn test2_part1() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 1240);
    }

    #[test]
    fn test1_part2() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), "co,de,ka,ta");
    }

    #[test]
    fn test2_part2() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), "am,aq,by,ge,gf,ie,mr,mt,rw,sn,te,yi,zb");
    }
}
