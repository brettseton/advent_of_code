use std::collections::{HashMap, HashSet};

const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");
const TEST_INPUT_3: &str = include_str!("../input/test3.txt");

fn parse_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut graph = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            continue;
        }
        let device = parts[0].trim();
        let outputs: Vec<&str> = parts[1].split_whitespace().collect();
        graph.insert(device, outputs);
    }
    graph
}

fn part1(input: &str) -> i32 {
    let graph = parse_graph(input);

    fn count_paths(
        graph: &HashMap<&str, Vec<&str>>,
        current: &str,
        target: &str,
        visited: &mut HashSet<String>,
    ) -> i32 {
        if current == target {
            return 1;
        }

        let key = current.to_string();
        if visited.contains(&key) {
            return 0;
        }

        visited.insert(key.clone());
        let mut count = 0;

        if let Some(neighbors) = graph.get(current) {
            for neighbor in neighbors {
                count += count_paths(graph, neighbor, target, visited);
            }
        }

        visited.remove(&key);
        count
    }

    let mut visited = HashSet::new();
    count_paths(&graph, "you", "out", &mut visited)
}

fn part2(input: &str) -> i64 {
    let graph = parse_graph(input);

    if !graph.contains_key("svr") || !graph.contains_key("dac") || !graph.contains_key("fft") {
        return 0;
    }

    fn count_paths_memoized(
        graph: &HashMap<&str, Vec<&str>>,
        start: &str,
        target: &str,
        visited: &mut HashSet<String>,
        memo: &mut HashMap<String, i128>,
    ) -> i128 {
        if start == target {
            return 1;
        }

        let key = start.to_string();
        if let Some(&cached) = memo.get(&key) {
            return cached;
        }

        if visited.contains(&key) {
            return 0;
        }

        visited.insert(key.clone());
        let mut count = 0i128;

        if let Some(neighbors) = graph.get(start) {
            for neighbor in neighbors {
                count += count_paths_memoized(graph, neighbor, target, visited, memo);
            }
        }

        visited.remove(&key);
        memo.insert(key, count);
        count
    }

    let compute_segment = |start: &str, target: &str| -> i128 {
        let mut memo = HashMap::new();
        let mut visited = HashSet::new();
        count_paths_memoized(&graph, start, target, &mut visited, &mut memo)
    };

    // Compute all path segments
    // Paths: (svr->dac->fft->out) + (svr->fft->dac->out)
    let dac_out = compute_segment("dac", "out");
    let fft_dac = compute_segment("fft", "dac");
    let svr_fft = compute_segment("svr", "fft");
    let fft_out = compute_segment("fft", "out");
    let dac_fft = compute_segment("dac", "fft");
    let svr_dac = compute_segment("svr", "dac");

    // Combine segments: (svr_dac * dac_fft * fft_out) + (svr_fft * fft_dac * dac_out)
    let result = (svr_dac * dac_fft * fft_out) + (svr_fft * fft_dac * dac_out);
    result as i64
}

fn main() {
    println!("Part 1 test 1: {}", part1(TEST_INPUT_1));
    println!("Part 1 test 2: {}", part1(TEST_INPUT_2));

    println!("Part 2 test 1: {}", part2(TEST_INPUT_1));
    println!("Part 2 test 2: {}", part2(TEST_INPUT_2));
    println!("Part 2 test 3: {}", part2(TEST_INPUT_3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part1() {
        assert_eq!(part1(TEST_INPUT_1), 5);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 649);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 0);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 458948453421420);
    }

    #[test]
    fn test3_part2() {
        assert_eq!(part2(TEST_INPUT_3), 2);
    }
}
