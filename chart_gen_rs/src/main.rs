use rand::{self, seq::SliceRandom, Rng};
use std::fmt;

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

#[derive(Debug, Clone)]
struct PairCount<const SEATS: usize> {
    counts: [[usize; SEATS]; SEATS],
}

impl<const SEATS: usize> PairCount<SEATS> {
    fn new() -> Self {
        Self {
            counts: [[0; SEATS]; SEATS],
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

    fn into_iter(self) -> Self::IntoIter {
        PairCountIterator {
            x: 0,
            y: 0,
            pair_count: self.clone(),
        }
    }
}

struct PairCountIterator<const SEATS: usize> {
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
                return None;
            }
        }
        Some(self.pair_count.counts[self.x][self.y])
    }
}

const fn get_partner_seat(seat: usize) -> usize {
    seat + 1 - 2 * (seat % 2)
}

/// Returns the seats of the player, their partner, and the two opponents
/// MEMOIZE THIS
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

const fn get_table_players<const SEATS: usize>(
    seat: usize,
    round: &[Player; SEATS],
) -> (Player, Player, (Player, Player)) {
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
                let pair = Pair::new(round[s], round[s + 1]);
                counts.inc(&pair);
            }
        }
        counts
    }

    fn count_opponents(rounds: [[Player; S]; R]) -> PairCount<S> {
        let mut counts = PairCount::new();
        for round in rounds {
            for s in (0..S).step_by(4) {
                let players = &round[s..s + 4];
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
        self.partner_counts
            .into_iter()
            .filter(|val| *val > 1)
            .count()
    }

    fn bad_opponents_counts_score(&self) -> usize {
        // improve/cache this for more performance
        self.opponent_counts
            .into_iter()
            .filter(|val| *val > 2)
            .count()
    }

    fn swap_seats(&mut self, round_num: usize, a: usize, b: usize) {
        let round = &mut self.rounds[round_num];

        let (player_a, partner_a, opponents_a) = get_table_players(a, &round);
        let (player_b, partner_b, opponents_b) = get_table_players(b, &round);

        self.partner_counts.dec(&Pair::new(player_a, partner_a));
        self.partner_counts.dec(&Pair::new(player_b, partner_b));

        self.partner_counts.inc(&Pair::new(player_a, partner_b));
        self.partner_counts.inc(&Pair::new(player_b, partner_a));

        self.opponent_counts
            .dec(&Pair::new(player_a, opponents_a.0));
        self.opponent_counts
            .dec(&Pair::new(player_a, opponents_a.1));

        self.opponent_counts
            .dec(&Pair::new(player_b, opponents_b.0));
        self.opponent_counts
            .dec(&Pair::new(player_b, opponents_b.1));

        self.opponent_counts
            .inc(&Pair::new(player_a, opponents_b.0));
        self.opponent_counts
            .inc(&Pair::new(player_a, opponents_b.1));

        self.opponent_counts
            .inc(&Pair::new(player_b, opponents_a.0));
        self.opponent_counts
            .inc(&Pair::new(player_b, opponents_a.1));

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

fn refine_with_random_swaps<const S: usize, const R: usize>(
    mut chart: Chart<S, R>,
    max_loops: usize,
) -> Chart<S, R> {
    let mut rng = rand::rng();

    for iteration in 0..max_loops {
        if chart.bad_opponents_counts_score() + chart.bad_partner_counts_score() == 0 {
            println!("Finished after {iteration} iterations.");
            break;
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
                            if rng.random_bool(0.2)
                                && chart.swap_improvements(r, seat, another_seat) > 3
                            {
                                chart.swap_seats(r, seat, another_seat);
                                break;
                            }
                        }
                    }
                }

                for opponent in &[opponents.0, opponents.1] {
                    if chart.opponent_counts.get(&Pair::new(player, *opponent)) > 2 {
                        for another_seat in 0..S {
                            let (another_player, ..) = get_table_players(another_seat, &round);
                            if !table_players.contains(&another_player) {
                                // what if the min swap improvements decreased over time?
                                // so once a good solution was found, higher risks were taken to
                                // find better ones
                                if rng.random_bool(0.2)
                                    && chart.swap_improvements(r, seat, another_seat) > 4
                                {
                                    chart.swap_seats(r, seat, another_seat);
                                    break;
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

fn refine_genetic_algorithm<const S: usize, const R: usize>(
    population: usize,
    max_generations: usize,
    max_loops: usize,
) -> Chart<S, R> {
    // generate population charts
    // then run the refine algorithm max_loops times on each
    // then filter and mutate
    // do this for max_generations
    let mut charts: Vec<Chart<S, R>> = (0..population).map(|_| Chart::new()).collect();
    let mut rng = rand::rng();

    let keep_count = (population as f32 * 0.2).floor() as usize;
    let new_count = (population as f32 * 0.2).floor() as usize;
    let mutated_count = population - keep_count - new_count;

    for gen in 0..max_generations {
        let mut refined_charts: Vec<Chart<S, R>> = charts
            .into_iter()
            .enumerate()
            .map(|(i, c)| {
                if i > 10 {
                    refine_with_random_swaps(c, max_loops)
                } else {
                    c
                }
            })
            .collect();

        (&mut refined_charts).sort_by(|a, b| {
            (a.bad_opponents_counts_score() + a.bad_partner_counts_score())
                .cmp(&(b.bad_opponents_counts_score() + a.bad_partner_counts_score()))
        });

        refined_charts.truncate(keep_count + mutated_count);

        let mut to_mutate = refined_charts.split_off(keep_count);
        to_mutate.iter_mut().for_each(|c| {
            for _ in 0..10 {
                random_swap(c, &mut rng)
            }
        });

        let mut new_charts: Vec<Chart<S, R>> = (0..new_count).map(|_| Chart::new()).collect();

        refined_charts.append(&mut new_charts);

        charts = refined_charts;

        let best_score =
            charts[0].bad_opponents_counts_score() + charts[0].bad_partner_counts_score();
        if best_score == 0 {
            break;
        }

        println!("Generation {gen}: {best_score}");
    }

    // TODO: this shifts the elements of chart, probably doesn't matter
    charts.remove(0)
}

fn random_swap<const S: usize, const R: usize>(
    chart: &mut Chart<S, R>,
    rng: &mut rand::rngs::ThreadRng,
) {
    let round = rng.random_range(0..R);
    let seat_a = rng.random_range(0..S);
    let mut seat_b = seat_a;
    while seat_a == seat_b {
        seat_b = rng.random_range(0..S);
    }
    chart.rounds[round].swap(seat_a, seat_b);
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

    let chart: Chart<20, 16> = refine_genetic_algorithm(1000, 500, 20);

    println!("{}", chart);
    println!("Bad partners: {:?}", chart.bad_partner_counts_score());
    println!("Bad opponents: {:?}", chart.bad_opponents_counts_score());
}

// ideas
// try compressing by assuming the first row is always 1..S, don't store that round
// multiprocessing?
