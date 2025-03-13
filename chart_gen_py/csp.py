import constraint
import string
from itertools import product

NUM_PLAYERS = 8
NUM_ROUNDS = 7

def print_solution(solution):
    print(solution)
    for round in string.ascii_lowercase[:NUM_ROUNDS]:
        print(f"ROUND {round}: ", end="")
        for seat in range(1, NUM_PLAYERS+1):
            key = f"{round}{seat}"
            value = solution[key]
            print(f"{value:>3}", end="")
        print()

if __name__ == "__main__":
    problem = constraint.Problem()

    players = list(range(1, NUM_PLAYERS+1))
    rounds = string.ascii_lowercase[:NUM_ROUNDS]

    player_product = 1
    for p in players:
        player_product *= p

    for round in rounds:
        for seat in players:
            problem.addVariable(f"{round}{seat}", players)

    # constraint 1: valid table
    # every row has each person exactly once
    for round in rounds:
        round_players = [ f"{round}{p}" for p in players ]
        problem.addConstraint(constraint.AllDifferentConstraint(), round_players)
        #for left, right in product(round_players, round_players):
        #    if left != right:
        #        problem.addConstraint(f"{left} != {right}")

    # constraint 2: 
    
    print_solution(problem.getSolution())