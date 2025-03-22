# here, we want to manually cycle through players
# and slowly add players to round, at each step verifying
# constraints on the table. 

# IMPORTANT: we will use 0 as "no one seated yet".  This means
# we are indexing at 1 as apposed to 0 for players

import numpy as np
from itertools import permutations # maybe
from pprint import pprint
from line_profiler import profile

def get_table_seats(seat) -> tuple[int, int, int, int]:
    partner = seat + 1 - 2 * (seat % 2)

    table = seat // 4
    if seat % 4 < 2:
        opponents = (table * 4 + 2, table * 4 + 3)
    else:
        opponents = (table * 4 + 0, table * 4 + 1)
        
    return (seat, partner, opponents[0], opponents[1])

def get_table_players(round, seat):
    partner = seat + 1 - 2 * (seat % 2)

    table = seat // 4
    if seat % 4 < 2:
        opponents = (table * 4 + 2, table * 4 + 3)
    else:
        opponents = (table * 4 + 0, table * 4 + 1)

    return (
        round[seat],
        round[partner],
        round[opponents[0]], 
        round[opponents[1]],
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


    @profile
    def set(self, round_num, seat, player) -> int:
        self.rounds[round_num, seat] = player
        player, partner, left, right = get_table_players(self.rounds[round_num], seat)

        partner_count, left_count, right_count = 0, 0, 0

        if partner != 0:
            if player < partner:
                self.partner_counts[player, partner] += 1
                partner_count = self.partner_counts[player, partner]
            else:
                self.partner_counts[partner, player] += 1
                partner_count = self.partner_counts[partner, player]

        if left != 0:
            if player < left:
                self.opponent_counts[player, left] += 1
                left_count = self.opponent_counts[player, left]
            else:
                self.opponent_counts[left, player] += 1
                left_count = self.opponent_counts[left, player]

        if right != 0:
            if player < right:
                self.opponent_counts[player, right] += 1
                right_count = self.opponent_counts[player, right]
            else:
                self.opponent_counts[right, player] += 1
                right_count = self.opponent_counts[right, player]

        return (partner_count, left_count, right_count)

    @profile
    def unset(self, round_num: int, seat: int) -> int:
        player, partner, left, right = get_table_players(self.rounds[round_num], seat)
        self.rounds[round_num, seat] = 0

        if partner != 0:
            if player < partner:
                self.partner_counts[player, partner] -= 1
            else:
                self.partner_counts[partner, player] -= 1

        if left != 0:
            if player < left:
                self.opponent_counts[player, left] -= 1
            else:
                self.opponent_counts[left, player] -= 1

        if right != 0:
            if player < right:
                self.opponent_counts[player, right] -= 1
            else:
                self.opponent_counts[right, player] -= 1
        
        return player
        
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
        partner_count, left_count, right_count = chart.set(round, seat, player)

        #if partner_count > 1 or left_count > 2 or right_count > 2:
        if partner_count > 1:
            chart.unset(round, seat)
            continue

        # great, recurse
        if round == chart.num_rounds - 1 and seat == chart.num_players - 1:
        #if round == 3 and seat == chart.num_players - 1:
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
def dfs_loop(chart):
    all_players = list(range(1, chart.num_players + 1))
    round_players = []
    round, seat = 0, 0
    player = 0

    while True:
        # try next player
        if player < chart.num_players:
            player += 1
        else:
            # that was the last player, so go to last seat
            if seat > 0:
                seat -= 1
            elif round > 0:
                round -= 1
                seat = chart.num_players - 1
                round_players = all_players[:]
            else:
                # tried everything, failed
                return None
            
            # remove the player and start the next one
            player = chart.unset(round, seat)
            round_players.remove(player)
            continue
            
        if player in round_players:
            continue

        partner_count, left_count, right_count = chart.set(round, seat, player)

        if partner_count > 1:
        #if partner_count > 1 or left_count > 2 or right_count > 2:
            # this player didn't work out
            chart.unset(round, seat)
            continue
        else:
            # valid player assignment, go to next seat
            if seat < chart.num_players - 1:
                seat += 1
                round_players.append(player)

            #elif round < 3:
            elif round < chart.num_rounds - 1:
                seat = 0
                round_players = []

                round += 1
            else:
                # all players assigned
                break

            player = 0 # start over

    return chart

@profile
def dfs_set_conditional(chart):
    all_players = list(range(1, chart.num_players + 1))
    round_players = []
    round, seat = 0, 0
    player = 1

    while True:
        _, partner, left, right = get_table_players(chart.rounds[round], seat)
        
        pp = (player, partner) if player < partner else (partner, player)
        pl = (player, left) if player < left else (left, player)
        pr = (player, right) if player < right else (right, player)

        if (
            (partner == 0 or chart.partner_counts[pp] < 1)
            and (left == 0 or chart.opponent_counts[pl] < 2) and (right == 0 or chart.opponent_counts[pr] < 2)
        ):
            # valid player assignment, go to next seat
            chart.rounds[round, seat] = player

            if partner > 0:
                chart.partner_counts[pp] += 1
            if left > 0:
                chart.opponent_counts[pl] += 1 
            if right > 0:
                chart.opponent_counts[pr] += 1

            if seat < chart.num_players - 1:
                seat += 1
                round_players.append(player)

            elif round < 3:
            #elif round < chart.num_rounds - 1:
                seat = 0
                round_players = []

                round += 1
                player = 1
                continue
            else:
                # all players assigned
                break

            next_player = 1 # start over
        else:
            next_player = player + 1

        # it appears that moving the increment to the bottom did not work
        # not sure why
        # this loop certainly doesn't help, I dont think, but it beats 
        # the previous version, surely?
        while next_player in round_players:
            next_player += 1

        if next_player > chart.num_players:
            # that was the last player, so go to last seat
            if seat > 0:
                seat -= 1
            elif round > 0:
                round -= 1
                seat = chart.num_players - 1
                round_players = all_players[:]
            else:
                # tried everything, failed
                return None
            
            # remove the player and start the next one
            player = chart.unset(round, seat)
            round_players.remove(player)
            continue
        else:
            player = next_player

    return chart

@profile
def main():
    #chart = Chart(12)
    #print(dfs_init(chart))
    #pprint(chart.partner_counts)
    #pprint(chart.opponent_counts)

    chart = Chart(8)
    print(dfs_set_conditional(chart))
    pprint(chart.partner_counts)
    pprint(chart.opponent_counts)

if __name__ == "__main__":
    main()

# TODO
# do this whole process, but do the pair-centric approach
# in this example, trying to find just the 6th round of the 8 person
# table is incredibly slow with my fastest algorithm