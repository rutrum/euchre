import numpy as np
from collections import defaultdict

def get_partner_seat(player_seat):
    """Return the seat number of the partner of a player seat."""
    if player_seat % 2 == 0:
        return player_seat + 1
    else:
        return player_seat - 1

def get_opponent_seats(player_seat):
    """Return the seats of the opponent of a player seat."""
    partner_seat = get_partner_seat(player_seat)
    if player_seat % 4 < 2:
        return [player_seat + 2, partner_seat + 2]
    else:
        return [player_seat - 2, partner_seat - 2]

class Pair:
    """Ordered pair of players."""

    def __init__(self, x, y):
        if x < y:
            self.x = x
            self.y = y
        else:
            self.x = y
            self.y = x

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y

    def __hash__(self):
        return hash((self.x, self.y))

    def __repr__(self):
        return f"({self.x}, {self.y})"


class Chart:
    def __init__(self, num_players, num_rounds):
        self.num_players = num_players
        self.num_rounds = num_rounds
        self.chart = np.asarray([
            np.arange(self.num_players)
            for round in range(self.num_rounds)
        ])
        for round in self.chart:
            np.random.shuffle(round)
        self.init_partners_count()
        self.init_opponents_count()
        self.refine()

    def init_partners_count(self):
        self.partners_count = defaultdict(lambda: 0)
        for round in range(self.num_rounds):
            for i in range(self.num_players // 2):
                left = self.chart[round, 2 * i]
                right = self.chart[round, 2 * i + 1]
                self.partners_count[Pair(left, right)] += 1

    def init_opponents_count(self):
        self.opponents_count = defaultdict(lambda: 0)
        for round in range(self.num_rounds):
            for table in range(self.num_players // 4):
                me = self.chart[round, 4 * table]
                left = self.chart[round, 4 * table + 1]
                ahead = self.chart[round, 2 * table + 2]
                right = self.chart[round, 2 * table + 3]
                self.opponents_count[Pair(me, left)] += 1
                self.opponents_count[Pair(me, right)] += 1
                self.opponents_count[Pair(left, ahead)] += 1
                self.opponents_count[Pair(right, ahead)] += 1

    def seat(self, round, player):
        return self.chart[round, player]

    def swap_players(self, round, a_seat, b_seat):
        a = self.seat(round, a_seat)
        a_partner_seat = get_partner_seat(a_seat)
        a_partner = self.seat(round, a_partner_seat)
        b = self.seat(round, b_seat)
        b_partner_seat = get_partner_seat(b_seat)
        b_partner = self.seat(round, b_partner_seat)

        self.partners_count[Pair(a, a_partner)] -= 1
        self.partners_count[Pair(b, b_partner)] -= 1
        self.partners_count[Pair(a, b_partner)] += 1
        self.partners_count[Pair(b, a_partner)] += 1

        # do this last ;)
        self.chart[round, a_seat], self.chart[round, b_seat] = self.chart[round, b_seat], self.chart[round, a_seat]

    def bad_partners(self) -> bool:
        for k, v in self.partners_count.items():
            if v > 1:
                return True
        return False

    def refine(self):
        max_loops = 10
        cur = 0
        while cur < max_loops and self.bad_partners():
            loop_total = 0
            for round in range(self.num_rounds):
                loop_total += self.refine_round(round)
            cur += 1
            print(cur, loop_total)
        print(cur)

    def refine_round(self, round):
        total_swaps = 0
        for player_seat in range(self.num_players):
            player = self.chart[round, player_seat]

            partner_seat = get_partner_seat(player_seat)
            partner = self.chart[round, partner_seat]
            opponent_seats = get_opponent_seats(player_seat)

            table_players = [
                self.chart[round, player]
                for player
                in opponent_seats + [player_seat, partner_seat]
            ]

            # now check if players are good
            if self.partners_count[Pair(player, partner)] > 1:
                # too many times, switch this up
                for another_player_seat in range(self.num_players):
                    another_player = self.chart[round, another_player_seat]
                    if another_player in list(table_players):
                        continue

                    another_partner_seat = get_partner_seat(another_player_seat)
                    another_partner = self.chart[round, another_partner_seat]
                    
                    if (self.partners_count[Pair(another_player, player)] == 0 and
                        self.partners_count[Pair(partner, another_partner)] == 0):
                        print(f"round {round}: swapping {another_player} and {partner}")
                        print(self.chart[round])
                        self.swap_players(round, another_player_seat, partner_seat)
                        print(self.chart[round])
                        total_swaps += 1
                        break # partner score closer to 1
        return total_swaps

    def __str__(self):
        s = ""
        for row in self.chart:
            for player in row:
                s += f"{player:<2} "
            s += "\n"
        return s

def sort_vals(d):
    return {k: v for k, v in sorted(d.items(), key=lambda item: -item[1])}

def main():
    rt = Chart(
        num_players=32, num_rounds=15)
    print(rt)
    # If it stops after the first round,
    # then the counts should be 1 or less,
    # but it stops swapping anymore
    print(list(sort_vals(rt.partners_count).items())[:10])
    #print(list(sort_vals(rt.opponents_count).items())[:10])
    print(f"bad partners: {rt.bad_partners()}")


if __name__ == "__main__":
    main()
