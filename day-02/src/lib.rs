pub(crate) mod parse;

use std::cmp::max;
use std::str::FromStr;
use crate::parse::NomError;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Cubes {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Cubes {
    pub fn rgb(red: u8, green: u8, blue: u8) -> Cubes {
        Cubes { red, green, blue }
    }

    // Partial order <=
    fn le(&self, other: &Self) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }

    fn max_elems(a: &Self, b: &Self) -> Self {
        Cubes {
            red: max(a.red, b.red),
            green: max(a.green, b.green),
            blue: max(a.blue, b.blue),
        }
    }

    pub fn power(&self) -> u64 {
        (self.red as u64) * (self.green as u64) * (self.blue as u64)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    pub id: u64,
    draws: Vec<Cubes>,
}

impl Game {
    pub fn new(id: u64, draws: Vec<Cubes>) -> Game {
        Self { id, draws }
    }

    pub fn possible(&self, all_cubes: Cubes) -> bool {
        self.draws.iter().all(|cubes| cubes.le(&all_cubes))
    }

    pub fn min_cubes(&self) -> Cubes {
        self.draws.iter()
            .fold(Cubes::default(), |ref a, b| Cubes::max_elems(a, b)) }
}

impl FromStr for Game {
    type Err = NomError<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse::game(s) {
            Ok((_input, game)) => Ok(game),
            Err(e) => Err(e.to_owned()),
        }
    }
}
