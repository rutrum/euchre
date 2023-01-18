//! For evaluating rotation charts to determine how "faulty" they are

use crate::{RotationChart, Player};
use itertools::Itertools;


/// returns the number of games any player had a duplicate partner
fn same_partner(rc: &RotationChart) -> i32 {

}

/// returns the number of games the provided player had a duplicate partner
fn same_partner_for_player(rc: &RotationChart, p: Player) -> i32 {
    let games = rc.games_with_player(p);
    let total_games = games.len();
    let different_partners = games.iter()
        .map(|game| game.partner_of(p))
        .unique()
        .count();
    total_games as i32 - different_partners as i32
}
