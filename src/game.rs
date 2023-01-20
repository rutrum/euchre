use std::fmt::{Display, Formatter};
use crate::{Partners, Player};
use crate::{Sort, Serialization};
use crate::container::*;

#[macro_export]
macro_rules! game {
    ($a:ident, $b:ident, $c:ident, $d:ident) => {
        Game::new(Partners::new($a, $b), Partners::new($c, $d))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Game(pub Partners, pub Partners);

impl Game {
    /// Creates a new game with partners in sorted order.
    pub fn new(p1: Partners, p2: Partners) -> Game {
        let game = Game(p1, p2);
        game.sort()
    }

    /// Replaces `replace` with `with` if `replace` player is in the game.
    /// Also sorts the result after replacement.
    pub fn substitute_player(mut self, replace: Player, with: Player) -> Self {
        self.0 = self.0.substitute(replace, with);
        self.1 = self.1.substitute(replace, with);
        self.sort()
    }

    /// Replaces `replace` with `with` if `replace` partners is in the game.
    /// Also sorts the result after replacement.
    pub fn substitute_partners(mut self, replace: Partners, with: Partners) -> Self {
        if self.0 == replace {
            self.0 = with;
        } else if self.1 == replace {
            self.1 = with;
        }
        self.sort()
    }
}

impl PlayerContainer for Game {
    fn players(&self) -> Vec<&Player> {
        vec![
            &self.0.0,
            &self.0.1,
            &self.1.0,
            &self.1.1,
        ]
    }

    fn from_players(partners: Vec<Player>) -> Self {
        Game(
            Partners::from_players(partners[0..=1].to_vec()),
            Partners::from_players(partners[2..=3].to_vec()),
        )
    }
}

impl PartnersContainer for Game {
    fn partners(&self) -> Vec<&Partners> {
        vec![&self.0, &self.1]
    }
}

impl GameContainer for Game {
    fn games(&self) -> Vec<&Game> {
        vec![self]
    }
}

impl Serialization for Game {
    fn serialize(self) -> String {
        format!("{}v{}", self.0, self.1)
    }

    fn deserialize(s: String) -> Result<Self, ()> {
        let partners = s.split('+')
            .map(|s| Partners::deserialize(s.to_string()))
            .collect::<Result<Vec<Partners>, ()>>()?;
        Ok(Self::new(
            *partners.get(0).ok_or(())?,
            *partners.get(1).ok_or(())?,
        ))
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result { 
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Sort for Game {
    fn sort(mut self) -> Self {
        if self.1 < self.0 {
            std::mem::swap(&mut self.1, &mut self.0);
        }
        self
    }
}
