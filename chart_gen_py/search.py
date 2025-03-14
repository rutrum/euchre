# here, we want to manually cycle through players
# and slowly add players to round, at each step verifying
# constraints on the table. 

# IMPORTANT: we will use 0 as "no one seated yet".  This means
# we are indexing at 1 as apposed to 0 for players

import numpy as np
from itertools import permutations # maybe
from pprint import pprint
from line_profiler import profile

def get_table_seats(seat) -> tuple[int, int, tuple[int, int]]:
    partner = seat + 1 - 2 * (seat % 2)

    table = seat // 4
    if seat % 4 < 2:
        opponents = (table * 4 + 2, table * 4 + 3)
    else:
        opponents = (table * 4 + 0, table * 4 + 1)
        
    return (seat, partner, opponents)

def get_table_players(round, seat):
    player, partner, opponents = get_table_seats(seat)
    return (
        round[player],
        round[partner],
        (round[opponents[0]], round[opponents[1]])
    )

class Chart:
    def __init__(self, num_players):
        """Create a blank table."""
        self.num_players = num_players
        self.num_rounds = num_players - 1

        self.rounds = np.zeros((self.num_rounds, self.num_players), dtype=int)

        # padded so we can use 1-based indexing
        self.partner_counts = np.zeros((num_players + 1, num_players + 1), dtype=int)

        # ignore for first pass
        self.opponent_counts = np.zeros_like(self.partner_counts, dtype=int)


    def get_partner_counts(self, left: int, right: int) -> int:
        if left < right:
            return self.partner_counts[left, right]
        else:
            return self.partner_counts[right, left]

    def inc_partner_counts(self, left: int, right: int) -> int:
        if left < right:
            self.partner_counts[left, right] += 1
        else:
            self.partner_counts[right, left] += 1

    def dec_partner_counts(self, left, right):
        if left < right:
            self.partner_counts[left, right] -= 1
        else:
            self.partner_counts[right, left] -= 1

    def get_opponent_counts(self, left: int, right: int) -> int:
        if left < right:
            return self.opponent_counts[left, right]
        else:
            return self.opponent_counts[right, left]

    def inc_opponent_counts(self, left: int, right: int) -> int:
        if left < right:
            self.opponent_counts[left, right] += 1
        else:
            self.opponent_counts[right, left] += 1

    def dec_opponent_counts(self, left, right):
        if left < right:
            self.opponent_counts[left, right] -= 1
        else:
            self.opponent_counts[right, left] -= 1

    @profile
    def set(self, round_num, seat, player) -> int:
        self.rounds[round_num, seat] = player
        player, partner, opponents = get_table_players(self.rounds[round_num], seat)

        if partner != 0:
            self.inc_partner_counts(player, partner)

        for opp in opponents:
            if opp != 0:
                self.inc_opponent_counts(player, opp)

        return (
            self.get_partner_counts(player, partner),
            (
                int(self.get_opponent_counts(player, opponents[0])),
                int(self.get_opponent_counts(player, opponents[1])),
            )
        )

    @profile
    def unset(self, round_num, seat):
        player, partner, opponents = get_table_players(self.rounds[round_num], seat)
        self.rounds[round_num, seat] = 0

        if partner != 0:
            self.dec_partner_counts(player, partner)

        for opp in opponents:
            if opp != 0:
                self.dec_opponent_counts(player, opp)
        
    def __str__(self):
        s = ""
        for round in self.rounds:
            for player in round:
                s += f"{player:<2} "
            s += "\n"
        return s


def dfs_init(chart):
    return dfs(chart, list(range(1, chart.num_players + 1)), 0, 0)

# speed up idea: remove recursion
@profile
def dfs(chart, players, round, seat):
    for player in players:
        partner_count, opponent_counts = chart.set(round, seat, player)

        if partner_count > 1 or opponent_counts[0] > 2 or opponent_counts[1] > 2:
            chart.unset(round, seat)
            continue

        # great, recurse
        #if round == chart.num_rounds - 1 and seat == chart.num_players - 1:
        if round == 3 and seat == chart.num_players - 1:
            # finished, return
            return chart

        if seat == chart.num_players - 1:
            response = dfs(chart, list(range(1, chart.num_players+1)), round=round+1, seat=0)
        else:
            next_players = players[:]
            next_players.remove(player)
            response = dfs(chart, next_players, round=round, seat=seat+1)

        if response is not None:
            return response

        # no response found, go to next iteration
        chart.unset(round, seat)

@profile
def main():
    chart = Chart(8)
    print(dfs_init(chart))
    pprint(chart.partner_counts)
    pprint(chart.opponent_counts)

if __name__ == "__main__":
    main()