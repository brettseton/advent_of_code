use std::{fmt::Display, fs};

#[derive(Debug, Clone, Copy)]
struct DiskBlock {
    file_id: usize,
    length: usize,
    is_blank: bool,
}

impl Display for DiskBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.length {
            write!(
                f,
                "{}",
                if self.is_blank {
                    '.'
                } else {
                    (b'A' + (self.file_id % 26) as u8) as char
                }
            )?;
        }
        Ok(())
    }
}

fn parse_disk_map(input: &str) -> Vec<DiskBlock> {
    let mut blocks = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // Get file length
        let file_len = chars[i].to_digit(10).unwrap() as usize;
        if file_len > 0 {
            blocks.push(DiskBlock {
                file_id: i / 2,
                length: file_len,
                is_blank: i % 2 == 1,
            });
        }
        i += 1;
    }

    blocks
}

fn compact_disk(blocks: &[DiskBlock]) -> Vec<DiskBlock> {
    let mut result = Vec::new();
    //iterate through the blocks with a left and right pointer
    let mut left = 0;
    let mut right = blocks.len() - 1;
    let mut remaining_blanks_left = 0;
    let mut remaining_length_right = blocks[right].length;

    while right > left {
        while !blocks[left].is_blank {
            result.push(blocks[left]);
            left += 1;
            remaining_blanks_left = blocks[left].length;
        }

        match remaining_blanks_left.cmp(&remaining_length_right) {
            std::cmp::Ordering::Greater => {
                result.push(DiskBlock {
                    file_id: blocks[right].file_id,
                    length: remaining_length_right,
                    is_blank: false,
                });
                remaining_blanks_left -= remaining_length_right;
                right -= 1;
                while blocks[right].is_blank {
                    right -= 1;
                }
                remaining_length_right = blocks[right].length;
            }
            std::cmp::Ordering::Less => {
                result.push(DiskBlock {
                    file_id: blocks[right].file_id,
                    length: remaining_blanks_left,
                    is_blank: false,
                });
                remaining_length_right -= remaining_blanks_left;
                left += 1;
            }
            std::cmp::Ordering::Equal => {
                result.push(DiskBlock {
                    file_id: blocks[right].file_id,
                    length: remaining_blanks_left,
                    is_blank: false,
                });
                left += 1;
                right -= 1;
                while blocks[right].is_blank {
                    right -= 1;
                }
                remaining_length_right = blocks[right].length;
            }
        }
    }

    if remaining_blanks_left > 0 && left == right {
        result.push(DiskBlock {
            file_id: blocks[right].file_id,
            length: remaining_length_right,
            is_blank: false,
        });
    }

    result
}

fn compact_disk_part2(blocks: &mut Vec<DiskBlock>) {
    // iterate through the blocks in reverse
    // for each block try to insert it into the first position where it fits
    let mut i = blocks.len() - 1;
    while i > 0 {
        let x = blocks.as_slice();
        while blocks[i].is_blank {
            i -= 1;
        }
        let end_file = blocks[i];
        for j in 0..i {
            if blocks[j].is_blank && blocks[j].length >= end_file.length {
                let blank_file = blocks[j];
                if blank_file.length == end_file.length {
                    blocks[j] = end_file;
                    blocks[i] = blank_file;
                } else {
                    // split the block into two parts
                    blocks[j] = end_file;
                    blocks[i] = blank_file;

                    blocks[i].length = end_file.length;
                    blocks.insert(
                        j + 1,
                        DiskBlock {
                            file_id: blank_file.file_id,
                            length: blank_file.length - end_file.length,
                            is_blank: true,
                        },
                    );
                    i += 1;
                }
                break;
            }
        }
        i -= 1;
    }
}

fn calculate_checksum(disk: &[DiskBlock]) -> usize {
    let mut i = 0;
    disk.iter()
        .map(|disk_block| {
            let mut sum = 0;
            for _j in 0..disk_block.length {
                sum += if disk_block.is_blank {
                    0
                } else {
                    disk_block.file_id * i
                };
                i += 1;
            }
            sum
        })
        .sum()
}

fn part1(input: &str) -> usize {
    let blocks = parse_disk_map(input);
    let compacted = compact_disk(&blocks);
    calculate_checksum(&compacted)
}

fn part2(input: &str) -> usize {
    let mut blocks = parse_disk_map(input);
    compact_disk_part2(&mut blocks);
    calculate_checksum(&blocks)
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
