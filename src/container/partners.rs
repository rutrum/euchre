use super::PlayerContainer;
use crate::{Game, Partners, Player, Round};
use rand::prelude::*;
use std::cmp::Ordering;

pub trait PartnersContainer: PlayerContainer {
    /// Returns a list of partners
    fn partners(&self) -> Vec<&Partners>;

    fn total_partners(&self) -> usize {
        self.partners().len()
    }

    /// Returns a reference to partners equal to the given partners.
    fn find_partners(&self, p: &Partners) -> Option<&Partners> {
        self.partners().into_iter().find(|&other| other == p)
    }

    /// Returns a vector with all partners equal to given partner.
    fn find_all_partners(&self, p: &Partners) -> Vec<&Partners> {
        self.partners()
            .into_iter()
            .filter(|&other| other == p)
            .collect()
    }

    /// Returns if a partners is equal to the given partners.
    fn has_partners(&self, p: &Partners) -> bool {
        self.partners().iter().any(|&other| other == p)
    }

    /// Returns the partner of a given player.  Returns None if that
    /// player is not in the game.
    fn partner_of(&self, p: &Player) -> Option<&Player> {
        self.partners()
            .iter()
            .find(|partners| partners.find_player(p).is_some())
            .and_then(|partners| {
                if partners.0 == *p {
                    Some(&partners.1)
                } else if partners.1 == *p {
                    Some(&partners.0)
                } else {
                    None
                }
            })
    }

    /// Returns the partners that contains the given player;
    fn find_partners_from_player(&self, p: &Player) -> Option<&Partners> {
        self.partners()
            .into_iter()
            .find(|partners| partners.has_player(p))
    }

    /// Swaps two random partners and consumes self.
    fn swap_random_partners(self, rng: &mut ThreadRng) -> Self {
        let first = rng.gen_range(0..self.total_partners());
        let second = rng.gen_range(0..self.total_partners());
        let mut partners = self.partners();
        match first.cmp(&second) {
            Ordering::Less => {
                let (left, right) = partners.split_at_mut(second);
                std::mem::swap(&mut left[first], &mut right[0]);
            }
            Ordering::Greater => {
                let (left, right) = partners.split_at_mut(first);
                std::mem::swap(&mut left[second], &mut right[0]);
            }
            Ordering::Equal => {}
        }
        self
    }
}

impl PartnersContainer for Vec<&Partners> {
    fn partners(&self) -> Vec<&Partners> {
        self.to_vec()
    }
}

impl PartnersContainer for Vec<&Game> {
    fn partners(&self) -> Vec<&Partners> {
        self.iter().flat_map(|&g| g.partners()).collect()
    }
}

impl PartnersContainer for Vec<&Round> {
    fn partners(&self) -> Vec<&Partners> {
        self.iter().flat_map(|&r| r.partners()).collect()
    }
}
