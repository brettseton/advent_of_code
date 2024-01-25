use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;
use std::{fs, usize, fmt};

fn main() {
    let ans = part1("input/test1.txt");
    println!("part 1 test 1 answer: {}", ans);

    let ans = part1("input/test2.txt");
    println!("part 1 test 2 answer: {}", ans);

    let ans = part2("input/test1.txt");
    println!("part 2 test 1 answer: {}", ans);

    let ans = part2("input/test2.txt");
    println!("part 2 test 2 answer: {}", ans);
}

fn part1(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("Unable to read the input file");
    let mut game: Game<Hand> = Game::new(&input);
    game.hands.sort_by(|a, b| a.cmp(b));

    let sum = game
        .hands
        .iter()
        .rev()
        .enumerate()
        .map(|(pos, e)| e.bet * (pos + 1))
        .sum();
    return sum;
}

fn part2(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("Unable to read the input file");
    let mut game: Game<JHand> = Game::new(&input);
    game.hands.sort_by(|a, b| a.cmp(b));

    let sum = game
        .hands
        .iter()
        .rev()
        .enumerate()
        .map(|(pos, e)| e.bet * (pos + 1))
        .sum();
    return sum;
}

#[derive(Debug)]
struct Game<T> {
    hands: Vec<T>,
}

impl<T: FromStr> Game<T> where
<T as FromStr>::Err: fmt::Debug,
T: fmt::Debug {
    pub fn new(str: &str) -> Game<T> {
        return Game::<T>::from_str(str).expect("Ctor from string failed");
    }
}

#[derive(Debug)]
struct GameParseError;

impl<T: FromStr> FromStr for Game<T> where
<T as FromStr>::Err: fmt::Debug,
T: fmt::Debug {
    type Err = GameParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let hands: Vec<T> = str.lines().map(|x| T::from_str(x).expect("")).collect();

        return Ok(Game { hands });
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct Hand {
    hand: String,
    hand_type: HandType,
    bet: usize,
}

#[derive(Debug)]
struct HandParseError;

impl Hand {
    pub fn cmp(&self, other: &Hand) -> Ordering {
        let hand_type_cmp = self.hand_type.cmp(&other.hand_type);

        if hand_type_cmp != Ordering::Equal {
            return hand_type_cmp;
        }

        for (a, b) in self.hand.chars().zip(other.hand.chars()) {
            let card_cmp = Hand::card_cmp(&a, &b);
            if card_cmp != Ordering::Equal {
                return card_cmp;
            }
        }

        return Ordering::Equal;
    }

    fn card_cmp(a: &char, b: &char) -> Ordering {
        const RANK: &str = "AKQJT98765432";
        let a_index = RANK.find(*a);
        let b_index = RANK.find(*b);
        return a_index.cmp(&b_index);
    }

    pub fn get_hand_type(str: &str) -> HandType {
        let mut char_counts = HashMap::new();

        for c in str.chars() {
            let count = char_counts.entry(c).or_insert(0);
            *count += 1;
        }

        return match char_counts.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if let Some(4) = char_counts.values().max() {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if let Some(3) = char_counts.values().max() {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => HandType::HighCard,
        };
    }
}

impl FromStr for Hand {
    type Err = HandParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if let [hand, bet_str] = &str
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>()[..]
        {
            let hand_type = Hand::get_hand_type(hand);
            return Ok(Hand {
                hand: hand.to_string(),
                hand_type,
                bet: bet_str.parse::<usize>().expect("no bet?"),
            });
        } else {
            return Err(HandParseError);
        }
    }
}

#[derive(Debug)]
struct JHand {
    hand: String,
    hand_type: HandType,
    bet: usize,
}

impl FromStr for JHand {
    type Err = HandParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if let [hand, bet_str] = &str
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>()[..]
        {
            let hand_type = JHand::get_hand_type(hand);
            return Ok(JHand {
                hand: hand.to_string(),
                hand_type,
                bet: bet_str.parse::<usize>().expect("no bet?"),
            });
        } else {
            return Err(HandParseError);
        }
    }
}

#[derive(Debug)]
struct JHandParseError;

impl JHand {
    pub fn cmp(&self, other: &JHand) -> Ordering {
        let hand_type_cmp = self.hand_type.cmp(&other.hand_type);

        if hand_type_cmp != Ordering::Equal {
            return hand_type_cmp;
        }

        for (a, b) in self.hand.chars().zip(other.hand.chars()) {
            let card_cmp = JHand::card_cmp(&a, &b);
            if card_cmp != Ordering::Equal {
                return card_cmp;
            }
        }

        return Ordering::Equal;
    }

    fn card_cmp(a: &char, b: &char) -> Ordering {
        const RANK: &str = "AKQT98765432J";
        let a_index = RANK.find(*a);
        let b_index = RANK.find(*b);
        return a_index.cmp(&b_index);
    }

    pub fn get_hand_type(str: &str) -> HandType {
        let mut char_counts = HashMap::new();

        for c in str.chars() {
            char_counts.entry(c).and_modify(|count| *count += 1).or_insert(1);
        }

        // Give the jokers to the highest current count
        // and remove it from the set
        if let Some(&joker_count) = char_counts.get(&'J') {
            if char_counts.len() > 1 {
                let (largest_key, _largest_value) = char_counts
                    .iter()
                    .filter(|(k, _)| *k != &'J')
                    .max_by_key(|(_, &v)| v)
                    .map(|(k, &v)| (*k, v))
                    .unwrap_or(('J', 0));
        
                char_counts
                    .entry(largest_key)
                    .and_modify(|count| *count += joker_count)
                    .or_insert(joker_count);
        
                char_counts.remove(&'J');
            }
        }

        return match char_counts.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if let Some(4) = char_counts.values().max() {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if let Some(3) = char_counts.values().max() {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => HandType::HighCard,
        };
    }
}

#[test]
fn part1_test1() {
    let result = part1("input/test1.txt");
    assert_eq!(result, 6440);
}

#[test]
fn part1_test2() {
    let result = part1("input/test2.txt");
    assert_eq!(result, 249390788);
}

#[test]
fn part2_test1() {
    let result = part2("input/test1.txt");
    assert_eq!(result, 5905);
}

#[test]
fn part2_test2() {
    let result = part2("input/test2.txt");
    assert_eq!(result, 248750248);
}
