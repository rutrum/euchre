use crate::{Player, Game, Partners};
use std::fmt::{Display, Formatter};
use crate::{Sort, Serialization};
use crate::container::*;
use itertools::Itertools;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Round {
    pub games: Vec<Game>,
    pub byes: Vec<Player>,
}

impl Round {
    /// Creates a new round with given games and no byes.
    pub fn new(games: Vec<Game>) -> Round {
        Round { games, byes: Vec::new() }
    }

    /// Creates a new round with given games and given byes.
    pub fn with_byes(games: Vec<Game>, byes: Vec<Player>) -> Round {
        Round { games, byes }
    }
}

impl PlayerContainer for Vec<Player> {
    fn players(&self) -> Vec<&Player> {
        self.iter().collect()
    }

    fn from_players(players: Vec<Player>) -> Self {
        players
    }
}

impl PlayerContainer for Round {
    fn players(&self) -> Vec<&Player> {
        let mut players = self.games.iter().flat_map(|x| x.players()).collect::<Vec<&Player>>();
        players.extend(&self.byes);
        players
    }

    fn from_players(players: Vec<Player>) -> Self {
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

impl PartnersContainer for Round {
    fn partners(&self) -> Vec<&Partners> {
        self.games.iter().flat_map(|g| g.partners()).collect()
    }
}

impl GameContainer for Round {
    fn games(&self) -> Vec<&Game> {
        self.games.iter().collect()
    }
}

impl RoundContainer for Round {
    fn rounds(&self) -> Vec<&Round> {
        vec![self]
    }
}

impl Display for Round {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result { 
        let games_str = self.games.iter()
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

impl Sort for Round {
    fn sort(mut self) -> Self {
        self.games.sort();
        self.byes.sort();
        self
    }
}

impl Serialization for Round {
    fn serialize(self) -> String {
        let games_string = self.games.iter()
            .map(|g| g.serialize())
            .join(",");
        let byes_string = self.byes.iter()
            .map(|p| p.serialize())
            .join("+");
        format!("{};{}", games_string, byes_string)
    }

    fn deserialize(s: String) -> Result<Self, ()> {
        let parts = s.split(';')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let games_s = parts.get(0).ok_or(())?;
        let byes_s = parts.get(0).ok_or(())?;

        let games = games_s.split(',')
            .map(|s| Game::deserialize(s.to_string()))
            .collect::<Result<Vec<Game>, ()>>()?;
        let byes = byes_s.split('+')
            .map(|s| Player::deserialize(s.to_string()))
            .collect::<Result<Vec<Player>, ()>>()?;

        Ok(Self::with_byes(games, byes))
    }
}
