use std::{io, env};
use std::collections::HashMap;
use RPSResult::*;
use MatchResult::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum RPSResult {
    Rock,
    Paper,
    Scissors
}

impl From::<char> for RPSResult {
    fn from(ch: char) -> Self {
        match ch {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Scissors,
            _ => panic!("Got unexpected value '{}' for RPSResult", ch)
        }
    }
}

impl RPSResult {
    fn points(self) -> u32 {
        match &self {
            Rock => 1,
            Paper => 2,
            Scissors => 3
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum MatchResult {
    Loss = 0,
    Tie = 1,
    Win = 2
}

impl From::<char> for MatchResult {
    fn from(ch: char) -> Self {
        match ch {
            // From the perspective of the other player
            'X' => Loss,
            'Y' => Tie,
            'Z' => Win,
            _ => panic!("Got unexpected value '{}' for MatchResult", ch)
        }
    }
}

impl MatchResult {
    fn points(self) -> u32 {
        match &self {
            Win => 6,
            Tie => 3,
            Loss => 0
        }
    }
}


fn main() {
    let aoc_phase = env::var("AOC_PHASE").unwrap_or("1".to_string())
        .parse::<usize>().expect("Bad value passed in for AOC_PHASE");

    let winning_moves: HashMap<RPSResult, [RPSResult; 3]> = HashMap::from([
        // Play               Loss       Tie       Win
        (RPSResult::Rock,     [Scissors, Rock,     Paper]),
        (RPSResult::Paper,    [Rock,     Paper,    Scissors]),
        (RPSResult::Scissors, [Paper,    Scissors, Rock])
    ]);

    let mut line = String::new();
    let mut points: u32 = 0;
    while io::stdin().read_line(&mut line).unwrap_or(0) != 0 {
        if aoc_phase == 1 {
            // Begin part 1
            let row: Vec<RPSResult> = line.split_whitespace().map(|f|
            RPSResult::from(f.chars().nth(0).unwrap())).collect();
            let (play, resp) = (row[0], row[1]);

            points += resp.points();
            points += if play == resp { 3 }
            else if winning_moves[&resp][Loss as usize] == play { 6 }
            else { 0 };
            // End part 1
        } else if aoc_phase == 2 {
            // Begin part 2
            let row: Vec<char> = line.split_whitespace().map(|f| f.chars().nth(0).unwrap()).collect();
            let (play, resp) = (RPSResult::from(row[0]), MatchResult::from(row[1]));
            let resp_play = winning_moves[&play][resp as usize];
            points += resp_play.points();
            points += resp.points();
            // End part 2
        } else {
            panic!("Unexpected AOC_PHASE")
        }

        line.clear();
    };
    println!("Result: {}", points);
}
