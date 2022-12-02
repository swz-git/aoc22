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

impl FromStr for RPSResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(RPSResult::Loss),
            "Y" => Ok(RPSResult::Draw),
            "Z" => Ok(RPSResult::Win),
            _ => Err(format!("'{}' is not a valid value for RPSResult", s)),
        }
    }
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
            let x: Vec<&str> = game.split(" ").collect();
            let opp = x[0].parse::<RPSChoice>().unwrap();
            let result = x[1].parse::<RPSResult>().unwrap();
            let me = match (opp, result) {
                (RPSChoice::Rock, RPSResult::Win) => RPSChoice::Paper,
                (RPSChoice::Rock, RPSResult::Loss) => RPSChoice::Scissors,
                (RPSChoice::Paper, RPSResult::Win) => RPSChoice::Scissors,
                (RPSChoice::Paper, RPSResult::Loss) => RPSChoice::Rock,
                (RPSChoice::Scissors, RPSResult::Win) => RPSChoice::Rock,
                (RPSChoice::Scissors, RPSResult::Loss) => RPSChoice::Paper,
                _ => opp,
            };
            (me, opp) // put me in front of opp
        })
        .collect::<Vec<(RPSChoice, RPSChoice)>>();

    // Play games
    let mut points_sum = 0;
    for game in games {
        // println!("{:?}, {:?}", game, points_from_game(game.0, game.1));
        points_sum += points_from_game(game.0, game.1)
    }

    println!("{}", points_sum)
}
