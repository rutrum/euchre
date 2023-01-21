use euchre_tournament::container::*;
use euchre_tournament::error::*;
use euchre_tournament::*;
use itertools::Itertools;
use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();

    let gen_size = 20;
    let num_players = 20;

    let players = Player::new_list(num_players);

    let mut generation = get_start(gen_size, num_players, &mut rng);

    let iterations = 100;

    let min_total_randoms = 30;
    let max_total_randoms = 50;

    for i in 0..iterations {
        generation = generation
            .clone()
            .into_iter()
            .chain({
                generation.clone().into_iter().map(|(mut schedule, _)| {
                    let total_randoms = rng.gen_range(min_total_randoms..max_total_randoms);
                    let (a, b): (bool, bool) = rng.gen();
                    if true {
                        schedule = move_same_partners(schedule, &mut rng);
                    } else {
                        for _ in 0..total_randoms {
                            if b {
                                schedule = schedule.swap_random_partners_random_round(&mut rng);
                            } else {
                                schedule = schedule.swap_random_players_random_round(&mut rng);
                            }
                        }
                    }
                    let new_error = total(&schedule);
                    (schedule, new_error)
                })
            })
            .collect::<Vec<(Schedule, i32)>>();
        generation.sort_by(|a, b| a.1.cmp(&b.1));
        generation = generation.into_iter().dedup().take(gen_size).collect();
        let top = &generation[0];

        if i % 10 == 0 {
            println!("Generation {}", i);
            println!("Generation size {}", generation.len());
            print_facts(&top.0);
        }

        if total(&top.0) == 0 {
            break;
        }
    }

    let best = generation[0].0.clone();
    print_facts(&best);
}

fn print_facts(schedule: &Schedule) {
    println!("Best:");
    let best = schedule.clone().sort();

    println!("{}", best);
    println!("same partner: {}", same_partner(&best));
    println!(
        "twice repeated opponents: {}",
        twice_repeated_opponents(&best)
    );
    println!("unequal byes: {}", unequal_byes(&best));
    println!("total: {}", total(&best));
    //println!("{}", best.serialize());
}

fn get_start<'a,'b>(num: usize, num_players: usize, rng: &'b mut ThreadRng) -> Vec<(Schedule<'a>, i32)> {
    todo!();
    /*
    (0..num)
        .map(|_| Schedule::new_random_order(num_players, rng))
        .map(|s| {
            let e = total(&s);
            (s, e)
        })
        .collect()
    */
}

fn move_same_partners<'a,'b>(schedule: Schedule<'a>, rng: &'b mut ThreadRng) -> Schedule<'a> {
    let players: Vec<Player> = schedule.players().into_iter().copied().collect();
    let mut new_rounds: Vec<Round> = schedule.rounds.clone();
    for p in &players {
        let games = schedule.find_all_games_from_player(p);
        let opponent_counts = games
            .find_all_opponents_as_players(p)
            .into_iter()
            .counts();
        let opponents_for_switching = players.iter()
            .map(|p| (p, opponent_counts.get(p).unwrap_or(&0)))
            .filter(|(_, count)| **count > 2)
            .filter(|(other, _)| *other > p) // don't undo everything!
            .map(|(p, _)| *p)
            .collect::<Vec<Player>>();
        for opponent in opponents_for_switching.clone() {
            new_rounds = new_rounds
                .into_iter()
                .map(|round: Round| {
                    match players.get_random_player_not_any(rng, &[*p, opponent]) {
                        Some(to_switch) => { round.swap_players(&opponent, to_switch) }
                        None => round,
                    }
                }).collect();
        }
    }
    Schedule { rounds: new_rounds }
}

/*
fn move_same_partners_new_opponent<'a,'b>(mut schedule: Schedule, rng: &'b mut ThreadRng) -> Schedule<'a> {
    let players: Vec<Player> = schedule.players().into_iter().copied().collect();
    for p in &players {
        let games = schedule.find_all_games_from_player(p);
        let player_counts = games
            .into_iter()
            .map(|g| g.find_opponents(p).unwrap())
            .flat_map(|partners| partners.players())
            .counts();
        // pick one round,
        // swap with someone who will guaruntee a different opponent

        let mut rounds = schedule.rounds.clone();
        let round_num = rng.gen_range(0..rounds.len());
        let mut new_round = rounds[round_num].clone();
        for player in &players {
            if player != p {
                let opponents_contain_player = schedule
                    .players()
                    .iter()
                    .map(|p| (p, player_counts.get(p).unwrap_or(&0)))
                    .filter(|(_, count)| **count > 3)
                    .filter(|(other, _)| **other > p) // don't undo everything!
                    .map(|(p, _)| **p)
                    .contains(player);
                if !opponents_contain_player {
                    new_round = new_round.swap_players(p, player);
                }
            }
        }
        rounds[round_num] = new_round;
        schedule.rounds = rounds
    }
    schedule
}
*/
