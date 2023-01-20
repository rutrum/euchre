use crate::Serialization;
use std::fmt::{Display, Formatter};
use crate::container::*;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Player(pub i32);

impl Player {
    pub fn many(num: i32, offset: i32) -> Vec<Player> {
        (0..num).map(|i| Player(i+offset)).collect()
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result { 
        write!(f, "{}", self.0)
    }
}

impl PlayerContainer for Player {
    fn players(&self) -> Vec<&Player> {
        vec![self]
    }

    fn from_players(players: Vec<Player>) -> Self {
        players[0]
    }
}

impl Serialization for Player {
    fn serialize(self) -> String {
        self.0.to_string()
    }

    fn deserialize(s: String) -> Result<Self, ()> {
        let id = s.parse::<i32>().map_err(|_| ())?;
        Ok(Player(id))
    }
}
