use crate::container::*;
use crate::Round;
use rand::prelude::*;

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
        self.rounds()
            .into_iter()
            .filter(|&other| other == r)
            .collect()
    }

    /// Does this round exist in the container?
    fn has_round(&self, r: &Round) -> bool {
        self.rounds().iter().any(|&other| other == r)
    }

    /// Returns a random player, if number of players is greater than 0.
    fn get_random_round(&self, rng: &mut ThreadRng) -> Option<&Round> {
        self.rounds()
            .get(rng.gen_range(0..self.num_rounds()))
            .copied()
    }

    /// Swaps two random players in a random round.
    fn swap_random_players_random_round(self, rng: &mut ThreadRng) -> Self {
        let mut round = self.get_random_round(rng).unwrap();
        let randomized = round.clone().swap_random_players(rng);
        let _ = std::mem::replace(&mut round, &randomized);
        self
    }

    /// Swaps two random partners in a random round.
    fn swap_random_partners_random_round(self, rng: &mut ThreadRng) -> Self {
        let mut round = self.get_random_round(rng).unwrap();
        let randomized = round.clone().swap_random_partners(rng);
        let _ = std::mem::replace(&mut round, &randomized);
        self
    }

    /// Replaces a reference to a round with a new one that is consumed.
    fn replace_round_with(self, replace: &Round, with: Round) -> Self {
        let mut round = self.find_round(replace).unwrap();
        round = &with;
        self
    }
}

impl RoundContainer for Vec<&Round> {
    fn rounds(&self) -> Vec<&Round> {
        self.to_vec()
    }
}
