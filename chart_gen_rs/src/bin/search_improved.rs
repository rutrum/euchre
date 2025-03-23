use chart_gen_rs::{get_table_players, get_table_players_unordered, Player};
use std::fmt;

#[derive(Debug, Clone)]
struct PairCount<const SEATS: usize> {
    pub counts: [[usize; SEATS]; SEATS],
}

impl<const SEATS: usize> PairCount<SEATS> {
    fn new() -> Self {
        Self {
            counts: [[0; SEATS]; SEATS],
        }
    }

    fn get(&self, left: Player, right: Player) -> usize {
        self.counts[left as usize - 1][right as usize - 1]
    }

    fn inc(&mut self, left: Player, right: Player) {
        self.counts[left as usize - 1][right as usize - 1] += 1;
        self.counts[right as usize - 1][left as usize - 1] += 1;
    }

    fn dec(&mut self, left: Player, right: Player) {
        self.counts[left as usize - 1][right as usize - 1] -= 1;
        self.counts[right as usize - 1][left as usize - 1] -= 1;
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

    let mut loop_count: u64 = 0;

    let max_player_per_seat: Vec<Player> = (0..SEATS)
        .map(|seat| match seat {
            0 => 1,
            s if s % 2 == 1 => SEATS,
            s if s % 4 == 2 => SEATS - 1,
            s if s % 4 == 0 => SEATS - (SEATS / 4 - seat / 4) - 1,
            _ => unreachable!(),
        } as Player)
        .collect();

    println!("max");
    max_player_per_seat.iter().for_each(|x| println!("{x}"));

    let mut loop_count_by_seat = [[0_u64; SEATS]; ROUNDS];

    loop {
        // player at this point means "we just checked player"
        // "we plan on looking at player+1 next"
        loop_count += 1;
        loop_count_by_seat[round][seat] += 1;

        if loop_count % 1_000_000_000 == 0 {
            println!("{loop_count}");
        }

        if loop_count > 10_000_000_000 {
            break None;
        }

        // what if I could check right here, or just know
        // that a certain player wasn't applicable to this spot
        // what if as I assigned players, I also scatched off seats
        // kind of like round_players...

        if player == max_player_per_seat[seat as usize] {
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
                chart.partner_counts.dec(player, partner);
            }
            if left > 0 {
                chart.opponent_counts.dec(player, left);
                chart.opponent_counts.dec(player, right);
            }

            continue;
            // here player can be 1 to 12
        }

        player += 1;

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

        if round_players[player as usize - 1] {
            continue;
            // here player is 1 to 12
        }

        // can I do this calculation in place or one at a time?
        // it might be faster and I only need to calculate left/right when
        // partner succeeds
        // I'm betting the compiler is on it, however...
        let (_, partner, (left, right)) = get_table_players_unordered(seat, &chart.rounds[round]);

        // still, left and right aren't necessary if partner check fails
        // so maybe this conditional should be broken up more

        if (partner == 0 || chart.partner_counts.get(player, partner) < 1)
            && (left == 0
                || (chart.opponent_counts.get(player, left) < 2)
                    && chart.opponent_counts.get(player, right) < 2)
        {
            // valid player assignment, go to next seat
            chart.rounds[round][seat] = player;

            // is there a chance I could do this increment and it not matter?
            // that would require partner_count have (seats+1)^2
            if partner > 0 {
                chart.partner_counts.inc(player, partner);
            }
            if left > 0 {
                chart.opponent_counts.inc(player, left);
                chart.opponent_counts.inc(player, right);
            }

            // now increment to next seat
            if seat < SEATS - 1 {
                seat += 1;
                round_players[player as usize - 1] = true;

                if seat % 2 == 1 {
                    // the right partner should be greater than the left
                    player = chart.rounds[round][seat - 1];
                } else if seat % 4 == 2 {
                    // the first opponent at a table should be greater than the player
                    player = chart.rounds[round][seat - 2];
                } else if seat % 4 == 0 {
                    // the first person of the next table should be greater than last
                    player = chart.rounds[round][seat - 4];
                }
            } else if round < ROUNDS - 1 {
                seat = 0;
                round += 1;
                round_players = [false; SEATS];
                player = 0;
            } else {
                // found it!
                println!("Final count: {loop_count}");
                for r in loop_count_by_seat {
                    let s: u64 = r.iter().sum();
                    println!("{s}: {r:?}");
                }
                break Some(chart);
            }
        }
        // What options can player be here?
        // 0 up to 11
    }
}

fn main() {
    //let chart = Chart::<8, 7>::new();
    //let chart = Chart::<12, 6>::new();
    let chart = Chart::<12, 7>::new();
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

// Other strategies
// use this to find a perfect table with the partner constraints
// use this as a seed for the table in swap.rs
// I may also try the state space of all possible X sets of swaps
// or at least search the space and pick the most optimal, and recurse

// Maybe multiprocessing
// I wonder how I can split the statespace without doing way too much work
// due to symmetry

// Actually, am I doing redundant work now due to symmetry?

// one way to avoid symmetry is to generate only a "cononically" ordered
// chart?  I might order every table (a, b, c, d) such that a < c,
// so I shouldn't check the reverse, because it will have the same outcome
// TODO: this

// why is the current found solution ordered already?
// I would expect, since we start with player 1 every time
// that this would naturally come out ordered
// maybe there's a bug, or something wrong with my logic

// Baseline: first timing of dfs_loop
//   8 players
//   with opponent constraint
// real    30m47.170s
// user    30m41.211s
// sys     0m0.108s

// Bench after removing pair swaps/comparisons
// now just updating both parts of the matricies
// real    26m32.926s
// user    26m27.027s
// sys     0m0.041s

// Bench after not enforcing that opponent left < right
// real    25m51.644s
// user    25m46.401s
// sys     0m0.104s

// Bench after asserting that partner % 4 == 2 is greater
// than player % 4 == 0
// real    0m18.093s
// user    0m17.998s
// sys     0m0.013s

// Bench after asserting that player % 2 == 1 is greater
// than player % 2 == 0
// real    0m0.398s
// user    0m0.300s
// sys     0m0.098s

// After checking player % 2 before player % 4
// real    0m0.120s
// user    0m0.105s
// sys     0m0.015s

// Time for a new benchmark.  Lets try 12 players, stop at 6 assigned rounds
// real    1m1.265s
// user    1m0.939s
// sys     0m0.111s

// Bench after asserting that first table player is greater than last table player
// stopping at 6 assigned rounds might not be fair, since improvements
// that prune the tree might prevent it

// A better one might be always undoing at round X as though every
// thing below it had been explored already
// then we see how quickly we can exhaust ALL possible 6 row combinations
// well...for 12 its far too big

// Lets count loops now
// 8 players: check until player == SEATS
// 31_77326

// 8 players: check until max_player_per_seat
// 13_91821

// 8 players: now start with min_player_per_seat
// 13_91821
// lol I already have better lower bounds set by sorting arguments

// 8 players: look ahead to next pair and make sure its okay
// 13_21149
// but I worry this is slower, since I'm doing O(SEATS) check
// that said, the common case is short circuited, so its not always SEATS
// checks

// One bench is just to actually make the chart have less rounds.
// 12 players, 6 rounds
// 33_74149

// Thought I saw a bug.  The final result ended with the second player
// being a 10, not a 7.  That's because the first 5 rounds had so much
// of the same opponents with 1 that it made partnering with 1 and 7
// inviable for everything.  This is a "lookahead" problem that
// I could fix.  When I assign 7, I should be able to see if there
// exist 2 opponents that would work for 7 and 1

// Problem: the case is more narrow than expected.  This scenario is "unlikely"
// in my testing.  It just happens the case I saw, when valid opponents
// were available they still failed since those opponents couldn't be
// partners.  I dont know if I can look ahead without incurring
// too much cost to time doing loops constantly.

// doing this first check made the first 10bil loops go from 1m to 1m14s

// look ahead checks
// before assigning seat % 4 == 1, make sure two opponents are viable
// for the next two slots
// if seat % 4 == 1 {
//     let opponents_viable = (1..=SEATS as Player).any(|i| {
//         chart.opponent_counts.get(player, i) < 2
//             && chart.opponent_counts.get(partner, i) < 2
//             && i != player
//             && i != partner
//     });

//     if !opponents_viable {
//         continue;
//     }
// }
//else if seat % 4 == 2 {
//    // check that partners are available
//    let partner_available = (1..=SEATS as Player)
//        .any(|i| chart.partner_counts.get(player, i) < 1 && i != player);

//    if !partner_available {
//        //continue;
//    }
//}

// After removing duplicate checks for left AND right...if left is nonzero, so is right
// real    0m55.372s
