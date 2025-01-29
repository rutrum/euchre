#show table: set align(center)

= Notes

If I had to write out a paper, how would I format this?

+ Introduction
+ Problem Definition
  + Valid chart
  + Optimal chart
  + Subproblems
  + Problem Extensions
+ Data Structures
  + Represenations that help, perhaps
  + But these might be algorithm/analytic specific
+ Trivial Charts
+ Algorithms for Finding Charts
+ Analytic methods for Charts
  + 4n <=> 4n-1 chart

Or I could sort it by the restrictions on the problem.

+ $4n$ players, $<4n$ rounds (no byes, partial chart)
+ $4n$ players, $4n$ rounds (no byes)
+ $n$ players, $n$-ish rounds

Then investigate methods for each. I like this, since this is how I would practically solve the problem.

= $4n$ Players, $<4n$ Rounds

A single round of $4n$ players is a trivial matter. Two rounds is not trivial, but also not difficult. Before solving across $4n$ rounds, we start by examining methods that work for less than $4n$ rounds, a subset of the optimal rotation chart that has less constraints.

As we add additional rows, we expect the number of valid configurations to grow as well as the optimal configurations to grow. I suspect this ratio decreases rapidly as we approach $4n$ players.

*todo*: count the number of valid and optimal configurations for each number of rounds.

Suppose we had $4n=12$ players.

#table(
  columns: (auto, auto, auto, auto),
  [*Rounds*], [*Valid Charts*], [*Optimal Charts*], [*Proportion*],
  [1], [$12!$], [$12!$], [$1$],
  [2], [$(12!)^2$], [$x$], [],
  [$dots.v$], [], [], [],
  [n], [$(12!)^n$], [], [],
)

What is $x$? Recall that we need to place the players in the second row such that each player has a new partner. In this case, we will never hit the 2 opponent maximum. So we can imagine the number of pairs that can be formed so everyone gets a new partner, and their opponent pairing won't matter.

Can I even analytically count this!?

Let's try starting with all possible pairings of people. That's $12 times 11$ pairings. We know that $6$ of those pairings are already present in round one. So we have $12 times 11 - 6$ pairings. Of these pairings, we have to select 6 more such that each player is only present once.

Another idea. Start will all possible _rows_ of pairs. Then remove rows that have pairs present. How many rows are there of all possible pairings? Well, for players $(1,2)$, there would $10 times 9$ pairings to build rows from. There's a recursive relationship here.

How many possible pairing are there of 4 people? It's 3 (and it's written down below). If we have 6 people, then *it could be* the number of ways to pair 2 people, followed by the number of ways to pair the next 4. But this would ultimately lead to duplicates. Maybe that's okay though? In the example above, as long as we count the duplicates twice, it should be okay. Let's call the total number of possible rows from pairs of 4 people $"PR"(4)$

So $"PR"(6) = (6 times 5) times "PR(4)"$. Then

$ "PR"(n) = (n times (n-1)) times "PR(n-2)" $

Now we need to unwrap this...which isn't hard?

$
  "PR"(n)
  &= underbrace((n times (n-1)) times ((n-2) times (n-3)) times dots times (6 times 5), n / 2 - 2) times "PR"(4) \
  &= (n! / (n-2)!) times ((n-2)! / (n-4)!) times dots times (6! / 4!) times "PR"(4) \
  &= n! / 4! times 3 = n! / 6
$

What's the reasoning here? We have $n!$ rounds of people in a row, and 5/6th of those rows...what? What does PR mean again?

Why would $"PR"(6)$ have duplicates? Right, because after doing $(1,2), "PR"(4)$, we would have $(1,3), "PR"(4)$ which would overlap. I guess it would overlap 5 times, hence the division by 6... But would that not happen on every increase? I would expect $n! / 6^(n/2)$ then.

Okay new idea for this. Line up all n! permutations in a column. Then go through each pair present in row 1 and count the number of permutations removed. So for the first pair, I would expect how many permutations to contain that pair? This is not so trivial, since the pair needs to be present in column $n mod 2$ and $n+1 mod 2$.


= Problem Definition

I need to run a euchre tournament. I need a chart that tells which player is partners with who, which pairs of partners will play against eachother, and what table they need to sit at, across many rounds. We refer to this as a _rotation chart_.

Ideally, the rotation chart will be in the following constraints.

- Everyone is partnered with every other person exactly once
- Everyone plays against every other person exactly 2 times (2 opponents)
- Each person is assigned 1 seat every round, including dedicated bye seats who are not in play

== Criteria

A rotation chart is defined by the number of _players_, and the number of rounds is determined based on the number of players and the constraints that need to be met.

What is a _valid_ rotation chart?

- A rotation chart where each player is present exactly once in each round.

What is an _optimal_ rotation chart?

- A valid rotation chart that also meets the criteria described above.

What is a round, seat, player, partner, opponent, and table?

=== Alternate Criteria

Does the idea of a valid or optimal rotation chart actually help with computation?

I think I've shown that by limiting the number of _rows_ to be explicitly less than the number of people, there is a much easier problem that can be found by naive searching. I can also fix the number of players to multiples of 4. This does still fit the idea of "valid". But it opens the door for _many_ optimal charts, but the criteria is different. Instead of being _exactly_ 2 times, I could say that it's _no more than_ 2 times.

= Trivial Charts

For less than 4 players, all players are in bye rounds. There is no gameplay, there is no tournament.

We can develop an optimal rotation chart for 4 people easily.

#table(
  columns: 2,
  align: (col, row) => (auto, auto).at(col),
  inset: 6pt,
  [Round], [Pairings],
  [1], [{1,2} {3,4}],
  [2], [{1,3} {2,4}],
  [3], [{1,4} {2,3}],
)

= Methods for Producing Optimal Rotation Charts

1. We generate valid _pairings_ of players across rounds. Then we join them into valid tables. A valid list of pairing may not necessarily mean that a valid rotation chart could be constructed from it.

== Generating Pairings for Powers of 2
<generating-pairings-for-powers-of-2>
I think a inductive argument makes sense here. Here’s an intuitive argument for parties of size $2^n$.

Suppose we have a valid rotation table for a party of size $n$. We will construct a valid rotation table $R_0$ for a party of size $2 n$. For players 1 through $n$, we use the $n$-player rotation table called $R_1$. We also use the $n$-player rotation table for players $n + 1$ to $2 n$ called $R_2$. We construct new rounds 1 through $n - 1$ such that round $i$ of $R_0$ of the union of round $i$ in $R_1$ and $R_2$. We now construct $n$ new rounds. Let round $k + n - 1$ be constructed by pairing player $i$ of $R_1$ with player $i + k m o d n$ of $R_2$. See that each round pairs each of the $2 n$ individuals with another individual. Also see that each round is distinct and unique. We have generated a valid rotation table for $2 n$.

*what am I smoking here*

Using our $n = 4$ case we can generate the rotation table for $n = 8$.

#table(
  columns: 2,
  align: (col, row) => (auto, auto).at(col),
  inset: 6pt,
  [Round], [Pairings],
  [1], [{1,2} {3,4} {5,6} {7,8}],
  [2], [{1,3} {2,4} {5,7} {6,8}],
  [3], [{1,4} {2,3} {5,8} {6,7}],
  [4], [{1,5} {2,6} {3,7} {4,8}],
  [5], [{1,6} {2,7} {3,8} {4,5}],
  [6], [{1,7} {2,8} {3,5} {4,6}],
  [7], [{1,8} {2,5} {3,6} {4,7}],
)

== Generating Pairings for $4 n + 2$ given $4 n + 1$ Rotation Table
<generating-pairings-for-4n2-given-4n1-rotation-table>
For $4 n + 1$ individuals, a rotation table $4 n$ rounds where each
individual has exactly one bye round exclusively. To construct the

\=\= Some Goals/Open Questions

- Is there a way to iterate through valid charts? \(valid meaning, all players exist exactly once in every round)
- What’s the equivalence between two charts? There’s lots of symmetry. Knowing this during search would help eliminate duplicate entries.
- Related: what’s the proper way to sort a chart? Of all symmetries, there should be exactly one representation that is considered "default" or "first".
- Tackle the 4n case before the others - What rules will always improve the table? Is there an algorithm for always resolving a problem in the table? For instance, if I know players x and y have been partners one too many times, how can I directly solve that problem? I might find one round where the players are partners and switch y with another individual who x hasn’t been partners with enough.
- Well defined cost functions. These are done well in the python code. They should be marked up.

== Symmetry
<symmetry>
Each round shares the same symmetry, so let’s first examine a single 12 person round.

Each round has 3 tables that can be any order. So that’s 3!\=6 permutations. For each of those tables, there are 8 possible configurations: each pair has two orderings, and the two pairs can be in either order. That means 3!#emph[8^3 symmetries. Of the total 12! permutations, only 12!/\(3!];8^3) are unique. In decimal form, thats 479 001 600 permutations, with 3072 symmetries, for a total of 155925 unique rounds.

A valid rotation chart for 12 people consists of 11 rounds, so there is 155925^11 permutations, or 1.324e57.

This value is too large to be traversed through. But only a subset of these permutations actually meet our constraints.

We can also use some baselines \(there’s probably a technical term for this?), for instance, we know at least 1 round will be 1 through 12 no matter what, so we could instead consider the remaining 155925^10 permutations, or 8.495e51.

== Another approach
<another-approach>

Instead of a cost function on valid tables, I could slowly build a set of rounds and perform depth first search until I find a valid solution. I can perform multiple checks along the way to make sure I’m not adding certain pairs/opponents that break the conditions. This may be "faster" than evaluating random tables, since the cost function is doing a lot of repeated work \(as of now). But I do need some other algorithms that are able to check if the rotation chart currently has no valid outcomes before I get there…that might be hard. I could do an approach that uses "both"…randomly add rounds to a list of rounds and check validity after each round is added. If the next round doesn’t work, pick the next one.

The enumeration problem could be isolated down to just valid rounds. How do I enumerate through all 155925 valid permutations?

== Symmetry Aware Representation
<symmetry-aware-representation>

Each pair can be in either order. To avoid maintain that order, I could use 1 number. For instance, a partnered with b could be p\_a\*p\_b where p\_i is the ith prime. This would be a unique value among all a,b combinations but because multiplication is commutative, either permutation gives the same result.

That lowers the memory by 1/2, but MIGHT incur some runtime cost for finding where problems are. This tradeoff is likely worth it.

Can I extend this representation between two pairs of partners? If I do the same trick again, I run into problems: \* p\_a#emph[p\_b\=P is a large number, which means p\_P];p\_P’ is even bigger! Too big \* Multiplication is commutative between partners, which means we lose information about who are partners and who are opponents.

I could simply add both numbers, where one is scaled by a large value. Suppose the max of p\_a#emph[p\_b is M. Then I could join both sets of partners together with P + M];P’. The problem is that this value will check if you switch P and P’ around. Can I get around that?

Is is possible that p\_a#emph[p\_b+1 is another product of two primes? If it was not, then it could allow me to compose \(p\_a];p\_b+1)#emph[\(p\_c];p\_d) which would uniquely factor…but the order still matters doesn’t it? That won’t work either.

I may not be able to extend this. I may need to explicitly use tuples that are sorted before use.

I might ask this, can I quickly check if two pairs are equal in values and not by position? This is one of those probabilistic things where it would be easy to check if they were unequal \(just multiply and compare) but verifying they are equal is harder. But this is a single conditional we are talking about, this isn’t worth thinking about more.

== Bounds for Valid and Optimal Charts

We can use some combinatorial reasoning to determine possible upper bounds for how many possible rotation charts exist, as perhaps rotation charts that are _valid_ and _optimal_.
