// model modules
mod game;
mod partners;
mod player;
mod round;
mod schedule;

pub mod container;

// functional modules
pub mod error;
pub mod validity;

pub use game::Game;
pub use partners::Partners;
pub use player::Player;
pub use round::Round;
pub use schedule::Schedule;

use crate::container::*;

/*
/// Transforming data structure into a string and back
pub trait Serialization {
    fn serialize(self) -> String;
    fn deserialize(s: String) -> Result<Self, ()>
    where
        Self: Sized;
}
*/

/// Ensures that data is ordered in a standard manner
pub trait Sort {
    fn sort(self) -> Self;
}

/*
pub fn trivial4(players: Vec<Player>) -> Schedule {
    let (a, b, c, d) = (players[0], players[1], players[2], players[3]);
    let r1 = Round::new(vec![game!(a, b, c, d)]);
    let r2 = Round::new(vec![game!(a, c, b, d)]);
    let r3 = Round::new(vec![game!(a, d, b, c)]);

    Schedule(vec![r1, r2, r3])
}

pub fn chart8() -> Schedule {
    let t1 = trivial4(Player::many(4, 1));
    let t2 = trivial4(Player::many(4, 5));

    let both_rounds = t1.0.iter().zip(t2.0.iter());

    let mut round_partners: Vec<Vec<Partners>> = both_rounds
        .map(|(a, b)| {
            let mut partners = Vec::new();
            partners.extend(a.partners());
            partners.extend(b.partners());
            partners
        })
        .collect();

    let t1_players = t1.players();
    let t2_players = t2.players();

    for offset in 0..4 {
        let mut partners = Vec::new();
        for i in 0..4 {
            let left = t1_players[i];
            let right = t2_players[(i + offset) % 4];
            partners.push(Partners::new(*left, *right));
        }
        round_partners.push(partners);
    }

    let rounds = round_partners
        .iter()
        .map(|l| {
            let g1 = Game::new(l[0], l[1]);
            let g2 = Game::new(l[2], l[3]);
            Round::new(vec![g1, g2])
        })
        .collect();

    Schedule(rounds)
}
*/
