= Introduction
<introduction>
Euchre is a playing-card game involving 4 individuals in teams of 2.
Each round involves a dealer initiating the round and the players play
through a series of tricks. After 5 tricks points are resolved and the
next player is dealer.

When more than 4 people play Euchre, you could make a tournament of
multiple games to resolve points across many people. The list of rounds,
which individuals are partnered together, and which pairs play against
other pairs for each game, is called a rotation chart.

= Definitions
<definitions>
A #emph[player] denoted $p_i$ represents an individual involved in the
tournament. We will denote the first player $p_0$ and the $n^(t h)$
player $p_(n - 1)$.

All games played across players in the tournament at a given time is
called a #emph[round];. The list of rounds needed to play the entire
tournament is called the #emph[rotation table];, or the description of
each player’s partner and opponents across each round.

A pair of individuals playing together in a Euchre is called a
partnership. It is the set containing both players. Two players
$p_i , p_j$ in a partnership ${ p_i , p_j }$ will notated as $t_(i j)$.
We say that $p_i$ is #emph[partners with] $p_j$.

A game of Euchre is played between two partnerships. We can define the
game as the set of two partnerships ${ t_(i j) , t_(m n) }$. We device
an alternative simple notation for this: $(i j , m n)$ or
$(i . j , m . n)$ if ambiguous.

A round is pair of games and byes of distinct players. #emph[Byes] is a
set of players not part of a game in that round. We say a round
$R = (G , B)$.

A tournament of $n$ players is determined by a #emph[rotation chart]
$T_n = { R_i }$ which is a set of rounds $R_i$.

A set of games, or a set of sets of parterships, can be cumbersome to
notate. We devise alternative notation.

== Rotation Chart Criteria
<rotation-chart-criteria>
We call a rotation chart $T$ #emph[optimal] if it meets the following
criteria.

#block[
#set enum(numbering: "1)", start: 1)
+ In the union of all partnerships of all games of all rounds of $T$
  each player $p_i$ is partners with each other player exactly once.
+ In the list construct from all byes across all rounds has each player
  the same number of times.
+ Rounds contain $⌊ n / 4 ⌋$ games.
+ Each player $p_i$ plays $2$ games as the opponent of each other
  player.
]

== Trivial Example
<trivial-example>
Suppose we have a rotation chart $T_4$ as follows.

#figure(
align(center)[#table(
  columns: 2,
  align: (col, row) => (right,auto,).at(col),
  inset: 6pt,
  [Round], [Games],
  [$R_0$],
  [\(12, 34)],
  [$R_1$],
  [\(13, 24)],
  [$R_2$],
  [\(14, 23)],
)]
)

We show that this is optimal.

#block[
#set enum(numbering: "1)", start: 1)
+ Each player $p_i$ is partners with every other $p_j eq.not p_i$.
+ Each player has the same number of byes $0$.
+ Rounds contain $⌊ 4 / 4 ⌋ = 1$ games.
+ Each player plays $2$ games as the opponent of each other player.
]

= Other Optimal Rotation Chart
<other-optimal-rotation-chart>
We can use a constructive argument to get us to an optimal rotation
chart. Let’s start with an induction argument that shows that given an
optimal rotation chart $T_(2^n)$ we can construct an optimal rotation
chart of $T_(2^(n + 1))$.

== Hmm?
<hmm>
#figure(
align(center)[#table(
  columns: 2,
  align: (col, row) => (auto,auto,).at(col),
  inset: 6pt,
  [Round], [Partnerships],
  [$R_0$],
  [12, 34, 56, 78],
  [$R_1$],
  [13, 24, 57, 68],
  [$R_2$],
  [14, 23, 58, 67],
  [$R_3$],
  [15, 26, 37, 48],
  [$R_4$],
  [16, 27, 38, 45],
  [$R_5$],
  [17, 28, 35, 46],
  [$R_6$],
  [18, 25, 36, 47],
)]
)

#figure(
align(center)[#table(
  columns: 2,
  align: (col, row) => (auto,auto,).at(col),
  inset: 6pt,
  [Round], [Games],
  [$R_0$],
  [\(12, 34) \(56, 78)],
  [$R_1$],
  [\(13, 57) \(24, 68)],
  [$R_2$],
  [\(14, 67) \(23, 58)],
  [$R_3$],
  [15, 26, 37, 48],
  [$R_4$],
  [16, 27, 38, 45],
  [$R_5$],
  [17, 28, 35, 46],
  [$R_6$],
  [18, 25, 36, 47],
)]
)

== How many Rounds?
<how-many-rounds>
If $T_n$ is optimal, then there is at least $n - 1$ games, since each
player must partner with each other $n - 1$ players. Because each round
must contain $⌊ n / 4 ⌋$ games, there will be $n med mod med 4 = r$ byes
for a round of $n$ players. If $T_n$ is optimal, each player has the
same number of byes, which means a player has $n - 1$ rounds playing
with a partner, and $r$ rounds as a bye. This means there must be
precisely $n - 1 + r$ rounds, since a player is either a bye or a
partner.

$T_n$ has $n - 1 + (n med mod med 4)$ rounds.

== Observations
<observations>
The rotation chart $T_4$ is unique. So is $T_5$?

== Rotation Chart $T_4$
<rotation-chart-t_4>
#figure(
align(center)[#table(
  columns: 2,
  align: (col, row) => (right,auto,).at(col),
  inset: 6pt,
  [Round], [Games],
  [$R_0$],
  [\(12, 34)],
  [$R_1$],
  [\(13, 24)],
  [$R_2$],
  [\(14, 23)],
)]
)

== Rotation Chart $T_5$
<rotation-chart-t_5>
#figure(
align(center)[#table(
  columns: 3,
  align: (col, row) => (right,auto,auto,).at(col),
  inset: 6pt,
  [Round], [Games], [Byes],
  [$R_0$],
  [\(12, 35)],
  [4],
  [$R_1$],
  [\(13, 45)],
  [2],
  [$R_2$],
  [\(14, 23)],
  [5],
  [$R_3$],
  [\(15, 24)],
  [3],
  [$R_4$],
  [\(25, 34)],
  [1],
)]
)

Pretty sure this is unique \(up to transpositions). Is it clear how I
would construct this from $T_4$?
