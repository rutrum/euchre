import numpy as np

class Matches:

    def __init__(self, n):
        self.num_players = n
        self.num_rounds = n - 1
        self.A = np.zeros((self.num_rounds, self.num_players), dtype=np.int32)

    def partner(self, round, player):
        return self.A[round, player]

    def already_partners(self, player, partner):
        for round in range(self.num_rounds):
            if self.partner(round, player) == partner + 1:
                return True
        return False

    def assign_partners(self, round, player, partner):
        self.A[round, player] = partner + 1
        self.A[round, partner] = player + 1

    def assigned(self, round, player):
        return self.A[round, player] > 0

    def assign(self):
        for player in range(self.num_players):
            for round in range(self.num_rounds):
                if not self.assigned(round, player):
                    for partner in range(self.num_players):
                        if player == partner:
                            continue
                        if self.already_partners(player, partner):
                            continue
                        if self.assigned(round, partner):
                            continue
                        self.assign_partners(round, player, partner)
                        self.print()
                        input()
                        break



    def print(self):
        print(f"{'Player:':>9}", end=" ")
        for player in range(self.num_players):
            print(f"{player+1:>3}", end=" ")
        print()
        for round in range(self.num_rounds):
            print(f"{f'Round {round + 1}:':>9}", end=" ")
            for player in range(self.num_players):
                partner = self.partner(round, player)
                print(f"{partner:>3}", end=" ")
            print()

def main():

    n = 12 # number of players

    m = Matches(n)
    m.assign()
    m.print()

if __name__ == "__main__":
    main()
