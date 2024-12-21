from typing import Tuple, List
from model import *

def list_players(n: int, offset=0) -> List[Player]:
    return [ Player(i+offset+1) for i in range(n) ]

def trivial4(ps: Tuple[Player, Player, Player, Player]):
    rounds = [
        Round([Game(Pair(ps[0], ps[1]), Pair(ps[2], ps[3]))]),
        Round([Game(Pair(ps[0], ps[2]), Pair(ps[1], ps[3]))]),
        Round([Game(Pair(ps[0], ps[3]), Pair(ps[1], ps[2]))]),
    ]
    return RotationChart(rounds)

print(trivial4(list_players(4)))

def table_4(a=1, b=2, c=3, d=4):
    return [
        [[a, b], [c, d]],
        [[a, c], [b, d]],
        [[a, d], [b, c]],
    ]

def chart8():
    chart_rounds = []
    t1 = trivial4(list_players(4, 0))
    t2 = trivial4(list_players(4, 4))
    for i in range(len(t1)):
        chart_rounds.append(t1.round(i) + t2.round(i))

    return RotationChart(chart_rounds)

print(chart8())

def table_8():
    table_8 = []
    t1 = table_4()
    t2 = table_4(5,6,7,8)
    for i in range(len(t1)):
        new_round = t1[i] + t2[i]
        table_8.append(new_round)
    for k in range(4):
        new_round = []
        for i in range(4):
            new_round.append([i + 1, (i+k) % 4 + 5])
        table_8.append(new_round)
    return table_8

def print_table(t):
    for i in range(len(t)):
        print(f"Round {i}: ", end="")
        for pair in t[i]:
            print(f"{pair[0]}{pair[1]}, ", end="")
        print()

def count_players(pairs):
    count = {}
    for pair in pairs:
        for i in pair:
            if i in count:
                count[i] += 1
            else:
                count[i] = 1
    return count

def too_many_players(pairs):
    c = count_players(pairs).items()
    for player, count in c:
        if count > 2:
            return True
    return False

def find_games(t):

    def advance(picks, opponents):
        if len(picks) == 7:
            print("done!")
            print(picks)
            return
        round = len(picks)
        for i in range(3):
            pair = t[round][i+1]
            if not too_many_players(opponents + [pair]):
                picks.append(i+1)
                opponents.append(pair)
                advance(picks, opponents)
                if len(picks) == 7:
                    return
        picks.pop()
        opponents.pop()

    picks = []
    opponents = []
    advance(picks, opponents)

    new_table = []
    for i, round in enumerate(t):
        new_round = [
            [round[0], round[picks[i]]],
            [ round[j + 1] for j in range(3) if j+1 != picks[i]]
        ]
        new_table.append(new_round)
    print_table(new_table)


print_table(table_8())
find_games(table_8())
