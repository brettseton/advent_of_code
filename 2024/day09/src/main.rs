use std::{fmt::Display, fs};

/// Represents a contiguous block on the disk that either contains a file or is blank space
#[derive(Debug, Clone, Copy)]
struct DiskBlock {
    file_id: usize,
    length: usize,
    is_blank: bool,
}

impl Display for DiskBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_char = if self.is_blank {
            '.'
        } else {
            (b'A' + (self.file_id % 26) as u8) as char
        };

        for _ in 0..self.length {
            write!(f, "{}", display_char)?;
        }
        Ok(())
    }
}

/// Trait defining different strategies for disk block compaction
trait CompactionStrategy {
    /// Compacts the given disk blocks according to the strategy's rules
    /// Returns a new Vec containing the compacted blocks
    fn compact(&self, blocks: &[DiskBlock]) -> Vec<DiskBlock>;
}

/// Strategy that compacts files by moving them from right to left
struct LeftRightCompactionStrategy;

/// Strategy that compacts files by inserting them into available spaces
struct InsertionCompactionStrategy;

impl CompactionStrategy for LeftRightCompactionStrategy {
    fn compact(&self, blocks: &[DiskBlock]) -> Vec<DiskBlock> {
        let mut compacted_blocks = Vec::new();
        if blocks.is_empty() {
            return compacted_blocks;
        }

        let mut left_pos = 0;
        let mut right_pos = blocks.len() - 1;
        let mut current_blank_space = 0;
        let mut current_file_length = blocks[right_pos].length;

        // Skip trailing blank blocks
        while blocks[right_pos].is_blank {
            right_pos -= 1;
            current_file_length = blocks[right_pos].length;
        }

        while right_pos > left_pos {
            // Collect non-blank blocks from the left
            while !blocks[left_pos].is_blank {
                compacted_blocks.push(blocks[left_pos]);
                left_pos += 1;
                current_blank_space = blocks[left_pos].length;
            }

            match current_blank_space.cmp(&current_file_length) {
                std::cmp::Ordering::Greater => {
                    compacted_blocks.push(DiskBlock {
                        file_id: blocks[right_pos].file_id,
                        length: current_file_length,
                        is_blank: false,
                    });
                    current_blank_space -= current_file_length;
                    right_pos -= 1;
                    while blocks[right_pos].is_blank {
                        right_pos -= 1;
                    }
                    current_file_length = blocks[right_pos].length;
                }
                std::cmp::Ordering::Less => {
                    compacted_blocks.push(DiskBlock {
                        file_id: blocks[right_pos].file_id,
                        length: current_blank_space,
                        is_blank: false,
                    });
                    current_file_length -= current_blank_space;
                    left_pos += 1;
                }
                std::cmp::Ordering::Equal => {
                    compacted_blocks.push(DiskBlock {
                        file_id: blocks[right_pos].file_id,
                        length: current_blank_space,
                        is_blank: false,
                    });
                    left_pos += 1;
                    right_pos -= 1;
                    while blocks[right_pos].is_blank {
                        right_pos -= 1;
                    }
                    current_file_length = blocks[right_pos].length;
                }
            }
        }

        // Handle the case where there's one block left and space for it
        if current_blank_space > 0 && left_pos == right_pos {
            compacted_blocks.push(DiskBlock {
                file_id: blocks[right_pos].file_id,
                length: current_file_length,
                is_blank: false,
            });
        }

        compacted_blocks
    }
}

impl CompactionStrategy for InsertionCompactionStrategy {
    fn compact(&self, blocks: &[DiskBlock]) -> Vec<DiskBlock> {
        let mut compacted_blocks = blocks.to_vec();
        let mut next_block_index = blocks.len() - 1;

        while next_block_index > 0 {
            let current_block_index = next_block_index;
            next_block_index -= 1;

            // Skip blank blocks
            if compacted_blocks[current_block_index].is_blank {
                continue;
            }

            let file_to_move = compacted_blocks[current_block_index];
            for target_pos in 0..current_block_index {
                let target_block = compacted_blocks[target_pos];
                if target_block.is_blank && target_block.length >= file_to_move.length {
                    if target_block.length == file_to_move.length {
                        // Simple swap if spaces are equal
                        compacted_blocks[target_pos] = file_to_move;
                        compacted_blocks[current_block_index] = target_block;
                    } else {
                        // Split the blank space and insert the file
                        compacted_blocks[target_pos] = file_to_move;
                        compacted_blocks[current_block_index] = target_block;
                        compacted_blocks[current_block_index].length = file_to_move.length;

                        let remaining_space = DiskBlock {
                            file_id: target_block.file_id,
                            length: target_block.length - file_to_move.length,
                            is_blank: true,
                        };
                        compacted_blocks.insert(target_pos + 1, remaining_space);
                        next_block_index = compacted_blocks.len() - 1;
                    }
                    break;
                }
            }
        }
        compacted_blocks
    }
}

/// Calculates a checksum for the given disk layout
fn calculate_checksum(disk_layout: &[DiskBlock]) -> usize {
    let mut position = 0;
    disk_layout
        .iter()
        .map(|block| {
            (0..block.length)
                .map(|_| {
                    let value = if block.is_blank {
                        0
                    } else {
                        block.file_id * position
                    };
                    position += 1;
                    value
                })
                .sum::<usize>()
        })
        .sum()
}

/// Extension trait for disk compaction operations
trait DiskCompaction {
    /// Compacts the disk using the left-right strategy
    fn left_right_compact(&self) -> Vec<DiskBlock>;
    /// Compacts the disk using the insertion strategy
    fn insertion_compact(&self) -> Vec<DiskBlock>;
}

impl DiskCompaction for Vec<DiskBlock> {
    fn left_right_compact(&self) -> Vec<DiskBlock> {
        LeftRightCompactionStrategy.compact(self)
    }

    fn insertion_compact(&self) -> Vec<DiskBlock> {
        InsertionCompactionStrategy.compact(self)
    }
}

/// Parses the input string into a vector of disk blocks
fn parse_disk_map(input: &str) -> Vec<DiskBlock> {
    let mut blocks = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut pos = 0;

    while pos < chars.len() {
        let block_length = chars[pos].to_digit(10).unwrap() as usize;
        if block_length > 0 {
            blocks.push(DiskBlock {
                file_id: pos / 2,
                length: block_length,
                is_blank: pos % 2 == 1,
            });
        }
        pos += 1;
    }

    blocks
}

fn part1(input: &str) -> usize {
    let blocks = parse_disk_map(input);
    let compacted = blocks.left_right_compact();
    calculate_checksum(&compacted)
}

fn part2(input: &str) -> usize {
    let blocks = parse_disk_map(input);
    let compacted = blocks.insertion_compact();
    calculate_checksum(&compacted)
}

fn main() {
    let input1 = fs::read_to_string("input/test1.txt").unwrap();
    let input2 = fs::read_to_string("input/test2.txt").unwrap();

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
        assert_eq!(part1(&test_input), 1928);
    }

    #[test]
    fn test2_part1() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part1(&test_input), 6432869891895);
    }

    #[test]
    fn test1_part2() {
        let test_input =
            fs::read_to_string("input/test1.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 2858);
    }

    #[test]
    fn test2_part2() {
        let test_input =
            fs::read_to_string("input/test2.txt").expect("Should have been able to read the file");
        assert_eq!(part2(&test_input), 6467290479134);
    }
}
