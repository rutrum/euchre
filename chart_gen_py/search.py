# here, we want to manually cycle through players
# and slowly add players to round, at each step verifying
# constraints on the table. 

# IMPORTANT: we will use 0 as "no one seated yet".  This means
# we are indexing at 1 as apposed to 0 for players

import numpy as np
from itertools import permutations # maybe
from pprint import pprint

def get_table_seats(seat) -> tuple[int, int, tuple[int, int]]:
    player = seat;
    partner = seat + 1 - 2 * (seat % 2)

    table = seat / 4;
    if seat % 4 < 2:
        opponents = (table * 4 + 2, table * 4 + 3)
    else:
        opponents = (table * 4 + 0, table * 4 + 1)

    return (player, partner, opponents)

class Chart:
    def __init__(self, num_players):
        """Create a blank table."""
        self.num_players = num_players
        self.num_rounds = num_players - 1

        self.rounds = np.zeros((self.num_rounds, self.num_players), dtype=int)

        # padded so we can use 1-based indexing
        self.partner_counts = np.zeros((num_players + 1, num_players + 1), dtype=int)

        # ignore for first pass
        #self.opponent_counts = np.zeros_like(self.partner_counts)

    def get_partner_counts(self, left: int, right: int) -> int:
        if left < right:
            return self.partner_counts[left, right]
        else:
            return self.partner_counts[right, left]

    def inc_partner_counts(self, left: int, right: int) -> int:
        if left < right:
            self.partner_counts[left, right] += 1
            return self.partner_counts[left, right]
        else:
            self.partner_counts[right, left] += 1
            return self.partner_counts[right, left]

    def dec_partner_counts(self, left, right):
        if left < right:
            self.partner_counts[left, right] -= 1
        else:
            self.partner_counts[right, left] -= 1

    def set(self, round, seat, player) -> int:
        self.rounds[round, seat] = player
        _, partner_seat, opponent_seats = get_table_seats(seat)
        if (partner := self.rounds[round, partner_seat]) != 0:
            return self.inc_partner_counts(player, partner)
        else:
            return self.get_partner_counts(player, partner)

    def unset(self, round, seat):
        player = self.rounds[round, seat]
        self.rounds[round, seat] = 0
        _, partner_seat, opponent_seats = get_table_seats(seat)
        if (partner := self.rounds[round, partner_seat]) != 0:
            self.dec_partner_counts(player, partner)
        
    def __str__(self):
        s = ""
        for round in self.rounds:
            for player in round:
                s += f"{player:<2} "
            s += "\n"
        return s


def dfs(chart, round=0, seat=0):
    for player in range(1, chart.num_players+1):
        if player not in chart.rounds[round]:
            partner_count = chart.set(round, seat, player)
            print(round, seat, player, "=", partner_count)
            if partner_count > 1:
                chart.unset(round, seat)
                continue

            # great, recurse
            if round == chart.num_rounds - 1 and seat == chart.num_players - 1:
                # finished, return
                return chart
            elif seat == chart.num_players - 1:
                response = dfs(chart, round=round+1, seat=0)
            else:
                response = dfs(chart, round=round, seat=seat+1)

            if response is not None:
                return response

            
            # no response found, go to next iteration
            chart.unset(round, seat)

if __name__ == "__main__":

    # this slows down at 20
    # try using Cython for fun
    # this is decoration that "precompiles" python for you, or something

    chart = Chart(16)
    print(dfs(chart))
    pprint(chart.partner_counts)