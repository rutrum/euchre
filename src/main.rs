use euchre_tournament::*;

fn main() {
    trivial4(Player::many(4, 1)).print();

    chart8().print();
}
