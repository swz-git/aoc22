use std::str::FromStr;

const INPUT: &str = include_str!("./input.txt");

#[derive(Debug, PartialEq, Clone, Copy)]
enum RPSChoice {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for RPSChoice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(RPSChoice::Rock),
            "B" => Ok(RPSChoice::Paper),
            "C" => Ok(RPSChoice::Scissors),
            "X" => Ok(RPSChoice::Rock),
            "Y" => Ok(RPSChoice::Paper),
            "Z" => Ok(RPSChoice::Scissors),
            _ => Err(format!("'{}' is not a valid value for RPSChoice", s)),
        }
    }
}
impl RPSChoice {
    fn get_value(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn play_against(&self, opp: &RPSChoice) -> RPSResult {
        match (self, opp) {
            (RPSChoice::Rock, RPSChoice::Paper) => RPSResult::Loss,
            (RPSChoice::Rock, RPSChoice::Scissors) => RPSResult::Win,
            (RPSChoice::Paper, RPSChoice::Rock) => RPSResult::Win,
            (RPSChoice::Paper, RPSChoice::Scissors) => RPSResult::Loss,
            (RPSChoice::Scissors, RPSChoice::Rock) => RPSResult::Loss,
            (RPSChoice::Scissors, RPSChoice::Paper) => RPSResult::Win,
            _ => RPSResult::Draw,
        }
    }
}

enum RPSResult {
    Win,
    Loss,
    Draw,
}

impl RPSResult {
    fn get_value(&self) -> i32 {
        match self {
            Self::Win => 6,
            Self::Loss => 0,
            Self::Draw => 3,
        }
    }
}

fn points_from_game(me: RPSChoice, opp: RPSChoice) -> i32 {
    let game_points = me.play_against(&opp).get_value();
    let choice_points = me.get_value();
    return game_points + choice_points;
}

fn main() {
    // Parse games
    let games = INPUT
        .split("\n")
        .filter(|x| x != &"")
        .map(|game| {
            let x: Vec<RPSChoice> = game
                .split(" ")
                .map(|choice| choice.parse::<RPSChoice>().unwrap())
                .collect();
            (x[1], x[0]) // put me in front of opp
        })
        .collect::<Vec<(RPSChoice, RPSChoice)>>();

    // Play games
    let mut points_sum = 0;
    for game in games {
        points_sum += points_from_game(game.0, game.1)
    }

    println!("{}", points_sum)
}
