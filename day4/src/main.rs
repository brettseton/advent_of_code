use std::fs;
use std::str::FromStr;

fn main() {
    let ans = part1("C:/git/advent_of_code/day4/input/test1.txt");
    println!("part 1 test 1 answer: {}", ans);

    let ans = part1("C:/git/advent_of_code/day4/input/test2.txt");
    println!("part 1 test 2 answer: {}", ans);

    let ans = part2("C:/git/advent_of_code/day4/input/test1.txt");
    println!("part 2 test 1 answer: {}", ans);

    let ans = part2("C:/git/advent_of_code/day4/input/test2.txt");
    println!("part 2 test 2 answer: {}", ans);
}

fn part1(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("Unable to read the input file");
    let game = Game::new(&input);
    return game.scratch_cards.iter().map(|x| x.points).sum();
}

fn part2(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("Unable to read the input file");
    let game = Game::new(&input);

    let mut duplicates = vec![1_usize ; game.scratch_cards.len()];
    for index in 0..duplicates.len()  {
        let card = game.scratch_cards.iter().nth(index).expect("no cards?!");
        for i in 1..=card.overlap_count {
            duplicates[index+i] += duplicates[index];
        }
    }

    return duplicates.iter().sum();
}

struct Game {
    scratch_cards: Vec<ScratchCard>
}

impl Game {
    pub fn new(str: &str) -> Game {
        return Game::from_str(str).expect("Ctor from string failed");
    }
}

#[derive(Debug)]
struct GameParseError;

impl FromStr for Game {
    type Err = GameParseError;
    
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut scratch_cards = Vec::new();
        for line in str.lines() {
            scratch_cards.push(ScratchCard::new(line));
        }
        return Ok(Game { scratch_cards });
    }
}

#[derive(Clone)]
struct ScratchCard {
    id: usize,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
    overlap_count: usize,
    points: usize
}

impl ScratchCard {
    pub fn new(str: &str) -> ScratchCard {
        return ScratchCard::from_str(str).expect("Ctor from string failed");
    }
}

#[derive(Debug)]
struct ScratchCardParseError;

impl FromStr for ScratchCard {
    type Err = ScratchCardParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let game_split: Vec<&str> = str.split(':').collect();
        if game_split.len() != 2 {
            return Err(ScratchCardParseError);
        }

        let id = game_split[0].split_whitespace().nth(1).expect("No Game Id present").parse::<usize>().expect("Unable to parse Game Id");
        let winning_numbers: Vec<u32>;
        let numbers: Vec<u32>;
        
        if let [winning_str, number_str] = &game_split[1].split('|').map(String::from).collect::<Vec<String>>()[..] {
            winning_numbers = winning_str.split_whitespace().into_iter().map(|x| x.parse::<u32>().expect("Unable to parse winning number")).collect();
            numbers = number_str.split_whitespace().into_iter().map(|x| x.parse::<u32>().expect("Unable to parse numbers")).collect();
        } else {
            return Err(ScratchCardParseError);
        }

        let overlap: Vec<u32> = numbers
        .iter()
        .filter(|&value| winning_numbers.contains(value))
        .cloned()
        .collect();

        let points = if overlap.len() > 0 {
            2_usize.pow((overlap.len() - 1) as u32)
        } else {
            0
        };

        return Ok(ScratchCard { id, winning_numbers, numbers, overlap_count: overlap.len(), points});
    }
}


#[test]
fn part1_test1() {
    let result = part1("C:/git/advent_of_code/day4/input/test1.txt");
    assert_eq!(result, 13);
}

#[test]
fn part1_test2() {
    let result = part1("C:/git/advent_of_code/day4/input/test2.txt");
    assert_eq!(result, 23441);
}

#[test]
fn part2_test1() {
    let result = part2("C:/git/advent_of_code/day4/input/test1.txt");
    assert_eq!(result, 30);
}

#[test]
fn part2_test2() {
    let result = part2("C:/git/advent_of_code/day4/input/test2.txt");
    assert_eq!(result, 5923918);
}
