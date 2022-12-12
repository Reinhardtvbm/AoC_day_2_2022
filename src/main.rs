use std::{cmp::Ordering, fs::read_to_string};

#[derive(PartialEq, Clone, Copy)]
enum Game {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl From<char> for Outcome {
    fn from(c: char) -> Self {
        match c {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            _ => Self::Win,
        }
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match other {
            Game::Rock => match self {
                Game::Rock => Some(Ordering::Equal),
                Game::Paper => Some(Ordering::Greater),
                Game::Scissors => Some(Ordering::Less),
            },
            Game::Paper => match self {
                Game::Rock => Some(Ordering::Less),
                Game::Paper => Some(Ordering::Equal),
                Game::Scissors => Some(Ordering::Greater),
            },
            Game::Scissors => match self {
                Game::Rock => Some(Ordering::Greater),
                Game::Paper => Some(Ordering::Less),
                Game::Scissors => Some(Ordering::Equal),
            },
        }
    }

    fn lt(&self, other: &Self) -> bool {
        if let Some(order) = self.partial_cmp(other) {
            match order {
                Ordering::Less => true,
                _ => false,
            }
        } else {
            false
        }
    }

    fn le(&self, other: &Self) -> bool {
        if let Some(order) = self.partial_cmp(other) {
            match order {
                Ordering::Greater => false,
                _ => true,
            }
        } else {
            false
        }
    }

    fn gt(&self, other: &Self) -> bool {
        if let Some(order) = self.partial_cmp(other) {
            match order {
                Ordering::Greater => true,
                _ => false,
            }
        } else {
            false
        }
    }

    fn ge(&self, other: &Self) -> bool {
        if let Some(order) = self.partial_cmp(other) {
            match order {
                Ordering::Less => false,
                _ => true,
            }
        } else {
            false
        }
    }
}

impl From<char> for Game {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::Rock,
            'B' => Self::Paper,
            _ => Self::Scissors,
        }
    }
}

fn main() {
    let file_contents = read_to_string("data.txt").unwrap();
    let lines = file_contents.lines();

    let score: i32 = lines
        .into_iter()
        .map(|line| {
            let player_1 = Game::from(line.chars().nth(0).unwrap());
            let outcome = Outcome::from(line.chars().nth(2).unwrap());

            let player_2 = match outcome {
                Outcome::Lose => match player_1 {
                    Game::Rock => Game::Scissors,
                    Game::Paper => Game::Rock,
                    Game::Scissors => Game::Paper,
                },
                Outcome::Win => match player_1 {
                    Game::Rock => Game::Paper,
                    Game::Paper => Game::Scissors,
                    Game::Scissors => Game::Rock,
                },
                Outcome::Draw => player_1.clone(),
            };

            let mut score = match player_2.partial_cmp(&player_1) {
                Some(order) => match order {
                    Ordering::Less => 0,
                    Ordering::Equal => 3,
                    Ordering::Greater => 6,
                },
                None => 0,
            };

            score += match player_2 {
                Game::Rock => 1,
                Game::Paper => 2,
                Game::Scissors => 3,
            };

            println!("{}", score);

            score
        })
        .sum();

    println!("\n sum = {}", score);
}
