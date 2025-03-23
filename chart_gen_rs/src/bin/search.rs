use chart_gen_rs::{get_table_players, Player};
use std::fmt;

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
        self.counts[pair.0 as usize - 1][pair.1 as usize - 1]
    }

    fn inc(&mut self, pair: &Pair) {
        self.counts[pair.0 as usize - 1][pair.1 as usize - 1] += 1
    }

    fn dec(&mut self, pair: &Pair) {
        self.counts[pair.0 as usize - 1][pair.1 as usize - 1] -= 1
    }
}

impl<const S: usize> fmt::Display for PairCount<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for r in 0..S {
            for s in 0..S {
                write!(f, " {:>3?}", self.counts[r][s])?;
            }
            if r < S - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

struct Chart<const SEATS: usize, const ROUNDS: usize> {
    // Consider Option<Player=NonZeroU8>
    rounds: [[Player; SEATS]; ROUNDS],
    partner_counts: PairCount<SEATS>,
    opponent_counts: PairCount<SEATS>,
}

impl<const S: usize, const R: usize> Chart<S, R> {
    fn new() -> Self {
        Self {
            rounds: [[0; S]; R],
            partner_counts: PairCount::new(),
            opponent_counts: PairCount::new(),
        }
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

fn dfs_loop<const SEATS: usize, const ROUNDS: usize>(
    mut chart: Chart<SEATS, ROUNDS>,
) -> Option<Chart<SEATS, ROUNDS>> {
    let mut round = 0;
    let mut seat = 0;
    let mut player: Player = 0;
    let mut round_players = [false; SEATS];

    // update this
    let mut possible_players = [true; SEATS];

    let mut loop_count: u64 = 0;

    loop {
        loop_count += 1;
        if loop_count % 1_000_000_000 == 0 {
            println!("{loop_count}");
        }
        //println!("{chart}");

        // what if I could check right here, or just know
        // that a certain player wasn't applicable to this spot
        // what if as I assigned players, I also scatched off seats
        // kind of like round_players...

        // try next player
        if (player as usize) < SEATS {
            player += 1;
        } else {
            // that was last player, go to last seat
            assert_eq!(chart.rounds[round][seat], 0);

            if seat > 0 {
                seat -= 1;
            } else if round > 0 {
                round -= 1;
                seat = SEATS - 1;
                round_players = [true; SEATS];
            } else {
                // tried everything, failed
                return None;
            }

            // undo the last assignment
            player = chart.rounds[round][seat];
            chart.rounds[round][seat] = 0;
            round_players[player as usize - 1] = false;

            let (_, partner, (left, right)) = get_table_players(seat, &chart.rounds[round]);

            if partner > 0 {
                let pp = Pair::new(player, partner);
                chart.partner_counts.dec(&pp);
            }
            if left > 0 {
                let pl = Pair::new(player, left);
                chart.opponent_counts.dec(&pl);
            }
            if right > 0 {
                let pr = Pair::new(player, right);
                chart.opponent_counts.dec(&pr);
            }

            continue;
        }

        //println!("{loop_count}: {round} {seat} : {player}");

        // if left has to match player later (opponent count(player, left) = 1)
        // but right can't be an opponent of player nor a partner of left
        // scan the opponent_table for player's opponents < 2
        // and see if present in right's partners < 1

        // prefilling the left 2 players might help,
        // but I suspect most the "trial and error" happens at
        // the last couple tables, where players options are low and
        // constraints are highest
        // I could record the number of loops for each seat/round
        // that would help.

        // another

        if round_players[player as usize - 1] {
            continue;
        }

        let (_, partner, (left, right)) = get_table_players(seat, &chart.rounds[round]);

        let pp = Pair::new(player, partner);
        let pl = Pair::new(player, left);
        let pr = Pair::new(player, right);

        if (partner == 0 || chart.partner_counts.get(&pp) < 1)
            && ((left == 0 || chart.opponent_counts.get(&pl) < 2)
                && (right == 0 || chart.opponent_counts.get(&pr) < 2))
        {
            // valid player assignment, go to next seat
            chart.rounds[round][seat] = player;

            if partner > 0 {
                chart.partner_counts.inc(&pp);
            }
            if left > 0 {
                chart.opponent_counts.inc(&pl);
            }
            if right > 0 {
                chart.opponent_counts.inc(&pr);
            }

            // now increment to next seat
            if seat < SEATS - 1 {
                seat += 1;
                round_players[player as usize - 1] = true;
            } else if round < ROUNDS - 1 {
                seat = 0;
                round += 1;
                round_players = [false; SEATS];
            } else {
                // found it!
                break Some(chart);
            }

            player = 0;
        } else {
            // not a good assignment
            player += 1;
        }
    }
}

fn main() {
    let chart = Chart::<8, 7>::new();
    let new_chart = dfs_loop(chart).unwrap();
    println!("{new_chart}");
    println!("Partner counts: \n{}", new_chart.partner_counts);
    println!("Opponent counts: \n{}", new_chart.opponent_counts);
    // let chart = Chart::<24, 23>::new();
    // let new_chart = dfs_loop(chart).unwrap();
    // println!("{new_chart}");
    // println!("{:?}", new_chart.partner_counts);
    // println!("{:?}", new_chart.opponent_counts);
}

// Baseline: first timing of dfs_loop
//   8 players
//   with opponent constraint
// real    30m47.170s
// user    30m41.211s
// sys     0m0.108s
