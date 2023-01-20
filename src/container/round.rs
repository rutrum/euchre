use crate::Round;
use super::{GameContainer, PlayerContainer};
use rand::prelude::*;
use crate::container::*;

pub trait RoundContainer: GameContainer {
    /// Returns a list of rounds.
    fn rounds(&self) -> Vec<&Round>;

    /// Returns the number of rounds in this container.
    fn num_rounds(&self) -> usize {
        self.rounds().len()
    }

    /// Returns a reference to a round equal to the given round
    fn find_round(&self, r: &Round) -> Option<&Round> {
        self.rounds().into_iter().find(|&other| other == r)
    }

    /// Returns a vector with all rounds equal to given round.
    fn find_all_rounds(&self, r: &Round) -> Vec<&Round> {
        self.rounds().into_iter().filter(|&other| other == r).collect()
    }

    /// Does this round exist in the container?
    fn has_round(&self, r: &Round) -> bool {
        self.rounds().iter().any(|&other| other == r)
    }

    /// Returns a random player, if number of players is greater than 0.
    fn get_random_round(&self, rng: &mut ThreadRng) -> Option<&Round> {
        todo!();
        self.rounds()
            .get(rng.gen_range(0..self.num_rounds()))
            .copied()
    }

    fn swap_random_players_random_round(self, rng: &mut ThreadRng) -> Self {
        let total = self.num_rounds();
        let mut rounds = self.rounds();
        let n = rng.gen_range(0..total);
        let copy = rounds[n].clone();
        let randomized_round = copy.swap_random_players(rng);
        let _ = std::mem::replace(&mut rounds[n], &randomized_round);
        self
    }

    fn swap_random_partners_random_round(self, rng: &mut ThreadRng) -> Self {
        let total = self.num_rounds();
        let mut rounds = self.rounds();
        let n = rng.gen_range(0..total);
        let copy = rounds[n].clone();
        let randomized_round = copy.swap_random_partners(rng);
        let _ = std::mem::replace(&mut rounds[n], &randomized_round);
        self
    }
}
