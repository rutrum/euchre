use std::fmt::{Display, Formatter};
use crate::{Partners, Player};

#[macro_export]
macro_rules! game {
    ($a:ident, $b:ident, $c:ident, $d:ident) => {
        Game::new(Partners::new($a, $b), Partners::new($c, $d))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Game(pub Partners, pub Partners);

impl Game {
    pub fn new(p1: Partners, p2: Partners) -> Game {
        if p1 < p2 {
            Game(p1, p2)
        } else {
            Game(p2, p1)
        }
    }

    pub fn has_player(&self, p: Player) -> bool {
        self.0.has_player(p) || self.1.has_player(p)
    }

    pub fn players(&self) -> Vec<Player> {
        vec![
            self.0.0,
            self.0.1,
            self.1.0,
            self.1.1,
        ]
    }

    pub fn partners(&self) -> Vec<Partners> {
        vec![self.0, self.1]
    }

    pub fn partner_of(&self, player: Player) -> Option<Player> {
        if self.0.has_player(player) {
            self.0.partner_of(player)
        } else if self.1.has_player(player) {
            self.1.partner_of(player)
        } else {
            None
        }
    }
}

impl From<String> for Game {
    /// Converts "3+5v1+2" into partners 3+5 vs 1+2
    fn from(s: String) -> Self {
        let partners: Vec<Partners> = s.split("v")
            .collect::<Vec<&str>>()
            .iter()
            .map(|&x| x.to_string().into())
            .collect();
        return Self::new(partners[0], partners[1]);
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result { 
        write!(f, "({}, {})", self.0, self.1)
    }
}

