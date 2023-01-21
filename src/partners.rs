use std::fmt::{Display, Formatter};

use crate::container::*;
use crate::Player;
use crate::{Sort};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, Hash, PartialOrd)]
pub struct Partners<'a>(pub &'a Player, pub &'a Player);

impl<'a> Partners<'a> {
    /// Creates a new pair of players in sorted order.
    pub fn new(p1: &'a Player, p2: &'a Player) -> Self {
        let partners = Partners(p1, p2);
        partners.sort()
    }
}

impl<'a> PlayerContainer for Partners<'a> {
    fn players(&self) -> Vec<&Player> {
        vec![&self.0, &self.1]
    }
}

impl<'a> PartnersContainer for Partners<'a> {
    fn partners(&self) -> Vec<&Partners> {
        vec![self]
    }
}

impl<'a> Display for Partners<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}+{}", self.0, self.1)
    }
}

/*
impl<'a> Serialization for Partners<'a> {
    fn serialize(self) -> String {
        format!("{}+{}", self.0, self.1)
    }

    fn deserialize(s: String) -> Result<Self, ()> {
        let players = s
            .split('+')
            .map(|s| Player::deserialize(s.to_string()))
            .collect::<Result<Vec<Player>, ()>>()?;
        Ok(Self::new(
            *players.get(0).ok_or(())?,
            *players.get(1).ok_or(())?,
        ))
    }
}
*/

impl<'a> Sort for Partners<'a> {
    fn sort(mut self) -> Self {
        if self.1 < self.0 {
            std::mem::swap(&mut self.1, &mut self.0)
        }
        self
    }
}
