use std::collections::HashMap;

const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

const INFINITY: i32 = 1000;
const MAX_TIME: usize = 30;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ValveId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ValveMask(u32);

impl ValveMask {
    fn new() -> Self {
        Self(0)
    }

    fn set(&self, id: ValveId) -> Self {
        Self(self.0 | (1 << id.0))
    }

    fn is_set(&self, id: ValveId) -> bool {
        (self.0 & (1 << id.0)) != 0
    }

    fn as_usize(&self) -> usize {
        self.0 as usize
    }

    fn complement(&self, max_mask: u32) -> Self {
        Self((!self.0) & max_mask)
    }
}

struct RawValveInfo<'a> {
    name: &'a str,
    flow_rate: i32,
    neighbors: Vec<&'a str>,
}

struct ValveNode {
    flow_rate: i32,
    distances: Vec<i32>,
}

struct ValveNetwork {
    nodes: Vec<ValveNode>,
    start_id: ValveId,
    flow_node_count: usize,
}

#[derive(Debug)]
struct ParseNetworkError(String);

impl std::fmt::Display for ParseNetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse ValveNetwork: {}", self.0)
    }
}

impl std::error::Error for ParseNetworkError {}

struct NetworkBuilder<'a> {
    raw_data: Vec<RawValveInfo<'a>>,
}

impl<'a> NetworkBuilder<'a> {
    fn parse(input: &'a str) -> Result<Self, ParseNetworkError> {
        let raw_data = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 10 {
                    return Err(ParseNetworkError(format!("Invalid line format: {}", line)));
                }

                let name = parts[1];
                let flow_rate = parts[4]
                    .trim_start_matches("rate=")
                    .trim_end_matches(';')
                    .parse::<i32>()
                    .map_err(|e| ParseNetworkError(format!("Invalid flow rate: {}", e)))?;

                let neighbors = parts[9..]
                    .iter()
                    .map(|&s| s.trim_end_matches(','))
                    .collect();

                Ok(RawValveInfo {
                    name,
                    flow_rate,
                    neighbors,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { raw_data })
    }

    fn build(self) -> Result<ValveNetwork, ParseNetworkError> {
        let mut all_names: Vec<&str> = self.raw_data.iter().map(|v| v.name).collect();
        all_names.sort_unstable();

        let name_to_idx: HashMap<&str, usize> =
            all_names.iter().enumerate().map(|(i, &n)| (n, i)).collect();

        if !name_to_idx.contains_key("AA") {
            return Err(ParseNetworkError(
                "Starting valve 'AA' not found".to_string(),
            ));
        }

        let n = all_names.len();
        let dist_matrix = self.compute_distance_matrix(n, &name_to_idx);

        let mut flow_valves: Vec<&RawValveInfo> =
            self.raw_data.iter().filter(|v| v.flow_rate > 0).collect();
        flow_valves.sort_unstable_by_key(|v| v.name);

        let flow_node_count = flow_valves.len();

        let nodes: Vec<ValveNode> = flow_valves
            .iter()
            .map(|valve| {
                let distances =
                    self.map_distances(valve.name, &flow_valves, &name_to_idx, &dist_matrix, n);
                ValveNode {
                    flow_rate: valve.flow_rate,
                    distances,
                }
            })
            .collect();

        let start_distances = self.map_distances("AA", &flow_valves, &name_to_idx, &dist_matrix, n);

        let mut final_nodes = nodes;
        final_nodes.push(ValveNode {
            flow_rate: 0,
            distances: start_distances,
        });

        Ok(ValveNetwork {
            nodes: final_nodes,
            start_id: ValveId(flow_node_count),
            flow_node_count,
        })
    }

    fn compute_distance_matrix(&self, n: usize, name_to_idx: &HashMap<&str, usize>) -> Vec<i32> {
        let mut dists = vec![INFINITY; n * n];
        for i in 0..n {
            dists[i * n + i] = 0;
        }

        for valve in &self.raw_data {
            let u = name_to_idx[valve.name];
            for &neighbor in &valve.neighbors {
                if let Some(&v) = name_to_idx.get(neighbor) {
                    Self::dist_matrix_set(&mut dists, u, v, n, 1);
                }
            }
        }

        for k in 0..n {
            for i in 0..n {
                let ik = i * n + k;
                if dists[ik] >= INFINITY {
                    continue;
                }
                for j in 0..n {
                    let kj = k * n + j;
                    let ij = i * n + j;
                    dists[ij] = dists[ij].min(dists[ik] + dists[kj]);
                }
            }
        }
        dists
    }

    fn map_distances(
        &self,
        source: &str,
        targets: &[&RawValveInfo],
        name_to_idx: &HashMap<&str, usize>,
        matrix: &[i32],
        n: usize,
    ) -> Vec<i32> {
        let u = name_to_idx[source];
        targets
            .iter()
            .map(|target| matrix[u * n + name_to_idx[target.name]])
            .collect()
    }

    fn dist_matrix_set(matrix: &mut [i32], u: usize, v: usize, n: usize, val: i32) {
        matrix[u * n + v] = val;
    }
}

impl ValveNetwork {
    fn mask_count(&self) -> usize {
        1 << self.flow_node_count
    }

    fn max_mask(&self) -> u32 {
        (self.mask_count() - 1) as u32
    }

    fn iter_masks(&self) -> impl Iterator<Item = ValveMask> {
        (0..self.mask_count()).map(|m| ValveMask(m as u32))
    }

    fn solve_for_subsets(&self, time_limit: i32) -> Vec<i32> {
        let mut max_pressures = vec![0; self.mask_count()];
        // Memoization table: [valve_index][mask][time_remaining]
        let time_dim = MAX_TIME + 1;
        let mut memo = vec![-1; self.nodes.len() * self.mask_count() * time_dim];

        self.dfs(
            self.start_id,
            time_limit,
            ValveMask::new(),
            0,
            &mut max_pressures,
            &mut memo,
        );
        self.sos_dp(max_pressures)
    }

    fn dfs(
        &self,
        u: ValveId,
        time: i32,
        mask: ValveMask,
        pressure: i32,
        max_pressures: &mut [i32],
        memo: &mut [i32],
    ) {
        max_pressures[mask.as_usize()] = max_pressures[mask.as_usize()].max(pressure);

        let time_dim = MAX_TIME + 1;
        let memo_idx = (u.0 * self.mask_count() + mask.as_usize()) * time_dim + time as usize;
        if memo[memo_idx] >= pressure {
            return;
        }
        memo[memo_idx] = pressure;

        let node = &self.nodes[u.0];
        for (v_idx, &dist) in node.distances.iter().enumerate() {
            let v = ValveId(v_idx);
            if !mask.is_set(v) {
                let time_left = time - dist - 1;
                if time_left > 0 {
                    let next_pressure = pressure + self.nodes[v.0].flow_rate * time_left;
                    self.dfs(
                        v,
                        time_left,
                        mask.set(v),
                        next_pressure,
                        max_pressures,
                        memo,
                    );
                }
            }
        }
    }

    fn sos_dp(&self, mut f: Vec<i32>) -> Vec<i32> {
        for i in 0..self.flow_node_count {
            for mask in 0..self.mask_count() {
                if (mask & (1 << i)) != 0 {
                    f[mask] = f[mask].max(f[mask ^ (1 << i)]);
                }
            }
        }
        f
    }
}

fn part1(input: &str) -> Result<i32, ParseNetworkError> {
    let network = NetworkBuilder::parse(input)?.build()?;
    let subset_pressures = network.solve_for_subsets(30);
    Ok(*subset_pressures.iter().max().unwrap_or(&0))
}

fn part2(input: &str) -> Result<i32, ParseNetworkError> {
    let network = NetworkBuilder::parse(input)?.build()?;
    let subset_pressures = network.solve_for_subsets(26);

    let max_mask = network.max_mask();
    let max_total = network
        .iter_masks()
        .map(|mask| {
            let p1 = subset_pressures[mask.as_usize()];
            let p2 = subset_pressures[mask.complement(max_mask).as_usize()];
            p1 + p2
        })
        .max()
        .unwrap_or(0);
    Ok(max_total)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Part 1 test 1: {}", part1(TEST_INPUT_1)?);
    println!("Part 1 test 2: {}", part1(TEST_INPUT_2)?);

    println!("Part 2 test 1: {}", part2(TEST_INPUT_1)?);
    println!("Part 2 test 2: {}", part2(TEST_INPUT_2)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() {
        assert_eq!(part1(TEST_INPUT_1).unwrap(), 1651);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2).unwrap(), 1580);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1).unwrap(), 1707);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2).unwrap(), 2213);
    }
}
