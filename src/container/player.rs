use crate::Partners;
use crate::{Game, Player, Round};
use rand::prelude::*;
use std::cmp::Ordering;

pub trait PlayerContainer: Sized {
    /// Returns a list of players.
    fn players(&self) -> Vec<&Player>;

    /// Returns the total number of players
    fn total_players(&self) -> usize {
        self.players().len()
    }

    /// Returns a reference to player equal to the given player.
    fn find_player(&self, p: &Player) -> Option<&Player> {
        self.players().into_iter().find(|&other| other == p)
    }

    /// Returns a vector with all players equal to given player.
    fn find_all_players(&self, p: &Player) -> Vec<&Player> {
        self.players()
            .into_iter()
            .filter(|&other| other == p)
            .collect()
    }

    /// Returns if a player is equal to the given player.
    fn has_player(&self, p: &Player) -> bool {
        self.players().iter().any(|&other| other == p)
    }

    /// Tries to swap two players if they exist in the container.  If
    /// neither are in the container, nothing happens.
    fn swap_players(self, p1: &Player, p2: &Player) -> Self {
        let mut found_players = self.players().into_iter().filter(|p| **p == *p1 || **p == *p2);
        if let Some(mut p1) = found_players.next() {
            if let Some(mut p2) = found_players.next() {
                std::mem::swap(&mut p1, &mut p2);
            }
        }
        self
    }

    /// Returns a random player, if number of players is greater than 0.
    fn get_random_player(&self, rng: &mut ThreadRng) -> Option<&Player> {
        self.players()
            .get(rng.gen_range(0..self.total_players()))
            .copied()
    }

    /// Returns a random player that isn't provided.  Returns None if there is no such other player.
    fn get_random_player_not(&self, rng: &mut ThreadRng, p: &Player) -> Option<&Player> {
        self.players()
            .into_iter()
            .filter(|other| *other != p)
            .collect::<Vec<&Player>>()
            .get(rng.gen_range(0..self.total_players()))
            .copied()
    }

    /// Returns a random player that isn't provided.  Returns None if there is no such other player.
    fn get_random_player_not_any(&self, rng: &mut ThreadRng, ps: &[Player]) -> Option<&Player> {
        self.players()
            .into_iter()
            .filter(|other| !ps.contains(other))
            .collect::<Vec<&Player>>()
            .get(rng.gen_range(0..self.total_players()))
            .copied()
    }

    /// Returns self with two players swapped in memory.
    fn swap_random_players(self, rng: &mut ThreadRng) -> Self {
        let first = rng.gen_range(0..self.total_players());
        let second = rng.gen_range(0..self.total_players());
        let mut players = self.players();
        match first.cmp(&second) {
            Ordering::Less => {
                let (left, right) = players.split_at_mut(second);
                std::mem::swap(&mut left[first], &mut right[0]);
            }
            Ordering::Greater => {
                let (left, right) = players.split_at_mut(first);
                std::mem::swap(&mut left[second], &mut right[0]);
            }
            Ordering::Equal => {}
        }
        self
    }
}

impl PlayerContainer for Vec<&Player> {
    fn players(&self) -> Vec<&Player> {
        self.to_vec()
    }
}

impl PlayerContainer for Vec<Player> {
    fn players(&self) -> Vec<&Player> {
        self.iter().collect()
    }
}

impl<'a> PlayerContainer for Vec<&Partners<'a>> {
    fn players(&self) -> Vec<&Player> {
        self.iter().flat_map(|&x| x.players()).collect()
    }
}

impl<'a> PlayerContainer for Vec<&Game<'a>> {
    fn players(&self) -> Vec<&Player> {
        self.iter().flat_map(|&x| x.players()).collect()
    }
}

impl<'a> PlayerContainer for Vec<&Round<'a>> {
    fn players(&self) -> Vec<&Player> {
        self.iter().flat_map(|&x| x.players()).collect()
    }
}
