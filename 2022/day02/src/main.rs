const TEST_INPUT_1: &str = include_str!("../input/test1.txt");
const TEST_INPUT_2: &str = include_str!("../input/test2.txt");

#[derive(Clone, Copy, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Move {
    fn from_opponent_code(code: &str) -> Option<Self> {
        match code {
            "A" => Some(Move::Rock),
            "B" => Some(Move::Paper),
            "C" => Some(Move::Scissors),
            _ => None,
        }
    }

    fn from_player_code(code: &str) -> Option<Self> {
        match code {
            "X" => Some(Move::Rock),
            "Y" => Some(Move::Paper),
            "Z" => Some(Move::Scissors),
            _ => None,
        }
    }

    fn score(self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn beats(self, other: Move) -> bool {
        matches!(
            (self, other),
            (Move::Rock, Move::Scissors)
                | (Move::Paper, Move::Rock)
                | (Move::Scissors, Move::Paper)
        )
    }

    fn move_for_outcome(self, outcome: Outcome) -> Move {
        match outcome {
            Outcome::Draw => self,
            Outcome::Win => self.winning_move(),
            Outcome::Loss => self.losing_move(),
        }
    }

    fn winning_move(self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn losing_move(self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }
}

impl Outcome {
    fn from_code(code: &str) -> Option<Self> {
        match code {
            "X" => Some(Outcome::Loss),
            "Y" => Some(Outcome::Draw),
            "Z" => Some(Outcome::Win),
            _ => None,
        }
    }

    fn score(self) -> i32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }

    fn determine(player: Move, opponent: Move) -> Self {
        if player.beats(opponent) {
            Outcome::Win
        } else if opponent.beats(player) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }
}

fn parse_round(line: &str) -> Option<(&str, &str)> {
    let mut parts = line.split_whitespace();
    Some((parts.next()?, parts.next()?))
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|line| {
            let (opponent_code, player_code) = parse_round(line)?;
            let opponent = Move::from_opponent_code(opponent_code)?;
            let player = Move::from_player_code(player_code)?;

            let outcome = Outcome::determine(player, opponent);
            Some(player.score() + outcome.score())
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|line| {
            let (opponent_code, outcome_code) = parse_round(line)?;
            let opponent = Move::from_opponent_code(opponent_code)?;
            let desired_outcome = Outcome::from_code(outcome_code)?;

            let player_move = opponent.move_for_outcome(desired_outcome);
            Some(player_move.score() + desired_outcome.score())
        })
        .sum()
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
        assert_eq!(part1(TEST_INPUT_1), 15);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(TEST_INPUT_2), 11150);
    }

    #[test]
    fn test1_part2() {
        assert_eq!(part2(TEST_INPUT_1), 12);
    }

    #[test]
    fn test2_part2() {
        assert_eq!(part2(TEST_INPUT_2), 8295);
    }
}
