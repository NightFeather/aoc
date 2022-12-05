use std::env::args;
use std::io::{self, BufReader, prelude::*};
use std::fs::File;
use std::cmp::{Ord, Eq, PartialOrd, PartialEq, Ordering};
use std::ops::{Add, Sub};

#[derive(Debug, Eq, PartialOrd, PartialEq, Clone, Copy)]
enum GameThrow {
    Rock = 0,
    Scissors,
    Paper
}

#[derive(Debug, Clone, Copy)]
enum GameStrategy {
    Lose = -1,
    Draw,
    Win,
}

impl From<&String> for GameThrow {
    fn from(c: &String) -> Self {
        match c.as_str() {
            "A" => GameThrow::Rock,
            "B" => GameThrow::Scissors,
            "C" => GameThrow::Paper,
            "X" => GameThrow::Rock,
            "Y" => GameThrow::Scissors,
            "Z" => GameThrow::Paper,
            _ => unimplemented!(),
        }
    } 
}

impl From<&String> for GameStrategy {
    fn from(c: &String) -> Self {
        match c.as_str() {
            "X" => GameStrategy::Lose,
            "Y" => GameStrategy::Draw,
            "Z" => GameStrategy::Win,
            _ => unimplemented!(),
        }
    } 
}

impl Ord for GameThrow {
    fn cmp(&self, other: &Self) -> Ordering {
        match ( 3 + (*self as i32) - (*other as i32) ) % 3 {
            0 => Ordering::Equal,
            1 => Ordering::Greater,
            2 => Ordering::Less,
            _ => unreachable!(),
        }
    }
}

impl Add<i32> for GameThrow {
    type Output = GameThrow;
    fn add(self, other: i32) -> Self::Output {
        match other.rem_euclid(3) {
            1 => match self {
                GameThrow::Rock => GameThrow::Scissors,
                GameThrow::Scissors => GameThrow::Paper,
                GameThrow::Paper => GameThrow::Rock,
            },
            2 => match self {
                GameThrow::Rock => GameThrow::Paper,
                GameThrow::Scissors => GameThrow::Rock,
                GameThrow::Paper => GameThrow::Scissors,
            },
            0 => self,
            _ => unreachable!()
        }
    }
}

impl Sub<i32> for GameThrow {
    type Output = GameThrow;
    fn sub(self, other: i32) -> Self::Output {
        match other.rem_euclid(3) {
            2 => match self {
                GameThrow::Rock => GameThrow::Scissors,
                GameThrow::Scissors => GameThrow::Paper,
                GameThrow::Paper => GameThrow::Rock,
            },
            1 => match self {
                GameThrow::Rock => GameThrow::Paper,
                GameThrow::Scissors => GameThrow::Rock,
                GameThrow::Paper => GameThrow::Scissors,
            },
            0 => self,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct GameOutcomePair { ours: GameThrow, others: GameThrow }

#[derive(Debug, Copy, Clone)]
struct GameOutcomeStrategy { ours: GameStrategy, others: GameThrow }

impl From<&Vec<String>> for GameOutcomePair {
    fn from(w : &Vec<String>) -> Self {
        let mut it = w.iter();
        GameOutcomePair {
            others: GameThrow::from(it.next().unwrap()),
            ours: GameThrow::from(it.next().unwrap())
        }
    }
}

impl From<&Vec<String>> for GameOutcomeStrategy {
    fn from(w : &Vec<String>) -> Self {
        let mut it = w.iter();
        GameOutcomeStrategy {
            others: GameThrow::from(it.next().unwrap()),
            ours: GameStrategy::from(it.next().unwrap())
        }
    }
}

impl GameOutcomePair {
    fn score(&self) -> i32 {
        let res : i32 = self.ours as i32;
        match self.ours.cmp(&self.others) {
            Ordering::Greater => res + 1 + 6,
            Ordering::Equal => res + 1 + 3,
            Ordering::Less => res + 1 + 0,
        }
    }
}

impl GameOutcomeStrategy {
    fn resolve(&self) -> GameOutcomePair {
       GameOutcomePair {
            others: self.others,
            ours: self.others + self.ours as i32 
       }
    }
}


fn main() -> Result<(),io::Error> {
    if let Some(filename) = args().skip(1).next() {
        let parsed : Vec<Vec<String>> = BufReader::new(File::open(filename)?)
            .lines()
            .take_while(|l| l.is_ok())
            .map(|r| r.unwrap())
            .map(|l|
                l.split_whitespace().take(2)
                    .map(|w| w.to_string())
                    .collect::<Vec<String>>()
            ).collect();

        let part1 : i32 =
            parsed.iter()
                .map(|w| GameOutcomePair::from(w))
                .map(|res| res.score())
                .sum();
        println!("{}", part1);

        let part2 : i32 =
            parsed.iter()
                .map(|w| GameOutcomeStrategy::from(w))
                .map(|st| st.resolve())
                .map(|res| res.score())
                .sum();
        println!("{:?}", part2);

    } else {
        println!("missing filename");
    }
    Ok(())
}

