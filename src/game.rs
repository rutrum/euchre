use crate::container::*;
use crate::{Partners, Player};
use crate::{Sort};
use std::fmt::{Display, Formatter};

#[macro_export]
macro_rules! game {
    ($a:ident, $b:ident, $c:ident, $d:ident) => {
        Game::new(Partners::new($a, $b), Partners::new($c, $d))
    };
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Game<'a>(pub Partners<'a>, pub Partners<'a>);

impl<'a> Game<'a> {
    /// Creates a new game with partners in sorted order.
    pub fn new(p1: Partners<'a>, p2: Partners<'a>) -> Game<'a> {
        let game = Game(p1, p2);
        game.sort()
    }
}

impl<'a> PlayerContainer for Game<'a> {
    fn players(&self) -> Vec<&Player> {
        vec![&self.0 .0, &self.0 .1, &self.1 .0, &self.1 .1]
    }
}

impl<'a> PartnersContainer for Game<'a> {
    fn partners(&self) -> Vec<&Partners> {
        vec![&self.0, &self.1]
    }
}

impl<'a> GameContainer for Game<'a> {
    fn games(&self) -> Vec<&Game> {
        vec![self]
    }
}

/*
impl<'a> Serialization for Game<'a> {
    fn serialize(self) -> String {
        format!("{}v{}", self.0, self.1)
    }

    fn deserialize(s: String) -> Result<Self, ()> {
        let partners = s
            .split('+')
            .map(|s| Partners::deserialize(s.to_string()))
            .collect::<Result<Vec<Partners>, ()>>()?;
        Ok(Self::new(
            partners.get(0).ok_or(())?,
            partners.get(1).ok_or(())?,
        ))
    }
}
*/

impl<'a> Display for Game<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl<'a> Sort for Game<'a> {
    fn sort(mut self) -> Self {
        if self.1 < self.0 {
            std::mem::swap(&mut self.1, &mut self.0);
        }
        self
    }
}
