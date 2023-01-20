//! For evaluating rotation charts to determine how "faulty" they are

use crate::container::*;
use crate::{Player, Schedule};
use itertools::Itertools;

/// returns the number of games any player had a duplicate partner
pub fn same_partner(schedule: &Schedule) -> i32 {
    schedule
        .players()
        .iter()
        .map(|p| same_partner_for_player(schedule, p))
        .sum::<i32>()
        / 2 // since any duplicates are counted twice
}

/// returns the number of games the provided player had a duplicate partner
fn same_partner_for_player(schedule: &Schedule, p: &Player) -> i32 {
    let games = schedule.find_all_games_from_player(p);
    let total_games = games.len();
    let different_partners = games.iter().map(|game| game.partner_of(p)).unique().count();
    total_games as i32 - different_partners as i32
}

/// returns the number of times any player had the same opponent past their 2nd time
pub fn twice_repeated_opponents(schedule: &Schedule) -> i32 {
    schedule
        .players()
        .iter()
        .map(|p| twice_repeated_opponents_for_player(schedule, p))
        .sum::<i32>()
        / 2
}

/// returns the number of times a player had an opponent past their 2nd time
fn twice_repeated_opponents_for_player(schedule: &Schedule, p: &Player) -> i32 {
    let games = schedule.find_all_games_from_player(p);
    let counts = games
        .into_iter()
        .map(|g| g.find_opponents(p).unwrap())
        .flat_map(|partners| partners.players())
        .counts();
    schedule
        .players()
        .iter()
        .map(|p| *counts.get(p).unwrap_or(&0) as i32)
        .map(|c| (c - 2).max(0))
        .sum()
}

/// Returns the number of times a player had more byes than necessary
pub fn unequal_byes(schedule: &Schedule) -> i32 {
    let num_sitting_out = schedule.rounds()[0].byes.len() as i32;
    let total_byes = schedule
        .rounds()
        .iter()
        .flat_map(|g| g.byes.clone())
        .counts();
    schedule
        .players()
        .iter()
        .map(|p| *total_byes.get(p).unwrap_or(&0) as i32)
        .map(|c| (c - num_sitting_out).max(0))
        .sum()
}

pub fn total(schedule: &Schedule) -> i32 {
    unequal_byes(schedule) + twice_repeated_opponents(schedule) + same_partner(schedule)
}
