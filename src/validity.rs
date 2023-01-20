//! For evaluating if a certain configuration is even valid, which
//! is making sure that there aren't duplicate player numbers

use crate::PlayerContainer;
use crate::{Game, Partners, Round};

pub fn partners(pair: Partners) -> bool {
    pair.0 != pair.1
}

pub fn game(game: Game) -> bool {
    partners(game.0) && partners(game.1) && game.0 .0 != game.1 .0
}

pub fn round(round: Round) -> bool {
    let players = round.players();
    players.iter().zip(players.iter()).all(|(a, b)| a != b)
}
