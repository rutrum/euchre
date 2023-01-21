use crate::container::*;
use crate::{Game, Partners, Player, Round};
use crate::{Sort};
use rand::prelude::*;
use std::fmt::{Display, Formatter};

#[derive(Clone, Eq, PartialEq)]
pub struct Schedule<'a> {
    pub rounds: Vec<Round<'a>>,
}

impl<'a> Schedule<'a> {

    pub fn new_random(players: &Vec<&'a Player>, rng: &mut ThreadRng) -> Schedule<'a> {
        let num_players = players.len();
        let num_rounds = num_players - 1 + (num_players % 4);
        let rounds = (0..num_rounds)
                .map(|_| {
                    let mut shuffled = players.clone();
                    shuffled.shuffle(rng);
                    Round::from_players(shuffled)
                })
                .collect();
        Schedule { rounds }
    }

    /*
    pub fn new_in_order(num_players: usize) -> Schedule<'a> {
        let players = Player::many(num_players as i32, 1);
        let player_refs: Vec<&Player> = players.iter().collect();
        let num_rounds = num_players - 1 + (num_players % 4);
        let rounds = (0..num_rounds)
                .map(|_| Round::from_players(player_refs.clone()))
                .collect();
        Schedule { rounds }
    }

    pub fn new_random_order(num_players: usize, rng: &mut ThreadRng) -> Schedule<'a> {
        let players = Player::many(num_players as i32, 1);
        let player_refs: Vec<&Player> = players.iter().collect();
        let num_rounds = num_players - 1 + (num_players % 4);
        let rounds = 
            (0..num_rounds)
                .map(|_| {
                    let mut shuffled = player_refs.clone();
                    shuffled.shuffle(rng);
                    Round::from_players(shuffled)
                })
                .collect();
        Schedule { rounds }
    }
    */
}

impl<'a> PlayerContainer for Schedule<'a> {
    fn players(&self) -> Vec<&Player> {
        self.rounds.iter().flat_map(|r| r.players()).collect()
    }
}

impl<'a> PartnersContainer for Schedule<'a> {
    fn partners(&self) -> Vec<&Partners> {
        self.rounds.iter().flat_map(|round| round.partners()).collect()
    }
}

impl<'a> GameContainer for Schedule<'a> {
    fn games(&self) -> Vec<&Game> {
        self.rounds.iter().flat_map(|round| round.games()).collect()
    }
}

impl<'a> RoundContainer for Schedule<'a> {
    fn rounds(&self) -> Vec<&Round> {
        self.rounds.iter().collect()
    }
}

impl<'a> Display for Schedule<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for (i, round) in self.rounds.iter().enumerate() {
            writeln!(f, "Round {}: {}", i, round)?;
        }
        Ok(())
    }
}

impl<'a> Sort for Schedule<'a> {
    fn sort(mut self) -> Self {
        self.rounds.sort();
        self
    }
}

/*
impl<'a> Serialization for Schedule<'a> {
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
*/
