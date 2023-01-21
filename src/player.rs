use crate::container::*;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Player(pub i32);

impl Player {
    pub fn new_list(num: i32) -> Vec<Self> {
        Self::many(num, 1)
    }

    pub fn many(num: i32, offset: i32) -> Vec<Self> {
        (0..num).map(|i| Player(i + offset)).collect()
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
}

/*
impl Serialization for Player {
    fn serialize(self) -> String {
        self.0.to_string()
    }

    fn deserialize(s: String) -> Result<Self, ()> {
        let id = s.parse::<i32>().map_err(|_| ())?;
        Ok(Player(id))
    }
}
*/
