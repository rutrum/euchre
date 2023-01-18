use std::fmt::{Display, Formatter};

use crate::Player;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Partners(pub Player, pub Player);

impl Partners {
    pub fn new(p1: Player, p2: Player) -> Self {
        if p1 < p2 {
            Partners(p1, p2)
        } else {
            Partners(p2, p1)
        }
    }

    pub fn has_player(&self, p: Player) -> bool {
        self.0 == p || self.1 == p
    }

    pub fn partner_of(&self, p: Player) -> Option<Player> {
        if self.0 == p {
            Some(self.1)
        } else if self.1 == p {
            Some(self.0)
        } else {
            None
        }
    }
}

impl From<String> for Partners {
    /// Converts "3+5" into Player(3) and Player(5)
    fn from(s: String) -> Self {
        let ints: Vec<i32> = s.split("+")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse().unwrap())
            .collect();
        return Self::new(Player(ints[0]), Player(ints[1]));
    }
}

impl Display for Partners {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result { 
        write!(f, "{}{}", self.0, self.1)
    }
}
