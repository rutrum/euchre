use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd)]
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
