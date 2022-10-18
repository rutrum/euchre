from typing import NewType, List

# A number, perhaps with a name
Player = NewType('Player', int)

# Partnership, a set of two players
class Pair:
    def __init__(self, a: Player, b: Player):
        if a < b:
            self.a = a
            self.b = b
        else:
            self.a = b
            self.b = a

    def has_player(self, p: Player) -> bool:
        return self.a == p or self.b == p

    def __str__(self):
        return f"{self.a}{self.b}"

# A set of two partnerships
class Game:
    def __init__(self, a: Pair, b: Pair):
        if a.a < b.a:
            self.a = a
            self.b = b
        else:
            self.a = b
            self.b = a

    def has_player(self, p: Player) -> bool:
        return self.a.has_player(p) or self.b.has_player(p)

    def __str__(self):
        return f"({self.a}, {self.b})"


# A set of games and byes
class Round:
    def __init__(self, games: List[Game], byes: List[Player] = []):
        self.games = games
        self.byes = list(sorted(byes))

    def game(self, index) -> Game:
        return self.games[index]

    def __add__(self, other: Game) -> Game:
        return Round(self.games + other.games, self.byes + other.byes)

    def __str__(self):
        games_str = " ".join([g.__str__() for g in self.games])
        byes_str = ",".join([b for b in self.byes])
        return f"{games_str}; {byes_str}"

# A list of rounds
class RotationChart:
    def __init__(self, rounds: List[Round]):
        self.rounds = rounds

    def round(self, index) -> Round:
        return self.rounds[index]

    def __len__(self):
        return len(self.rounds)

    def __str__(self):
        s = ""
        for r, round in enumerate(self.rounds):
            s += f"Round {r}: {round}\n"
        return s
