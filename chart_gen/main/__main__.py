from . import *
from .pairchart import *
from pprint import pprint

import random
import copy

def init_pool(pool_size):
    pool = [ RotationTable() for _ in range(pool_size) ]
    [ rt.shuffle() for rt in pool ]
    return pool

def generational_search():
    pool = init_pool(1000)

    num_generations = 1000
    top_stay = 200
    every_other = 200
    brand_new = 50
    mutate = num_generations-top_stay-every_other-brand_new

    best_score = 100000
    best_table = None

    # parallel?
    for generation in range(num_generations):
        if generation % 100 == 99:
            print("Generation", generation)
            print(len(pool))
        # eval
        costs = [ (rt.cost(), rt) for rt in pool ]
        costs = sorted(costs, key=lambda x: x[0])
        pool = [ x[1] for x in costs ]

        if costs[0][0] < best_score:
            best_score = costs[0][0]
            best_table = costs[0][1]
            print(f"New best score: {best_score}")
            if best_score == 0:
                break
        
        best = pool[:top_stay]

        newbies = [ RotationTable() for _ in range(brand_new) ]
        [ rt.shuffle() for rt in newbies ]

        mutated = []
        for _ in range(mutate):
            i = random.randrange(top_stay)
            num_mut = random.randrange(1, 4)
            base = copy.deepcopy(best[i])
            for _ in range(num_mut):
                base.random_swap()
            mutated.append(base)

        not_so_good = pool[top_stay:top_stay+every_other*2:2]
            
        pool = best + mutated + not_so_good + newbies

    print(f"Best table with score {best_score}:")
    print(best_table)

if __name__ == "__main__":
    #generational_search()
    rc = RotationChartPairs()