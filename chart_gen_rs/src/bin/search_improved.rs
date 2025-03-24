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
        let mut chart = Self {
            rounds: [[0; S]; R],
            partner_counts: PairCount::new(),
            opponent_counts: PairCount::new(),
        };
        // first row
        for p in 1..=S as Player {
            let seat = p as usize - 1;
            chart.rounds[0][seat] = p;
            if seat % 2 == 1 {
                chart.partner_counts.inc(p - 1, p);
            }
            if seat % 4 == 2 {
                chart.opponent_counts.inc(p - 2, p);
                chart.opponent_counts.inc(p - 1, p);
            } else if seat % 4 == 3 {
                chart.opponent_counts.inc(p - 3, p);
                chart.opponent_counts.inc(p - 2, p);
            }
        }
        // first two columns
        for r in 1..R {
            chart.rounds[r][0] = 1;
            chart.rounds[r][1] = r as Player + 2;
            chart.partner_counts.inc(1, r as Player + 2);
        }
        chart
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
    let mut round = 1;
    let mut seat = 2;
    let mut player: Player = 3;
    let mut round_players = [false; SEATS];
    round_players[0] = true;
    round_players[round + 1] = true;

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

    print!("max player per seat: ");
    max_player_per_seat.iter().for_each(|x| print!("{x} "));
    println!();

    let mut loop_count_by_seat = [[0_u64; SEATS]; ROUNDS];

    loop {
        // player at this point means "we just checked player"
        // "we plan on looking at player+1 next"
        loop_count += 1;
        loop_count_by_seat[round][seat] += 1;

        if loop_count % 1_000_000_000 == 0 {
            println!("{loop_count}");
        }

        if loop_count % 10_000_000_000 == 0 {
            println!("Count: {loop_count}");
            for r in loop_count_by_seat {
                let s: u64 = r.iter().sum();
                println!("{s}: {r:?}");
            }
        }

        // what if I could check right here, or just know
        // that a certain player wasn't applicable to this spot
        // what if as I assigned players, I also scatched off seats
        // kind of like round_players...

        //println!("round {round} on seat {seat} testing player {player}");

        if player == max_player_per_seat[seat as usize] {
            // that was last player, go to last seat
            assert_eq!(chart.rounds[round][seat], 0);

            if seat > 2 {
                seat -= 1;
            } else if round > 1 {
                round -= 1;
                seat = SEATS - 1;
                round_players = [true; SEATS];
                // something blundered here.  Something about the state of the
                // chart at this point tells us something about why the round
                // we just tried failed
                // in this case, it's exactly the next table that failed.  Why?

                //println!("Final count: {loop_count}");
                //for r in loop_count_by_seat {
                //    let s: u64 = r.iter().sum();
                //    println!("{s}: {r:?}");
                //}
                //println!("{chart}");
                //println!("Partner counts: \n{}", chart.partner_counts);
                //println!("Opponent counts: \n{}", chart.opponent_counts);
                //println!();
            } else {
                // tried everything, failed
                return Some(chart);
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

        if round_players[player as usize - 1] {
            continue;
            // here player is 1 to 12
        }

        let (_, partner, (left, right)) = get_table_players_unordered(seat, &chart.rounds[round]);

        // another check could be before the last table
        // can I look at the remaining players and try to find an inconsistency?
        // could I look at all permutations of the final four players and quick early?
        // 4!=4*3*2=24/8 symmetries = 3 possibilities
        // I could check 3 possibilities probably without doing this loop and backup stuff
        // of the four
        // the least is in the left
        // any of the three are its partner
        // the final two are in order
        // if I know a < b < c < d then check
        // a, b, c, d
        // a, c, b, d
        // a, d, b, c
        // how can I check if these are valid?
        // just check directly
        // there might be recursion here, or a subproblem
        // without loss of generality:
        // is part(ab) < 1?
        // is part(cd) < 1?
        // is opp(ac) < 2?
        // is opp(ad) < 2?
        // is opp(bc) < 2?
        // is opp(bd) < 2?
        // I'm trying to short circuit here
        // thats 3*6=18 conditions/computations to check, which means 18 lookups
        // that's not even true... There's overlap!  I should check those first,
        // which is the opponent constraints
        // pairs: ab, ac, ad, cd, bd, bc (all of them)
        // opps: ac, ad, ab, bc, bd, cd (only 6)
        // so thats 12 looksup max

        // the "cool" part about this is that if I don't invalidate a solution
        // I can return a fixed list of solutions
        // my concern is how I recurse/undo this
        // I'll first just try "if these fail quit early"
        // they later maybe try "well 2 solutions fit, lets loop over these exact two"

        if (partner == 0 || chart.partner_counts.get(player, partner) < 1)
            && (left == 0
                || (chart.opponent_counts.get(player, left) < 2
                    && chart.opponent_counts.get(player, right) < 2))
        {
            // valid player assignment, go to next seat
            chart.rounds[round][seat] = player;

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
                    // the first right opponent should be greater than the first left opponent
                    player = chart.rounds[round][seat - 2];
                } else if seat % 4 == 0 {
                    // the first person of the next table should be greater than last
                    player = chart.rounds[round][seat - 4];
                }
            } else if round < ROUNDS - 1 {
                seat = 2;
                round += 1;

                round_players = [false; SEATS];
                round_players[0] = true;
                round_players[round + 1] = true;
                player = 1;
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
    //println!("{chart}");
    //println!("Partner counts: \n{}", chart.partner_counts);
    //println!("Opponent counts: \n{}", chart.opponent_counts);

    //let chart = Chart::<12, 6>::new();
    let chart = Chart::<12, 10>::new();
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

// TODO: instead of loops above, why not treat these as bitsets?
// then do & and | operators?  Not sure this applies, but it could
// for any checks that involve for loops over the partner counts

// If I want to do the above, I need to be able to look far in advance for problems...at least
// the next table, or perhaps the next round.  Can I fabricate a scenario where
// every possible next round is invalid?  Clearly this happens, when you see how often
// the first value is tried over and over again in a round.
// This one I'm looking at has 12_93339 and another 109_35093 in the first cell.

// After removing duplicate checks for left AND right...if left is nonzero, so is right
// real    0m55.372s

// Another idea...redo this whole thing
// but generate all the valid pairs of players, then assign that way.

// I ran some computation overnight.  7 rounds, 12 people
// real    594m45.789s
// I think if I reran this after tinkering it, it might be 8/9 hours

// After pre-setting the first row and first two columns
// 8 players:
// 33696

// 12 players, 6 rounds:
// 8470

// 12 players, 7 rounds: [NEW BENCHMARK]
// 71124_16763
// real    0m38.368s
// That feels very large, but I dont think I've
// ever gotten that value before.  So cool, now we have 7 rows achievable
// of the 12 in 38s.
// Still need to get that value down to milliseconds, unfortunately
// But I should have hope, since filling in the first two spots
// actually substatially decreases the scaling of the problem, the same
// way enforcing the first spot to be 1 did.

// getting lucky

// 12 players, 8 rounds:
// 71417_43762
// 12 players, 9 rounds:
// 71526_07753
// 12 players, 10 rounds:
