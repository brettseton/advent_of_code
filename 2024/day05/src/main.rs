use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug)]
struct Input {
    rules: Vec<(i32, i32)>,
    updates: Vec<Vec<i32>>,
}

fn parse_input(input: &str) -> Input {
    let mut parts = input.split("\n\n");

    // Parse rules
    let rules_str = parts.next().unwrap();
    let rules: Vec<(i32, i32)> = rules_str
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut nums = line.split('|');
            let before = nums.next().unwrap().parse().unwrap();
            let after = nums.next().unwrap().parse().unwrap();
            (before, after)
        })
        .collect();

    // Parse updates
    let updates_str = parts.next().unwrap();
    let updates: Vec<Vec<i32>> = updates_str
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    Input { rules, updates }
}

fn is_valid_order(update: &[i32], rules: &[(i32, i32)]) -> bool {
    // Build a map of positions for O(1) lookup
    let positions: HashMap<i32, usize> = update
        .iter()
        .enumerate()
        .map(|(i, &num)| (num, i))
        .collect();

    // Check each applicable rule
    for &(before, after) in rules {
        // Skip rules that don't apply to this update
        if !positions.contains_key(&before) || !positions.contains_key(&after) {
            continue;
        }

        // If the rule applies, check that 'before' comes before 'after'
        if positions[&before] >= positions[&after] {
            return false;
        }
    }
    true
}

fn sort_by_rules(pages: &[i32], rules: &[(i32, i32)]) -> Vec<i32> {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut in_degree: HashMap<i32, i32> = HashMap::new();
    let pages_set: HashSet<i32> = pages.iter().copied().collect();

    // Initialize in-degree for all pages
    for &page in pages {
        graph.entry(page).or_default();
        in_degree.insert(page, 0);
    }

    // Build graph and count in-degrees
    for &(before, after) in rules {
        // Only consider rules where both pages are in our update
        if pages_set.contains(&before) && pages_set.contains(&after) {
            graph.entry(before).or_default().push(after);
            *in_degree.entry(after).or_default() += 1;
        }
    }

    // Kahn's algorithm for topological sort
    let mut result = Vec::new();
    let mut queue: Vec<i32> = in_degree
        .iter()
        .filter(|&(_, &degree)| degree == 0)
        .map(|(&node, _)| node)
        .collect();

    while let Some(node) = queue.pop() {
        result.push(node);

        if let Some(neighbors) = graph.get(&node) {
            for &next in neighbors {
                *in_degree.get_mut(&next).unwrap() -= 1;
                if in_degree[&next] == 0 {
                    queue.push(next);
                }
            }
        }
    }

    result
}

fn part1(input: &str) -> i32 {
    let data = parse_input(input);

    // Process each update
    let mut sum = 0;
    for update in &data.updates {
        if is_valid_order(update, &data.rules) {
            // For valid updates, add the middle number
            let mid_idx = update.len() / 2;
            sum += update[mid_idx];
        }
    }

    sum
}

fn part2(input: &str) -> i32 {
    let data = parse_input(input);

    // Process each update
    let mut sum = 0;
    for update in &data.updates {
        if !is_valid_order(update, &data.rules) {
            // For invalid updates, sort them and get the middle number
            let sorted = sort_by_rules(update, &data.rules);
            let mid_idx = sorted.len() / 2;
            sum += sorted[mid_idx];
        }
    }

    sum
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
        assert_eq!(part1(&test_input), 143);
    }

    #[test]
    fn test2_part1() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 5108);
    }

    #[test]
    fn test1_part2() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 123);
    }

    #[test]
    fn test2_part2() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 0);
    }
}
