//! For evaluating if a certain configuration is even valid, which
//! is making sure that there aren't duplicate player numbers

use crate::{Partners, Game, Round};

fn valid_partners(pair: Partners) -> bool {
    pair.0 != pair.1
}

fn valid_game(game: Game) -> bool {
    valid_partners(game.0) && 
        valid_partners(game.1) && 
        game.0.0 != game.1.0
}

fn valid_round(round: Round) -> bool {
    let players = round.players();
    players.iter().zip(players.iter()).all(|(a, b)| a != b)
}
