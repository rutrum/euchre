use std::fmt::{Display, Formatter};

mod player;
mod partners;
mod game;
mod error;
mod validity;

pub use player::Player;
pub use partners::Partners;
pub use game::Game;

pub fn trivial4(players: Vec<Player>) -> RotationChart {
    let (a, b, c, d) = (players[0], players[1], players[2], players[3]);
    let r1 = Round::new(vec![game!(a, b, c, d)]);
    let r2 = Round::new(vec![game!(a, c, b, d)]);
    let r3 = Round::new(vec![game!(a, d, b, c)]);

    return RotationChart(vec![r1, r2, r3]);
}

pub fn chart8() -> RotationChart {
    let t1 = trivial4(Player::many(4, 1));
    let t2 = trivial4(Player::many(4, 5));

    let both_rounds = t1.0.iter().zip(t2.0.iter());

    let mut round_partners: Vec<Vec<Partners>> = both_rounds.map(|(a, b)| {
        let mut partners = Vec::new();
        partners.extend(a.partners());
        partners.extend(b.partners());
        partners
    }).collect();

    let t1_players = t1.players();
    let t2_players = t2.players();

    for offset in 0..4 {
        let mut partners = Vec::new();
        for i in 0..4 {
            let left = t1_players[i];
            let right = t2_players[(i+offset)%4];
            partners.push(Partners::new(left, right));
        }
        round_partners.push(partners);
    }

    print_partners(round_partners);

    t1
}

fn print_partners(partners: Vec<Vec<Partners>>) {
    let s = partners.iter()
        .map(|g| {
            g.iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<String>>()
            .join(", ")
        }).collect::<Vec<String>>();
    for (i, game) in s.iter().enumerate() {
        println!("Round {}: {}", i, game);
    }
}

pub struct GamelessRound {
    pub partners: Vec<Partners>,
}

impl GamelessRound {
    fn new(partners: Vec<Partners>) -> GamelessRound {
        GamelessRound { partners }
    }
}

impl Display for GamelessRound {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result { 
        let games_str = self.partners.iter()
            .map(|g| format!("{}", g))
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "{}", games_str)
    }
}

pub struct Round {
    pub games: Vec<Game>,
    pub byes: Vec<Player>,
}

impl Round {
    fn new(games: Vec<Game>) -> Round {
        Round { games, byes: Vec::new() }
    }

    fn with_byes(games: Vec<Game>, byes: Vec<Player>) -> Round {
        Round { games, byes }
    }

    fn players(&self) -> Vec<Player> {
        let mut players = self.games.iter().flat_map(|x| x.players()).collect::<Vec<Player>>();
        players.extend(&self.byes);
        players
    }

    fn partners(&self) -> Vec<Partners> {
        self.games.iter().flat_map(|g| g.partners()).collect()
    }

    fn find_game_with_player(&self, p: Player) -> &Game {
        self.games.iter().find(|g| g.has_player(p)).unwrap()
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

pub struct RotationChart(pub Vec<Round>);

impl RotationChart {
    pub fn print(&self) {
        println!("{}", &self);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn players(&self) -> Vec<Player> {
        if self.0.is_empty() {
            Vec::new()
        } else {
            self.0[0].players()
        }
    }

    pub fn games_with_player(&self, p: Player) -> Vec<Game> {
        self.0.iter().map(|round| *round.find_game_with_player(p)).collect()
    }
}

impl Display for RotationChart {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result { 
        for (i, round) in self.0.iter().enumerate() {
            write!(f, "Round {}: {}\n", i, round)?;
        }
        Ok(())
    }
}
