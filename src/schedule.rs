use crate::{Player, Partners, Game, Round};
use std::fmt::{Display, Formatter};
use crate::container::*;
use crate::{Sort, Serialization};
use itertools::Itertools;
use rand::prelude::*;

#[derive(Clone, Eq, PartialEq)]
pub struct Schedule(pub Vec<Round>);

impl Schedule {
    pub fn new_in_order(num_players: usize) -> Schedule {
        let players = Player::many(num_players as i32, 1);
        let num_rounds = num_players - 1 + (num_players % 4);
        Schedule(
            (0..num_rounds)
                .map(|_| Round::from_players(players.clone()))
                .collect()
        )
    }

    pub fn new_random_order(num_players: usize, rng: &mut ThreadRng) -> Schedule {
        let players = Player::many(num_players as i32, 1);
        let num_rounds = num_players - 1 + (num_players % 4);
        Schedule(
            (0..num_rounds)
            .map(|_| {
                let mut shuffled = players.clone();
                shuffled.shuffle(rng);
                Round::from_players(shuffled)
            })
                .collect()
        )
    }

    pub fn players(&self) -> Vec<&Player> {
        self.0[0].players()
    }
}

/*
impl PlayerContainer for Schedule {
    fn players(&self) -> Vec<&Player> {
        self.0[0].players()
    }
}
*/

impl PartnersContainer for Schedule {
    fn partners(&self) -> Vec<&Partners> {
        self.0.iter().flat_map(|round| round.partners()).collect()
    }
}

impl GameContainer for Schedule {
    fn games(&self) -> Vec<&Game> {
        self.0.iter().flat_map(|round| round.games()).collect()
    }
}

impl RoundContainer for Schedule {
    fn rounds(&self) -> Vec<&Round> {
        self.0.iter().collect()
    }
}

impl Display for Schedule {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result { 
        for (i, round) in self.0.iter().enumerate() {
            writeln!(f, "Round {}: {}", i, round)?;
        }
        Ok(())
    }
}

impl Sort for Schedule {
    fn sort(mut self) -> Self {
        self.0.sort();
        self
    }
}

impl Serialization for Schedule {
    fn serialize(self) -> String {
        self.0.iter()
            .map(|round| {
                round.clone().serialize()
            })
            .join("\n")
    }

    fn deserialize(s: String) -> Result<Self, ()> {
        todo!();
    }
}
