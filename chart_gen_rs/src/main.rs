use std::fmt;
use rand::{self, seq::SliceRandom, Rng};

type Player = u8;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Pair(Player, Player);

impl Pair {
    fn new(left: Player, right: Player) -> Self {
        if left < right {
            Pair(left, right)
        } else {
            Pair(right, left)
        }
    }
}

// TODO: next time, integrate this paircount instead of hashmaps
// this should help eliminate the errors/bugs/problems with hashmaps
// retrieving keys that don't exist, hopefully
#[derive(Debug, Clone)]
struct PairCount<const SEATS: usize> {
    counts: [[usize; SEATS]; SEATS],
}

impl<const SEATS: usize> PairCount<SEATS> {
    fn new() -> Self {
        Self {
            counts: [[0; SEATS]; SEATS]
        }
    }

    fn get(&self, pair: &Pair) -> usize {
        self.counts[pair.0 as usize][pair.1 as usize]
    }

    fn inc(&mut self, pair: &Pair) {
        self.counts[pair.0 as usize][pair.1 as usize] += 1
    }

    fn dec(&mut self, pair: &Pair) {
        self.counts[pair.0 as usize][pair.1 as usize] -= 1
    }
}

impl<'a, const SEATS: usize> IntoIterator for &PairCount<SEATS> {
    type Item = usize;
    type IntoIter = PairCountIterator<SEATS>;

    fn into_iter(self) -> Self::IntoIter{
        PairCountIterator{x: 0, y: 0, pair_count: self.clone()}
    }
}

struct PairCountIterator<const SEATS: usize>{
    pair_count: PairCount<SEATS>,
    x: usize, 
    y: usize,
}

impl<const SEATS: usize> Iterator for PairCountIterator<SEATS> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.y += 1;
        if self.y == SEATS {
            self.x += 1;
            self.y = self.x;
            if self.x == SEATS {
                return None
            }
        }
        Some(self.pair_count.counts[self.x][self.y])
    }
}

const fn get_partner_seat(seat: usize) -> usize {
    if seat % 2 == 0 {
        seat + 1
    } else {
        seat - 1
    }
}

/// Returns the seats of the player, their partner, and the two opponents
const fn get_table_seats(seat: usize) -> (usize, usize, (usize, usize)) {
    let player = seat;
    let partner = get_partner_seat(seat);
    let table = seat / 4;

    let opponents = if seat % 4 < 2 {
        (table * 4 + 2, table * 4 + 3)
    } else {
        (table * 4 + 0, table * 4 + 1)
    };

    (player, partner, opponents)
}

const fn get_table_players<const SEATS: usize>(seat: usize, round: &[Player; SEATS]) 
        -> (Player, Player, (Player, Player)) {
    let (player_seat, partner_seat, opponent_seats) = get_table_seats(seat);
    let player = round[player_seat];
    let partner = round[partner_seat];
    let opponents = (round[opponent_seats.0], round[opponent_seats.1]);
    (player, partner, opponents)
}

#[derive(Debug)]
struct Chart<const SEATS: usize, const ROUNDS: usize> {
    rounds: [[Player; SEATS]; ROUNDS],
    partner_counts: PairCount<SEATS>,
    opponent_counts: PairCount<SEATS>,
}

impl<const S: usize, const R: usize> Chart<S, R> {

    /// Create a new valid chart.
    fn new() -> Self {
        let mut rng = rand::rng();
        let mut rounds: [[Player; S]; R] = [[0; S]; R];

        for round in rounds.iter_mut() {
            for s in 0..S {
                round[s] = s as Player;
            }
            round.shuffle(&mut rng)
        }
        Self { 
            rounds, 
            partner_counts: Self::count_partners(rounds),
            opponent_counts: Self::count_opponents(rounds),
        }
    }

    fn count_partners(rounds: [[Player; S]; R]) -> PairCount<S> {
        let mut counts = PairCount::new();
        for round in rounds {
            for s in (0..S).step_by(2) {
                let pair = Pair::new(round[s], round[s+1]);
                counts.inc(&pair);
            }
        }
        counts
    }

    fn count_opponents(rounds: [[Player; S]; R]) -> PairCount<S> {
        let mut counts = PairCount::new();
        for round in rounds {
            for s in (0..S).step_by(4) {
                let players = &round[s..s+4];
                let pairs = &[
                    Pair::new(players[0], players[2]),
                    Pair::new(players[0], players[3]),
                    Pair::new(players[1], players[2]),
                    Pair::new(players[1], players[3]),
                ];
                for pair in pairs {
                    counts.inc(&pair);
                }
            }
        }
        counts
    }

    fn bad_partner_counts_score(&self) -> usize {
        self.partner_counts.into_iter().filter(|val| *val > 1).count()
    }

    fn bad_opponents_counts_score(&self) -> usize {
        // improve/cache this for more performance
        self.opponent_counts.into_iter().filter(|val| *val > 2).count()
    }

    fn swap_seats(&mut self, round_num: usize, a: usize, b: usize) {
        let round = &mut self.rounds[round_num];

        // another idea: instead of hash tables, use an upper triangular matrix

        let (player_a, partner_a, opponents_a) = get_table_players(a, &round);
        let (player_b, partner_b, opponents_b) = get_table_players(b, &round);

        self.partner_counts.dec(&Pair::new(player_a, partner_a));
        self.partner_counts.dec(&Pair::new(player_b, partner_b));

        self.partner_counts.inc(&Pair::new(player_a, partner_b));
        self.partner_counts.inc(&Pair::new(player_b, partner_a));

        self.opponent_counts.dec(&Pair::new(player_a, opponents_a.0));
        self.opponent_counts.dec(&Pair::new(player_a, opponents_a.1));

        self.opponent_counts.dec(&Pair::new(player_b, opponents_b.0));
        self.opponent_counts.dec(&Pair::new(player_b, opponents_b.1));

        self.opponent_counts.inc(&Pair::new(player_a, opponents_b.0));
        self.opponent_counts.inc(&Pair::new(player_a, opponents_b.1));

        self.opponent_counts.inc(&Pair::new(player_b, opponents_a.0));
        self.opponent_counts.inc(&Pair::new(player_b, opponents_a.1));

        round.swap(a, b);
    }

    fn swap_improvements(&self, round_num: usize, a: usize, b: usize) -> usize {
        let round = self.rounds[round_num];
        let (a, a_partner, a_opponents) = get_table_players(a, &round);
        let (b, b_partner, b_opponents) = get_table_players(b, &round);

        (self.partner_counts.get(&Pair::new(a, b_partner)) < 1) as usize
        + (self.partner_counts.get(&Pair::new(b, a_partner)) < 1) as usize
        + (self.opponent_counts.get(&Pair::new(a, b_opponents.0)) < 2) as usize
        + (self.opponent_counts.get(&Pair::new(a, b_opponents.1)) < 2) as usize
        + (self.opponent_counts.get(&Pair::new(b, a_opponents.0)) < 2) as usize
        + (self.opponent_counts.get(&Pair::new(b, a_opponents.1)) < 2) as usize
    }
}

impl<const S: usize, const R: usize> fmt::Display for Chart<S, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for r in 0..R {
            for s in 0..S {
                write!(f, " {:>3?}", self.rounds[r][s])?;
            }
            if r < R - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

fn refine_with_random_swaps<const S: usize, const R: usize>(mut chart: Chart<S, R>, max_loops: usize) -> Chart<S, R> {
    let mut rng = rand::rng();

    for iteration in 0..max_loops {
        if chart.bad_opponents_counts_score() + chart.bad_partner_counts_score() == 0 {
            println!("{iteration}");
            break
        }
        for r in 0..R {
            let round = chart.rounds[r];
            for seat in 0..S {
                let (player, partner, opponents) = get_table_players(seat, &round);
                let table_players = [player, partner, opponents.0, opponents.1];

                if chart.partner_counts.get(&Pair::new(player, partner)) > 1 {
                    // too many times, switch it up
                    for another_seat in 0..S {
                        let (another_player, ..) = get_table_players(another_seat, &round);
                        if !table_players.contains(&another_player) {
                            // swapping would change something
                            // how much does it change?  Is it an improvement?
                            if chart.swap_improvements(r, seat, another_seat) > 4 && rng.random_bool(0.8) {
                                chart.swap_seats(r, seat, another_seat);
                                break
                            }
                        }
                    }
                }

                for opponent in &[opponents.0, opponents.1] {
                    if chart.opponent_counts.get(&Pair::new(player, *opponent)) > 2 {
                        for another_seat in 0..S {
                            let (another_player, ..) = get_table_players(another_seat, &round);
                            if !table_players.contains(&another_player) {
                                // swapping would change something
                                // how much does it change?  Is it an improvement?
                                if chart.swap_improvements(r, seat, another_seat) > 4 && rng.random_bool(0.8) {
                                    chart.swap_seats(r, seat, another_seat);
                                    break
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    chart
}

fn main() {
    let chart = Chart::<20, 16>::new();

    println!("{}", chart);
    println!("Bad partners: {:?}", chart.bad_partner_counts_score());
    println!("Bad opponents: {:?}", chart.bad_opponents_counts_score());

    let chart = refine_with_random_swaps(chart, 10000);

    println!("{}", chart);
    println!("Bad partners: {:?}", chart.bad_partner_counts_score());
    println!("Bad opponents: {:?}", chart.bad_opponents_counts_score());

}
