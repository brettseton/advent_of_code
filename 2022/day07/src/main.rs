const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

const TOTAL_DISK_SPACE: u64 = 70_000_000;
const UPDATE_REQUIRED_SPACE: u64 = 30_000_000;
const PART1_SIZE_LIMIT: u64 = 100_000;

use std::collections::HashMap;

#[derive(Debug)]
enum Node {
    File(u64),
    Directory(HashMap<String, Node>),
}

impl Node {
    fn new_dir() -> Self {
        Self::Directory(HashMap::new())
    }

    fn calculate_sizes(&self, all_sizes: &mut Vec<u64>) -> u64 {
        match self {
            Self::File(size) => *size,
            Self::Directory(children) => {
                let total_size: u64 = children
                    .values()
                    .map(|node| node.calculate_sizes(all_sizes))
                    .sum();
                all_sizes.push(total_size);
                total_size
            }
        }
    }

    fn get_or_create_dir(&mut self, path: &[String]) -> &mut HashMap<String, Node> {
        let mut current = self;
        for segment in path {
            if let Self::Directory(children) = current {
                current = children
                    .entry(segment.clone())
                    .or_insert_with(Self::new_dir);
            }
        }

        if let Self::Directory(children) = current {
            children
        } else {
            panic!("Expected directory at path, found file");
        }
    }
}

struct FileSystem {
    root: Node,
}

impl FileSystem {
    fn parse(input: &str) -> Self {
        let mut root = Node::new_dir();
        let mut current_path = Vec::new();

        for line in input.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts.as_slice() {
                ["$", "cd", "/"] => current_path.clear(),
                ["$", "cd", ".."] => {
                    current_path.pop();
                }
                ["$", "cd", dir] => current_path.push(dir.to_string()),
                ["$", "ls"] => {}
                ["dir", name] => {
                    root.get_or_create_dir(&current_path)
                        .entry(name.to_string())
                        .or_insert_with(Node::new_dir);
                }
                [size_str, name] if size_str.chars().all(|c| c.is_ascii_digit()) => {
                    let size = size_str.parse::<u64>().unwrap();
                    root.get_or_create_dir(&current_path)
                        .insert(name.to_string(), Node::File(size));
                }
                _ => {}
            }
        }
        Self { root }
    }

    fn dir_sizes(&self) -> Vec<u64> {
        let mut sizes = Vec::new();
        self.root.calculate_sizes(&mut sizes);
        sizes
    }
}

fn part1(input: &str) -> u64 {
    let fs = FileSystem::parse(input);
    fs.dir_sizes()
        .into_iter()
        .filter(|&size| size <= PART1_SIZE_LIMIT)
        .sum()
}

fn part2(input: &str) -> u64 {
    let fs = FileSystem::parse(input);
    let sizes = fs.dir_sizes();

    let used_space = *sizes.iter().max().unwrap_or(&0);
    let current_free_space = TOTAL_DISK_SPACE.saturating_sub(used_space);
    let need_to_free = UPDATE_REQUIRED_SPACE.saturating_sub(current_free_space);

    sizes
        .into_iter()
        .filter(|&size| size >= need_to_free)
        .min()
        .unwrap_or(0)
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
        assert_eq!(part1(TEST_INPUT_1), 95437);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 2061777);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 24933642);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 4473403);
    }
}
