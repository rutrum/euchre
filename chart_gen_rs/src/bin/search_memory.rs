use std::fmt;

pub type Player = usize;

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

struct TableStack<const SIZE: usize> {
    rounds: [usize; SIZE],
    tables: [[usize; 4]; SIZE],
    pointer: usize,
}

impl<const SIZE: usize> TableStack<SIZE> {
    fn new() -> Self {
        Self {
            rounds: [0; SIZE],
            tables: [[0; 4]; SIZE],
            pointer: 0,
        }
    }

    fn push(&mut self, round: usize, table: [usize; 4]) {
        self.rounds[self.pointer] = round;
        self.tables[self.pointer] = table;
        self.pointer += 1;
    }

    fn peek_round(&self) -> usize {
        self.rounds[self.pointer - 1]
    }

    fn pop(&mut self) -> [usize; 4] {
        self.pointer -= 1;
        self.tables[self.pointer]
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

    // 0 index represents round: 0 is null
    // 1-4 index are the four players
    let mut last_table_stack = TableStack::<36>::new();

    let mut loop_count: u64 = 0;

    // let max_player_per_seat: Vec<Player> = (0..SEATS)
    //     .map(|seat| match seat {
    //         0 => 1,
    //         s if s % 2 == 1 => SEATS,
    //         s if s % 4 == 2 => SEATS - 1,
    //         s if s % 4 == 0 => SEATS - (SEATS / 4 - seat / 4) - 1,
    //         _ => unreachable!(),
    //     } as Player)
    //     .collect();

    let max_player_per_seat: [Player; 12] = [1, 12, 11, 12, 9, 12, 11, 12, 10, 12, 11, 12];

    // let mut loop_count_by_seat = [[0_u64; SEATS]; ROUNDS];

    loop {
        // player at this point means "we just checked player"
        // "we plan on looking at player+1 next"
        loop_count += 1;
        // loop_count_by_seat[round][seat] += 1;

        if cfg!(debug_assertions) && loop_count % 1_000_000_000 == 0 {
            println!("{loop_count}");
        }

        if loop_count % 100_000_000_000 == 0 {
            println!("Count: {loop_count}");
            // for r in loop_count_by_seat {
            //     let s: u64 = r.iter().sum();
            //     println!("{s}: {r:?}");
            // }
        }

        // what if I could check right here, or just know
        // that a certain player wasn't applicable to this spot
        // what if as I assigned players, I also scatched off seats
        // kind of like round_players...

        //println!("round {round} on seat {seat} testing player {}", player + 1);

        if player == max_player_per_seat[seat as usize] {
            // that was last player, go to last seat
            //assert_eq!(chart.rounds[round][seat], 0);

            if seat > 2 {
                seat -= 1;

                // undo the last assignment
                player = chart.rounds[round][seat];
                chart.rounds[round][seat] = 0;
                round_players[player as usize - 1] = false;

                let table = &chart.rounds[round][seat / 4 * 4..];
                match seat % 4 {
                    1 => {
                        let partner = table[0];
                        chart.partner_counts.dec(player, partner);
                    }
                    2 => {
                        let (left, right) = (table[0], table[1]);
                        chart.opponent_counts.dec(player, left);
                        chart.opponent_counts.dec(player, right);
                    }
                    3 => {
                        let (left, right, partner) = (table[0], table[1], table[2]);
                        chart.partner_counts.dec(player, partner);
                        chart.opponent_counts.dec(player, left);
                        chart.opponent_counts.dec(player, right);
                    }
                    _ => {} // nothing to undo
                };
            } else if round > 1 {
                // try the next stack
                round -= 1;
                if last_table_stack.peek_round() == round {
                    let table = &chart.rounds[round][SEATS - 4..];
                    let (a, b, c, d) = (table[0], table[1], table[2], table[3]);

                    // update counts
                    chart.partner_counts.dec(a, b);
                    chart.partner_counts.dec(c, d);
                    chart.opponent_counts.dec(a, c);
                    chart.opponent_counts.dec(a, d);
                    chart.opponent_counts.dec(b, c);
                    chart.opponent_counts.dec(b, d);

                    let table @ [a, b, c, d] = last_table_stack.pop();

                    // this could be more clever
                    chart.partner_counts.inc(a, b);
                    chart.partner_counts.inc(c, d);
                    chart.opponent_counts.inc(a, c);
                    chart.opponent_counts.inc(a, d);
                    chart.opponent_counts.inc(b, c);
                    chart.opponent_counts.inc(b, d);

                    chart.rounds[round][SEATS - 4..].copy_from_slice(&table);

                    round += 1;

                    player = 1;
                } else {
                    // no other tables to try
                    // back up to last seat

                    // Update last table
                    let table = &chart.rounds[round][SEATS - 4..];
                    let (a, b, c, d) = (table[0], table[1], table[2], table[3]);

                    // update counts
                    chart.partner_counts.dec(a, b);
                    chart.partner_counts.dec(c, d);
                    chart.opponent_counts.dec(a, c);
                    chart.opponent_counts.dec(a, d);
                    chart.opponent_counts.dec(b, c);
                    chart.opponent_counts.dec(b, d);

                    // update SEATS-5 player
                    seat = SEATS - 5;
                    let previous_table = &chart.rounds[round][SEATS - 8..];
                    let (right, left, partner) =
                        (previous_table[0], previous_table[1], previous_table[2]);
                    player = previous_table[3];

                    chart.partner_counts.dec(player, partner);
                    chart.opponent_counts.dec(player, left);
                    chart.opponent_counts.dec(player, right);

                    // unset all the players
                    round_players = [true; SEATS];
                    // adds two seconds?
                    // round_players[a as usize - 1] = false;
                    // round_players[b as usize - 1] = false;
                    // round_players[c as usize - 1] = false;
                    // round_players[d as usize - 1] = false;
                    // round_players[player as usize - 1] = false;
                    for &p in &chart.rounds[round][SEATS - 5..] {
                        round_players[p as usize - 1] = false;
                    }
                    chart.rounds[round][seat..].copy_from_slice(&[0, 0, 0, 0, 0]);
                }

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

            continue;
            // here player can be 1 to 12
        }

        player += 1;

        if round_players[player as usize - 1] {
            continue;
            // here player is 1 to 12
        }

        let table = &chart.rounds[round][seat / 4 * 4..];
        let table_seat = seat % 4;
        let meets_criteria = match table_seat {
            1 => {
                let partner = table[0];

                chart.partner_counts.get(player, partner) < 1
            }
            2 => {
                let (left, right) = (table[0], table[1]);

                chart.opponent_counts.get(player, left) < 2
                    && chart.opponent_counts.get(player, right) < 2
            }
            3 => {
                let (left, right, partner) = (table[0], table[1], table[2]);

                chart.partner_counts.get(player, partner) < 1
                    && chart.opponent_counts.get(player, left) < 2
                    && chart.opponent_counts.get(player, right) < 2
            }
            _ => true,
        };

        if meets_criteria {
            // valid player assignment, go to next seat
            chart.rounds[round][seat] = player;

            let table = &chart.rounds[round][seat / 4 * 4..];
            match table_seat {
                1 => {
                    let partner = table[0];

                    chart.partner_counts.inc(player, partner);
                }
                2 => {
                    let (left, right) = (table[0], table[1]);

                    chart.opponent_counts.inc(player, left);
                    chart.opponent_counts.inc(player, right);
                }
                3 => {
                    let (left, right, partner) = (table[0], table[1], table[2]);

                    chart.partner_counts.inc(player, partner);
                    chart.opponent_counts.inc(player, left);
                    chart.opponent_counts.inc(player, right);
                }
                _ => {}
            };

            seat += 1;
            round_players[player as usize - 1] = true;

            if seat == SEATS - 4 {
                // special behavior
                // should I check this before assignmment? yes
                get_table_options(round, &chart, &round_players, &mut last_table_stack);

                // is there at least one table?
                if last_table_stack.peek_round() == round {
                    let table @ [a, b, c, d] = last_table_stack.pop();

                    // assign table
                    chart.rounds[round][SEATS - 4..].copy_from_slice(&table);

                    // update counts
                    chart.partner_counts.inc(a, b);
                    chart.partner_counts.inc(c, d);
                    chart.opponent_counts.inc(a, c);
                    chart.opponent_counts.inc(a, d);
                    chart.opponent_counts.inc(b, c);
                    chart.opponent_counts.inc(b, d);

                    // setup for next round
                    if round < ROUNDS - 1 {
                        seat = 2;
                        round += 1;

                        round_players = [false; SEATS];
                        round_players[0] = true;
                        round_players[round + 1] = true;
                        player = 1;
                    } else {
                        // winner?
                        println!("Final count: {loop_count}");
                        // for r in loop_count_by_seat {
                        //     let s: u64 = r.iter().sum();
                        //     println!("{s}: {r:?}");
                        // }
                        return Some(chart);
                    }
                } else {
                    // we can unwind now
                    // just go to next player

                    round_players[player as usize - 1] = false;
                    seat -= 1;

                    chart.rounds[round][seat] = 0;

                    let table = &chart.rounds[round][seat / 4 * 4..];
                    let (left, right, partner) = (table[0], table[1], table[2]);

                    chart.partner_counts.dec(player, partner);
                    chart.opponent_counts.dec(player, left);
                    chart.opponent_counts.dec(player, right);
                };
            } else if seat % 2 == 1 {
                // the right partner should be greater than the left
                player = chart.rounds[round][seat - 1];
            } else if seat % 4 == 2 {
                // the first right opponent should be greater than the first left opponent
                player = chart.rounds[round][seat - 2];
            } else if seat % 4 == 0 {
                // the first person of the next table should be greater than last
                player = chart.rounds[round][seat - 4];
            }
        }
        // What options can player be here?
        // 0 up to 11
    }
}

// I'm trying to short circuit here
// thats 3*6=18 conditions/computations to check, which means 18 lookups
// that's not even true... There's overlap!  I should check those first,
// which is the opponent constraints
// pairs: ab, ac, ad, cd, bd, bc (all of them)
// opps: ac, ad, ab, bc, bd, cd (only 6)
// so thats 12 looksup max
fn get_table_options<const SEATS: usize, const ROUNDS: usize, const SIZE: usize>(
    round: usize,
    chart: &Chart<SEATS, ROUNDS>,
    round_players: &[bool; SEATS],
    table_stack: &mut TableStack<SIZE>,
) {
    let mut player_idx = 0;
    let mut players: [Player; 4] = [0; 4];
    let first_player_last_table = chart.rounds[round][SEATS - 4 - 4];
    for (player, present) in round_players.iter().enumerate() {
        if !*present {
            if player_idx == 0 {
                if player + 1 < first_player_last_table {
                    return;
                }
            }
            players[player_idx] = player + 1;
            player_idx += 1;
        }
    }

    let (a, b, c, d) = (players[0], players[1], players[2], players[3]);

    if (chart.partner_counts.get(a, d) < 1)
        && (chart.partner_counts.get(b, c) < 1)
        && (chart.opponent_counts.get(a, b) < 2) // 4
        && (chart.opponent_counts.get(a, c) < 2) // 6
        && (chart.opponent_counts.get(d, b) < 2) // 5
        && (chart.opponent_counts.get(d, c) < 2)
    // 3
    {
        table_stack.push(round, [a, d, b, c]);
    }

    if (chart.partner_counts.get(a, c) < 1)
        && (chart.partner_counts.get(b, d) < 1)
        && (chart.opponent_counts.get(a, b) < 2) // 4
        && (chart.opponent_counts.get(a, d) < 2) // 1
        && (chart.opponent_counts.get(c, b) < 2) // 2
        && (chart.opponent_counts.get(c, d) < 2)
    // 3
    {
        table_stack.push(round, [a, c, b, d]);
    }

    if (chart.partner_counts.get(a, b) < 1)
        && (chart.partner_counts.get(c, d) < 1)
        && (chart.opponent_counts.get(a, c) < 2) // 6
        && (chart.opponent_counts.get(a, d) < 2) // 1
        && (chart.opponent_counts.get(b, c) < 2) // 2
        && (chart.opponent_counts.get(b, d) < 2)
    // 5
    {
        table_stack.push(round, [a, b, c, d]);
    }
}

fn main() {
    //let chart = Chart::<8, 7>::new();
    let chart = Chart::<12, 9>::new();
    //let chart = Chart::<12, 10>::new();
    let new_chart = dfs_loop(chart).unwrap();
    println!("{new_chart}");
    println!("Partner counts: \n{}", new_chart.partner_counts);
    println!("Opponent counts: \n{}", new_chart.opponent_counts);
}

// The speed for 8 players is the same for each
// Lets bump it up to 12

// 12 players 6 rounds
// improved: 0.041s
// memory: 0.041s

// 12 players 7 rounds
// improved: 0m41.6s
//      71124_16736 loops
// memory: 5m4.2s
//    4_08115_80846 loops
// Fixed symmetry problem
// memory: 0m27.658s
//      31396_83392 loops

// does it still scale luckily? yes
// 12 players 9 rounds
// memory: 29.207s
//      31555_88425

// Lets send it.  12 players 11 rounds.
// The first tests at round 11 happened at
// 383_00000_00000
// That means it went through 383/822 = 46% of the normal iterations.
// Unforunately it's still much slower per iteration, but I think it
// makes up for it in descreased iterations

// Despite being less than half the loop count, each loops
// still takes a long time. I should consider how this could
// be made quicker, and also clean up the code.

// Push directly to stack, don't return a vector
// 27.669s
// 31555_88425

// Combine two of the opponent checks together
// 28.187s
// 31555_88425
// Undoing this one, probably worse because the
// partner check is likely to make these fail early

// Remove assert
// 26.621s

// Remmove seat < SEATS - 1 check
// 30s...
// Undid this

// Remove other assert
// 27.98s

// Remove seat < SEATS - check again
// 27.4s

// Unnecessary remove left<0 check
// 27.7s

// Remove dead code
// 27.959

// remove loop from remove round_players
// 29.649s
// I undid this

// last_chunk instead of manual variable assignment
// same

// lots of perf changes later including
// * making my own stack
// * simplifying conditions for checking paircounts
// * taking most things off heap
// using Vec<Player> as max_player_per_seat
// 18.1s
// hardcoding it with an array
// 15.4s
// Final time right before 2nd run was 16.0s

// Unsafe code?
// 17s?
