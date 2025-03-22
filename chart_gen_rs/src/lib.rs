/// Returns the seat of the partner of player at seat
const fn get_partner_seat(seat: usize) -> usize {
    seat + 1 - 2 * (seat % 2)
}

/// Returns the seats of the player, their partner, and the two opponents
pub const fn get_table_seats(seat: usize) -> (usize, usize, (usize, usize)) {
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

pub type Player = u8;

pub const fn get_table_players<const SEATS: usize>(
    seat: usize,
    round: &[Player; SEATS],
) -> (Player, Player, (Player, Player)) {
    let (player_seat, partner_seat, opponent_seats) = get_table_seats(seat);
    let player = round[player_seat];
    let partner = round[partner_seat];
    let opponents = (round[opponent_seats.0], round[opponent_seats.1]);
    (player, partner, opponents)
}
