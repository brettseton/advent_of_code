use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    str::FromStr,
};

fn main() {
    let ans = part1("input/test1.txt");
    println!("part 1 test 1 : {}", ans);

    let ans = part1("input/test2.txt");
    println!("part 1 test 2 : {}", ans);
}

fn part1(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("file input");
    let wire_diagram = WireDiagram::new(&input);
    wire_diagram.get_sum()
}

struct WireDiagram {
    graph: HashMap<String, HashSet<String>>,
}

impl WireDiagram {
    pub fn new(str: &str) -> WireDiagram {
        WireDiagram::from_str(str).expect("")
    }

    fn get_sum(&self) -> usize {
        let vertices: Vec<String> = self.graph.keys().cloned().collect();
        let n = vertices.len();
        let vertex_to_idx: HashMap<String, usize> = vertices
            .iter()
            .enumerate()
            .map(|(i, v)| (v.clone(), i))
            .collect();

        // Try each vertex as source until we find a cut of size 3
        let source = 0;
        for target in 1..n {
            let (flow, partition) = self.ford_fulkerson(&vertices, &vertex_to_idx, source, target);
            if flow == 3 {
                return partition.len() * (self.graph.len() - partition.len());
            }
        }

        panic!("No cut of size 3 found");
    }

    fn ford_fulkerson(
        &self,
        vertices: &[String],
        vertex_to_idx: &HashMap<String, usize>,
        source: usize,
        target: usize,
    ) -> (i32, HashSet<String>) {
        let n = vertices.len();
        let mut residual = vec![vec![0; n]; n];

        // Initialize residual graph
        for (v1, neighbors) in &self.graph {
            let i = vertex_to_idx[v1];
            for v2 in neighbors {
                let j = vertex_to_idx[v2];
                residual[i][j] = 1;
            }
        }

        let mut total_flow = 0;

        // Find augmenting paths using BFS
        while let Some(path) = self.find_path(&residual, source, target) {
            // Find minimum residual capacity along the path
            let mut min_flow = i32::MAX;
            for i in 0..path.len() - 1 {
                min_flow = min_flow.min(residual[path[i]][path[i + 1]]);
            }

            // Update residual capacities
            for i in 0..path.len() - 1 {
                let u = path[i];
                let v = path[i + 1];
                residual[u][v] -= min_flow;
                residual[v][u] += min_flow;
            }

            total_flow += min_flow;
        }

        // Find the partition (reachable vertices from source in residual graph)
        let mut partition = HashSet::new();
        let mut visited = vec![false; n];
        let mut queue = VecDeque::new();
        queue.push_back(source);
        visited[source] = true;

        while let Some(u) = queue.pop_front() {
            partition.insert(vertices[u].clone());
            for (v, visited_v) in visited.iter_mut().enumerate() {
                if !*visited_v && residual[u][v] > 0 {
                    *visited_v = true;
                    queue.push_back(v);
                }
            }
        }

        (total_flow, partition)
    }

    fn find_path(&self, residual: &[Vec<i32>], source: usize, target: usize) -> Option<Vec<usize>> {
        let n = residual.len();
        let mut visited = vec![false; n];
        let mut parent = vec![None; n];
        let mut queue = VecDeque::new();

        visited[source] = true;
        queue.push_back(source);

        while let Some(u) = queue.pop_front() {
            for v in 0..n {
                if !visited[v] && residual[u][v] > 0 {
                    visited[v] = true;
                    parent[v] = Some(u);
                    queue.push_back(v);
                }
            }
        }

        if !visited[target] {
            return None;
        }

        // Reconstruct path
        let mut path = Vec::new();
        let mut current = target;
        while let Some(p) = parent[current] {
            path.push(current);
            current = p;
        }
        path.push(source);
        path.reverse();
        Some(path)
    }
}

#[derive(Debug)]
struct WireDiagramError;

impl FromStr for WireDiagram {
    type Err = WireDiagramError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
        s.lines().for_each(|s| {
            let [label_str, children_str] =
                &s.split(':').map(String::from).collect::<Vec<String>>()[..]
            else {
                panic!()
            };
            let links = children_str
                .split_whitespace()
                .map(String::from)
                .collect::<Vec<String>>();

            for link in links.iter() {
                let entry = graph.entry(link.to_string()).or_default();
                entry.insert(label_str.to_string());
                let new_entry = graph.entry(label_str.to_string()).or_default();
                new_entry.insert(link.to_string());
            }
        });
        Ok(WireDiagram { graph })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part1_test1() {
        let ans = part1("input/test1.txt");
        assert_eq!(ans, 54);
    }

    #[test]
    pub fn part1_test2() {
        let ans = part1("input/test2.txt");
        assert_eq!(ans, 562912);
    }
}
