use crate::container::*;
use crate::{Game, Partners, Player, Round};
use crate::{Serialization, Sort};
use itertools::Itertools;
use rand::prelude::*;
use std::fmt::{Display, Formatter};

#[derive(Clone, Eq, PartialEq)]
pub struct Schedule {
    pub players: Vec<Player>,
    pub rounds: Vec<Round>,
}

impl Schedule {
    pub fn new_in_order(num_players: usize) -> Schedule {
        let players = Player::many(num_players as i32, 1);
        let num_rounds = num_players - 1 + (num_players % 4);
        let rounds = (0..num_rounds)
                .map(|_| Round::from_players(players.clone()))
                .collect();
        Schedule { players, rounds }
    }

    pub fn new_random_order(num_players: usize, rng: &mut ThreadRng) -> Schedule {
        let players = Player::many(num_players as i32, 1);
        let num_rounds = num_players - 1 + (num_players % 4);
        let rounds = 
            (0..num_rounds)
                .map(|_| {
                    let mut shuffled = players.clone();
                    shuffled.shuffle(rng);
                    Round::from_players(shuffled)
                })
                .collect();
        Schedule { players, rounds }
    }
}

impl PlayerContainer for Schedule {
    fn players(&self) -> Vec<&Player> {
        self.rounds.iter().flat_map(|r| r.players()).collect()
    }
}

impl PartnersContainer for Schedule {
    fn partners(&self) -> Vec<&Partners> {
        self.rounds.iter().flat_map(|round| round.partners()).collect()
    }
}

impl GameContainer for Schedule {
    fn games(&self) -> Vec<&Game> {
        self.rounds.iter().flat_map(|round| round.games()).collect()
    }
}

impl RoundContainer for Schedule {
    fn rounds(&self) -> Vec<&Round> {
        self.rounds.iter().collect()
    }
}

impl Display for Schedule {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for (i, round) in self.rounds.iter().enumerate() {
            writeln!(f, "Round {}: {}", i, round)?;
        }
        Ok(())
    }
}

impl Sort for Schedule {
    fn sort(mut self) -> Self {
        self.rounds.sort();
        self
    }
}

impl Serialization for Schedule {
    fn serialize(self) -> String {
        self.rounds
            .iter()
            .map(|round| round.clone().serialize())
            .join("\n")
    }

    fn deserialize(s: String) -> Result<Self, ()> {
        todo!();
    }
}
