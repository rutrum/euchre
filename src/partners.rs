use std::fmt::{Display, Formatter};

use crate::container::*;
use crate::Player;
use crate::{Serialization, Sort};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, Hash, PartialOrd)]
pub struct Partners(pub Player, pub Player);

impl Partners {
    /// Creates a new pair of players in sorted order.
    pub fn new(p1: Player, p2: Player) -> Self {
        let partners = Partners(p1, p2);
        partners.sort()
    }

    /// Replaces `replace` with `with` if `replace` is in the pair.
    /// Also sorts the result after replacement.
    pub fn substitute(mut self, replace: Player, with: Player) -> Self {
        if self.0 == replace {
            self.0 = with;
        } else if self.1 == replace {
            self.1 = with;
        }
        self.sort()
    }
}

impl PlayerContainer for Partners {
    fn players(&self) -> Vec<&Player> {
        vec![&self.0, &self.1]
    }
}

impl PartnersContainer for Partners {
    fn partners(&self) -> Vec<&Partners> {
        vec![self]
    }
}

impl Display for Partners {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}+{}", self.0, self.1)
    }
}

impl Serialization for Partners {
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

impl Sort for Partners {
    fn sort(mut self) -> Self {
        if self.1 < self.0 {
            std::mem::swap(&mut self.1, &mut self.0)
        }
        self
    }
}
