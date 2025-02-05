# Let's try again but in python

# need to come up with the main data structure for a rotation table
# it should be a 2d array, and "tables" might be considered
# based on index values modulo 4, or index / 4

# Let's investigate the 12 or 16 person rotation table, not sure
# yet.  12 is easier (and I think there's a known optimal solution)
# but 16 is a power of 2.  Let's go with 12 for now, and if it's problematic
# we can upgrade later.

# I need to define a "cost" function that counts the number of "mistakes"
# These are the contrainst that I'm optimizing

# So first, the data structure

# new new idea
# create a big hashmap that takes any table in lexicographical order hashes
# it to a unique value that I can store instead?

# who a player is partnered with and who they are opponents with
# should be the upper and lower triangular matrices

# ONLY MOVE PARTNERS
# this would keep the contstrain that each player is with one another exactly once
# generate each pair of players, populate the rotation chart with that only
# when randomly swapping, just move pairs around.

from collections import defaultdict
import random
import itertools
#import numpy as np

PRIMES = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79]

# TODO: make immutable?
# I need an easy way to track if it's been updated or not
# or I need to track the opponents/pairs at the same time
# let's try this first: store all the pairs together,
# then modify random swap to update the pair list dynamically
# this might justify using a new datastructure to count the pairs,
# perhaps consider using the hash value as key to a dictionary
# oh that's nice, because I can just do this to check all partners of a player:
# for i in range(player_i, players*players, players):
#   hash = i * player_i
#   num_pairs = pairs[hash]
class RotationTable:
    def __init__(self, players=12):
        # players are represented 
        self.players = players
        self.rounds = []
        for _ in range(players-1):
            self.rounds.append(list(range(players)))
        self.updated = True

    def shuffle(self):
        for i in range(len(self.rounds)):
            random.shuffle(self.rounds[i])
        self.updated = True

    def random_swap(self):
        round = random.randrange(len(self.rounds))
        round = self.rounds[round]
        a, b = random.sample(range(self.players), 2)
        if a // 2 == b // 2:
            self.random_swap()
        else:
            round[a], round[b] = round[b], round[a]
        self.updated = True

    def find_rounds(self, pair: tuple[int, int]) -> list[int]:
        found = []
        for round in self.rounds:
            for i in range(0, len(round), 2):
                if sorted(round[i:i+2]) == sorted(pair):
                    found.append(i)
        return found

    def swap_problem(self, top=10):
        """Pick a person with a known problem and swap them.
        
        This is too slow.  Need to cache more things, calculate them
        less dumb.  Maybe even track the number of pairs dynamically.
        This would avoid recounting everytime to compute the cost.

        Ways to improve problem solving:
        * check to see if both opponents are bad, if so then swap the whole pair?
        * maybe shuffle an entire row with the most issues?
        * shuffle an entire rows with the least issues?

        I think I just need to try some non-intuitive things, since I think
        that's going to fix some issues.
        """
        pairs = self.pairs().items()
        pairs = sorted(pairs, key=lambda x: -x[1])
        # pick from top
        pair = pairs[random.randrange(top)][0]
        # find this pair
        round_nums = self.find_rounds(pair)
        round_i = random.choice(round_nums)

        player = pair[random.randrange(2)]
        other = player
        while other == player:
            other = random.randrange(self.players)
        
        player_loc = self.rounds[round_i].index(player)
        other_loc = self.rounds[round_i].index(other)
        self.rounds[round_i][player_loc], self.rounds[round_i][other_loc] = \
            self.rounds[round_i][other_loc], self.rounds[round_i][player_loc]
        
        self.updated = True

    # make some other helper functions, like getting
    # the location of a player in any round, or the parter of
    # that player, or the opponents
    # I should really make pairs/opponents dictionary a
    # constantly updated thing

    def pairs(self):
        if self.updated:
            pairs = defaultdict(lambda: 0)
            for round in self.rounds:
                for i in range(0, len(round), 2):
                    pair = tuple(sorted([round[i], round[i+1]]))
                    pairs[pair] += 1
            self._pairs = pairs
            return pairs
        else:
            return self._pairs

    def opponents(self):
        opponents = defaultdict(lambda: 0)
        for round in self.rounds:
            for i in range(0, len(round), 4):
                table = round[i:i+4]
                a, b, c, d = table
                pairs = [
                    [a, c],
                    [a, d],
                    [b, c],
                    [b, d],
                ]
                pairs = [ tuple(sorted(p)) for p in pairs ]
                for pair in pairs:
                    opponents[pair] += 1
        return opponents

    def cost(self):
        # cache this too?  Many trees stick around for many generations
        # or make this data structure immutable
        cost_unique_pairs = sum([ count - 1 for _, count in self.pairs().items() ])
        cost_min_opponents = sum([ max(count - 2, 0) for _, count in self.opponents().items() ])
        return cost_unique_pairs + cost_min_opponents

    def __str__(self):
        s = ""
        for i, round in enumerate(self.rounds):
            s += f"{i:>2} "
            for i in range(0, len(round), 4):
                table = round[i:i+4]
                if len(table) == 0:
                    continue
                a, b, c, d = tuple(table)
                #if a > b:
                #    a, b = b, a
                #if c > d:
                #    c, d = d, c
                s += f"[{a:>2} {b:>2}:{c:>2} {d:>2}]"
            s += "\n"
        return s

def hash_pair(a, b):
    return PRIMES[a] * PRIMES[b]

def unhash_pair(hash):
    for p in PRIMES:
        if hash % p == 0:
            return PRIMES.index(p), PRIMES.index(hash // p)
    raise ValueError("hash not factor of primes")

# idea: to avoid constantly checking for pairs of player and sorting the order
# use primes to achieve a unique "hash" for any two pairs of players
# in other words, player i and j would have hash primes[i] * primes[j]

def batched(iterable, n):
    if n < 1:
        raise ValueError('n must be at least one')
    iterator = iter(iterable)
    t = []
    for i in iterator:
        t.append(i)
        if len(t) == n:
            yield t
            t = []
    if t:
        yield t

class RotationChart:
    def __init__(self, players=12):
        self.players = players;
        self.partners = defaultdict(lambda: 0); 
        self.opponents = defaultdict(lambda: 0);
        self.rounds = np.ndarray(shape=(players, players-1))
        
        # build rounds
        for _ in range(players-1):
            round = list(range(players))
            random.shuffle(round)
            self.rounds.append(round)
            for a, b, c, d in batched(round, n=4):
                for partner in [[a, b], [c, d]]:
                    h = hash_pair(*partner)
                    self.partners[h] += 1
                for opponents in [[a, c], [a, d], [b, c], [b, d]]:
                    h = hash_pair(*opponents)
                    self.opponents[h] += 1

    def print_partners(self):
        for k, v in sorted(self.partners.items()):
            a, b = unhash_pair(k)
            print(f"({a:>2}, {b:>2}): {v:>2}")

    def __str__(self):
        s = ""
        for i, round in enumerate(self.rounds):
            s += f"{i:>2} "
            for i in range(0, len(round), 4):
                table = round[i:i+4]
                if len(table) == 0:
                    continue
                a, b, c, d = tuple(table)
                #if a > b:
                #    a, b = b, a
                #if c > d:
                #    c, d = d, c
                s += f"[{a:>2} {b:>2}:{c:>2} {d:>2}]"
            s += "\n"
        return s