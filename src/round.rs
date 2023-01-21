use crate::container::*;
use crate::{Game, Partners, Player};
use crate::{Sort};
use itertools::Itertools;
use rand::prelude::*;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Round<'a> {
    pub games: Vec<Game<'a>>,
    pub byes: Vec<&'a Player>,
}

impl<'a> Round<'a> {
    /// Creates a new round with given games and no byes.
    pub fn new(games: Vec<Game<'a>>) -> Self {
        Round {
            games,
            byes: Vec::new(),
        }
    }

    /// Creates a new round with given games and given byes.
    pub fn with_byes(games: Vec<Game<'a>>, byes: Vec<&'a Player>) -> Self {
        Round { games, byes }
    }

    /*  // no sense anymore, since I don't own players
    /// Creates a new random round with `num_players`.
    pub fn new_random(num_players: usize, rng: &mut ThreadRng) -> Self {
        let mut players = Player::many(num_players as i32, 1);
        players.shuffle(rng);
        Self::from_players(players)
    }
    */

    /// Creates a round from a list of players.
    pub fn from_players(players: Vec<&'a Player>) -> Self {
        let mut games = vec![];
        let mut table = vec![];
        for player in players {
            table.push(player);
            if table.len() == 4 {
                games.push(Game(
                    Partners(table[0], table[1]),
                    Partners(table[2], table[3]),
                ));
                table = vec![];
            }
        }

        Round { games, byes: table }
    }
}

impl<'a> PlayerContainer for Round<'a> {
    fn players(&self) -> Vec<&Player> {
        self.games.iter()
            .flat_map(|x| x.players())
            .chain(self.byes.clone().into_iter()) // cloning the list, not the data in the list
            .collect()
    }
}

impl<'a> PartnersContainer for Round<'a> {
    fn partners(&self) -> Vec<&Partners> {
        self.games.iter().flat_map(|g| g.partners()).collect()
    }
}

impl<'a> GameContainer for Round<'a> {
    fn games(&self) -> Vec<&Game> {
        self.games.iter().collect()
    }
}

impl<'a> RoundContainer for Round<'a> {
    fn rounds(&self) -> Vec<&Round> {
        vec![self]
    }
}

impl<'a> Display for Round<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let games_str = self
            .games
            .iter()
            .map(|g| format!("{}", g))
            .collect::<Vec<String>>()
            .join(" ");
        if self.byes.is_empty() {
            write!(f, "{}", games_str)
        } else {
            write!(f, "{}; {:?}", games_str, self.byes)
        }
    }
}

impl<'a> Sort for Round<'a> {
    fn sort(mut self) -> Self {
        self.games.sort();
        self.byes.sort();
        self
    }
}

/*
impl<'a> Serialization for Round<'a> {
    fn serialize(self) -> String {
        let games_string = self.games.iter().map(|g| g.serialize()).join(",");
        let byes_string = self.byes.iter().map(|p| p.serialize()).join("+");
        format!("{};{}", games_string, byes_string)
    }

    fn deserialize(s: String) -> Result<Self, ()> {
        let parts = s.split(';').map(|s| s.to_string()).collect::<Vec<String>>();

        let games_s = parts.get(0).ok_or(())?;
        let byes_s = parts.get(0).ok_or(())?;

        let games = games_s
            .split(',')
            .map(|s| Game::deserialize(s.to_string()))
            .collect::<Result<Vec<Game>, ()>>()?;
        let byes = byes_s
            .split('+')
            .map(|s| Player::deserialize(s.to_string()))
            .collect::<Result<Vec<Player>, ()>>()?;

        Ok(Self::with_byes(games, byes))
    }
}
*/
