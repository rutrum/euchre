/// Returns the seat of the partner of player at seat
const fn get_partner_seat(seat: usize) -> usize {
    seat + 1 - 2 * (seat % 2)
}

/// Returns the seats of the player, their partner, and the two opponents
pub const fn get_table_seats(seat: usize) -> (usize, usize, (usize, usize)) {
    let player = seat;
    let partner = get_partner_seat(seat);
    let table = seat / 4;

    let left = if seat % 4 < 2 {
        table * 4 + 2
    } else {
        table * 4 + 0
    };

    (player, partner, (left, left + 1))
}

/// Returns the seats of the player, their partner, and the two opponents
pub const fn get_table_seats_unordered(seat: usize) -> (usize, usize, (usize, usize)) {
    let player = seat;
    let partner = get_partner_seat(seat);

    if seat % 4 < 2 {
        (player, partner, (player + 2, partner + 2))
    } else {
        (player, partner, (player - 2, partner - 2))
    }
}

pub type Player = u8;

pub const fn get_table_players<const SEATS: usize>(
    seat: usize,
    round: &[Player; SEATS],
) -> (Player, Player, (Player, Player)) {
    let (player_seat, partner_seat, (left_seat, right_seat)) = get_table_seats(seat);
    let player = round[player_seat];
    let partner = round[partner_seat];
    let opponents = (round[left_seat], round[right_seat]);

    (player, partner, opponents)
}

pub const fn get_table_players_unordered<const SEATS: usize>(
    seat: usize,
    round: &[Player; SEATS],
) -> (Player, Player, (Player, Player)) {
    let (player_seat, partner_seat, (left_seat, right_seat)) = get_table_seats_unordered(seat);
    let player = round[player_seat];
    let partner = round[partner_seat];
    let opponents = (round[left_seat], round[right_seat]);

    (player, partner, opponents)
}
