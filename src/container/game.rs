use super::{PartnersContainer, PlayerContainer};
use crate::{Game, Partners, Player, Round};
use itertools::Itertools;

pub trait GameContainer: PartnersContainer {
    /// Returns a list of games
    fn games(&self) -> Vec<&Game>;

    /// Returns the total number of games
    fn num_games(&self) -> usize {
        self.games().len()
    }

    /// Returns a reference to a game equal to the given game
    fn find_game(&self, g: &Game) -> Option<&Game> {
        self.games().into_iter().find(|&other| other == g)
    }

    /// Returns a vector with all games equal to given game.
    fn find_all_games(&self, g: &Game) -> Vec<&Game> {
        self.games()
            .into_iter()
            .filter(|&other| other == g)
            .collect()
    }

    /// Does this game exist in the container?
    fn has_game(&self, g: &Game) -> bool {
        self.games().iter().any(|&other| other == g)
    }

    /// Return the first game that contains the given player.
    fn find_game_from_player(&self, p: &Player) -> Option<&Game> {
        self.games().into_iter().find(|game| game.has_player(p))
    }

    /// Return a vector with all games that contain the given player.
    fn find_all_games_from_player(&self, p: &Player) -> Vec<&Game> {
        self.games()
            .into_iter()
            .filter(|game| game.has_player(p))
            .collect()
    }

    /// Return the first game that contains the given partner.
    fn find_game_from_partners(&self, p: &Partners) -> Option<&Game> {
        self.games().into_iter().find(|game| game.has_partners(p))
    }

    /// Return the first game that contains the given partner.
    fn find_all_games_from_partners(&self, p: &Partners) -> Vec<&Game> {
        self.games()
            .into_iter()
            .filter(|game| game.has_partners(p))
            .collect()
    }

    /// Return the opponents in the first game that contains the given player.
    fn find_opponents(&self, p: &Player) -> Option<&Partners> {
        self.find_game_from_player(p).and_then(|game| {
            game.partners()
                .into_iter()
                .find(|partners| !partners.has_player(p))
        })
    }

    /// Return the opponents in the first game that contains the given player.
    fn find_opponents_as_players(&self, p: &Player) -> Option<Vec<&Player>> {
        self.find_opponents(p).map(|partner| partner.players())
    }

    /// Return the opponents in the games that contains the given player.
    fn find_all_opponents(&self, p: &Player) -> Vec<&Partners> {
        self.find_all_games_from_player(p)
            .iter()
            .map(|game| game.find_opponents(p).unwrap())
            .collect()
    }

    /// Return the opponents in the games that contains the given player.
    fn find_all_opponents_as_players(&self, p: &Player) -> Vec<&Player> {
        self.find_all_opponents(p)
            .iter()
            .flat_map(|partner| partner.players())
            .collect()
    }
}

impl GameContainer for Vec<&Game> {
    fn games(&self) -> Vec<&Game> {
        self.to_vec()
    }
}

impl GameContainer for Vec<&Round> {
    fn games(&self) -> Vec<&Game> {
        self.iter().flat_map(|&x| x.games()).collect()
    }
}
