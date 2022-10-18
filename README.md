# Euchre Tournament

I need to run a euchre tournament. I want to create a website that will let me keep track of results throughout the tournament, and make it clear who is partners with who and what table they sit at for which round.

In addition, I need to generate a valid list of matches that meet some criteria:

* Everyone is partnered with every other person once
* Everyone plays against every other person 2 times (2 opponents)
* Bye rounds are accounted for when there is a non-multiple of 4 participants

## Generating Pairings for Powers of 2

I think a inductive argument makes sense here.  Here's an intuitive argument for parties of size $2^n$.

For a party of 4 people we know a valid rotation chart that takes place over 3 rounds.

| Round | Pairings |
| --- | --- |
| 1 | {1,2} {3,4} |
| 2 | {1,3} {2,4} |
| 3 | {1,4} {2,3} |

Suppose we have a valid rotation table for a party of size $n$.  We will construct a valid rotation table $R_0$ for a party of size $2n$.  For players 1 through $n$, we use the $n$-player rotation table called $R_1$.  We also use the $n$-player rotation table for players $n+1$ to $2n$ called $R_2$.  We construct new rounds 1 through $n-1$ such that round $i$ of $R_0$ of the union of round $i$ in $R_1$ and $R_2$.  We now construct $n$ new rounds.  Let round $k+n-1$ be constructed by pairing player $i$ of $R_1$ with player $i+k mod n$ of $R_2$.  See that each round pairs each of the $2n$ individuals with another individual.  Also see that each round is distinct and unique.  We have generated a valid rotation table for $2n$.

Using our $n=4$ case we can generate the rotation table for $n=8$.

| Round | Pairings |
| --- | --- |
| 1 | {1,2} {3,4} {5,6} {7,8} |
| 2 | {1,3} {2,4} {5,7} {6,8} |
| 3 | {1,4} {2,3} {5,8} {6,7} |
| 4 | {1,5} {2,6} {3,7} {4,8} |
| 5 | {1,6} {2,7} {3,8} {4,5} |
| 6 | {1,7} {2,8} {3,5} {4,6} |
| 7 | {1,8} {2,5} {3,6} {4,7} |

## Generating Pairings for $4n+2$ given $4n+1$ Rotation Table

For $4n+1$ individuals, a rotation table $4n$ rounds where each individual has exactly one bye round exclusively.  To construct the 
