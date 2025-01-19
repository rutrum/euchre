import numpy as np
from collections import defaultdict
import json

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
        ], dtype="u8")
        for round in self.chart:
            np.random.shuffle(round)

        self.skip_chance = 0.2
        
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
                ahead = self.chart[round, 4 * table + 1]
                left = self.chart[round, 4 * table + 2]
                right = self.chart[round, 4 * table + 3]
                self.opponents_count[Pair(me, left)] += 1
                self.opponents_count[Pair(me, right)] += 1
                self.opponents_count[Pair(left, ahead)] += 1
                self.opponents_count[Pair(right, ahead)] += 1

    def swap_players(self, round, a_seat, b_seat):
        a = self.chart[round, a_seat]
        b = self.chart[round, b_seat]

        # First update partner counds
        a_partner_seat = get_partner_seat(a_seat)
        a_partner = self.chart[round, a_partner_seat]
        b_partner_seat = get_partner_seat(b_seat)
        b_partner = self.chart[round, b_partner_seat]

        self.partners_count[Pair(a, a_partner)] -= 1
        self.partners_count[Pair(b, b_partner)] -= 1
        self.partners_count[Pair(a, b_partner)] += 1
        self.partners_count[Pair(b, a_partner)] += 1

        # Now update opponent counts
        a_opponent_seats = get_opponent_seats(a_seat)
        a_opponents = [ self.chart[round, seat] for seat in a_opponent_seats ]
        b_opponent_seats = get_opponent_seats(b_seat)
        b_opponents = [ self.chart[round, seat] for seat in b_opponent_seats ]

        for opp in a_opponents:
            self.opponents_count[Pair(a, opp)] -= 1
            self.opponents_count[Pair(b, opp)] += 1
        for opp in b_opponents:
            self.opponents_count[Pair(b, opp)] -= 1
            self.opponents_count[Pair(a, opp)] += 1

        # do this last ;)
        self.chart[round, a_seat], self.chart[round, b_seat] = self.chart[round, b_seat], self.chart[round, a_seat]

    def bad_partners(self) -> bool:
        """True when partners are matched together more than once."""
        for k, v in self.partners_count.items():
            if v > 1:
                return True
        return False

    def bad_opponents(self) -> bool:
        """True when opponents are matched together more than twice."""
        for k, v in self.opponents_count.items():
            if v > 2:
                return True
        return False

    def refine(self):
        max_loops = 100000
        cur = 0
        while cur < max_loops and (self.bad_partners() or self.bad_opponents()):
            loop_total = 0
            for round in range(self.num_rounds):
                loop_total += self.refine_round(round)
            cur += loop_total
            #print(cur, loop_total)
        print(cur)

    def refine_round(self, round):
        total_swaps = 0
        for player_seat in range(self.num_players):
            player = self.chart[round, player_seat]

            partner_seat = get_partner_seat(player_seat)
            partner = self.chart[round, partner_seat]
            opponent_seats = get_opponent_seats(player_seat)
            opponents = [ self.chart[round, opponent_seat] for opponent_seat in opponent_seats ]

            table_seats = opponent_seats + [player_seat, partner_seat]
            table_players = [
                self.chart[round, player]
                for player
                in table_seats
            ]

            # now check if players are good
            if self.partners_count[Pair(player, partner)] > 1:
                # too many times, switch this up
                for another_player_seat in range(self.num_players):
                    if another_player_seat in table_seats:
                        continue

                    another_player = self.chart[round, another_player_seat]
                    another_partner_seat = get_partner_seat(another_player_seat)
                    another_partner = self.chart[round, another_partner_seat]
                    
                    # during a swap, theres 6 metrics that change
                    # I should consider swapping if some number of those criteria are met

                    #if (self.partners_count[Pair(another_player, player)] == 0 and
                    #    self.partners_count[Pair(partner, another_partner)] == 0):
                    if self.swap_improvements(round, player, another_player) > 4 and np.random.random() > self.skip_chance:
                        #print(f"round {round}: swapping {another_player} and {partner}")
                        #print(self.chart[round])
                        self.swap_players(round, another_player_seat, partner_seat)
                        #print(self.chart[round])
                        total_swaps += 1
                        break # partner score closer to 1
            
            for opponent in opponents:
                if self.opponents_count[Pair(player, opponent)] > 2:
                    for another_player_seat in range(self.num_players):
                        # this check has to be done without following reference
                        if another_player_seat in table_seats:
                            continue
                        another_player = self.chart[round, another_player_seat]

                        # see if making the swap improves something else
                        if self.swap_improvements(round, player_seat, another_player_seat) > 4 and np.random.random() > self.skip_chance:
                            print(f"round {round}: swapping {another_player} and {partner}")
                            self.swap_players(round, player_seat, another_player_seat)
                            total_swaps += 1
                            break # partner score closer to 1
        return total_swaps

    def swap_improvements(self, round, seat_a, seat_b):
        """Returns a score from 1-6 based on how many improvements are made to counts."""
        a = self.chart[round, seat_a]
        a_partner = self.chart[round, get_partner_seat(seat_a)]
        a_opps = [ self.chart[round, opp_seat] for opp_seat in get_opponent_seats(seat_a) ]

        b = self.chart[round, seat_b]
        b_partner = self.chart[round, get_partner_seat(seat_b)]
        b_opps = [ self.chart[round, opp_seat] for opp_seat in get_opponent_seats(seat_b) ]
        
        return (
            (self.partners_count[Pair(a, b_partner)] < 1)
            + (self.partners_count[Pair(b, a_partner)] < 1)
            + (self.opponents_count[Pair(a, b_opps[0])] < 2)
            + (self.opponents_count[Pair(a, b_opps[1])] < 2)
            + (self.opponents_count[Pair(b, a_opps[0])] < 2)
            + (self.opponents_count[Pair(b, a_opps[1])] < 2)
        )

    def seat(self, round, player):
        """Returns the seat of the player in the round."""
        for seat in range(self.num_players):
            if self.chart[round, seat] == player:
                return seat

    def sort_chart(self):
        for round in range(self.num_rounds):
            # first sort the partners
            for seat in np.arange(0, self.num_players, step=2):
                player = self.chart[round, seat]
                partner = self.chart[round, seat+1]
                if player > partner:
                    self.chart[round, seat] = partner
                    self.chart[round, seat+1] = player

            # then sort apponents
            for seat in np.arange(0, self.num_players, step=4):
                player = self.chart[round, seat]
                opponent = self.chart[round, seat+2]
                if player > opponent:
                    partner = self.chart[round, seat+1]
                    opp2 = self.chart[round, seat+3]
                    self.chart[round, seat] = opponent
                    self.chart[round, seat+1] = opp2
                    self.chart[round, seat+2] = player
                    self.chart[round, seat+3] = partner

    def player_info(self, player):
        rounds = []
        for round in range(self.num_rounds):
            seat = self.seat(round, player)
            rounds.append({
                "seat": seat,
                "table": seat // 4,
                "partner": int(self.chart[round, get_partner_seat(seat)]),
                "opponents": [
                    int(self.chart[round, opp])
                    for opp in get_opponent_seats(seat)
                ],
            })
        return rounds


    def to_json(self):
        # this may not even be necessary, why not just return the numpy 2d array?
        d = {
            "num_players": self.num_players,
            "num_rounds": self.num_rounds,
            "chart": [
                [
                    int(self.chart[i_round][i_player])
                    for i_player in range(self.num_players)
                ]
                for i_round in range(self.num_rounds)
            ],
            "players": {
                int(i): self.player_info(i)
                for i in range(self.num_players)
            }
        }
        return json.dumps(d, indent=None)

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
    rt = Chart(num_players=32, num_rounds=12)
    print(rt)
    # If it stops after the first round,
    # then the counts should be 1 or less,
    # but it stops swapping anymore
    print(list(sort_vals(rt.partners_count).items())[:10])
    print(list(sort_vals(rt.opponents_count).items())[:10])
    print(f"bad partners: {rt.bad_partners()}")
    print(f"bad opponents: {rt.bad_opponents()}")

    rt.sort_chart()

    # filename = f"data/{rt.num_players}players_{rt.num_rounds}rounds.json"
    filename = "data/chart.json"
    with open(filename, "w") as f:
        f.write(rt.to_json())


if __name__ == "__main__":
    main()
