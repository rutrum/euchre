"""A RotationChart implementation using pairs.

For the 4n rotation chart, we want to enforce the constraint that each
player is partners with every other player exactly once.  If we consider
the 4n choose 2 pairs, we can restructure the rotation chart in a way
that this constraint is always enforced.

We do this by constructing keys that represent every combination of 2
players.  For each round we consider 4n/2 of these pairs across 4n-1 rounds.

We then only have to search for the second constraint, because any swap in
pairs in a round will still yield a valid rotation chart with the
first constraint still met.

PROBLEM: If I want to keep this constaint, it's not longer trivial
to maintain the constraint that each round has each person a single time.

Once I have such a valid table, I think I could perform swaps that
are smart and maintain both constraints.
"""

import itertools

PRIMES = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79]

round_hash = product(PRIMES[:12])

def hash_pair(pair):
    a, b = pair
    return PRIMES[a] * PRIMES[b]

def unhash_pair(hash):
    for p in PRIMES:
        if hash % p == 0:
            return (PRIMES.index(p), PRIMES.index(hash // p))
    raise ValueError("hash not factor of primes")

class RotationChartPairs:
    def __init__(self, players=12):
        # first build all combinations of 12 players
        pairs = sorted({ (p1, p2) for p1 in range(players) for p2 in range(players) })
        self.players = players
        print(pairs)

        # COOL IDEA: is a round valid?
        # then the product of hashes should be exactly the product of the first
        # players primes
        
        self.rounds = []
        for round in range(players):
            # how to build a valid chart from this?
            # valid as in, each player is present in each round once
            pass

    def _add_pair(self, rounds, round, pairs):
        if len(pairs) == 0:
            if product(round) == round_hash:
                return True, rounds
            else:
                return False, rounds

        p = pairs[0]
        pairs = pairs[1:]
        round.append(p)

        if len(round) == players // 2:
            if product(round) == round_hash:
                # great, next round
                rounds += round
                result, final = self._add_pair(rounds, [], pairs)
                if result:
                    return True, final
                else:
                    # it failed, undo last

            