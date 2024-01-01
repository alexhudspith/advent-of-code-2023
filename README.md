# Advent of Code 2023

Solutions for [Advent of Code 2023][aoc], written in Rust.

| Day | Name                     | Description                                         | Strategy                                                 | Run time* |
|----:|--------------------------|-----------------------------------------------------|----------------------------------------------------------|----------:|
|   1 | Trebuchet?!              | Find first and last number words                    | RegEx & HashMap                                          |      4 ms |
|   2 | Cube Conundrum           | Red, green, blue cube selection game                | [Nom] parse, partial order on RGB triples                |      1 ms |
|   3 | Gear Ratios              | Numbers and symbols in a grid                       | Row-major scan with look-around                          |      2 ms |
|   4 | Scratchcards             | Winning numbers win more cards                      | Set intersection & 1-pass array scan                     |      3 ms |
|   5 | Fertilizer               | Follow range-to-range mappings                      | Range intersection via sort/merge join                   |      2 ms |
|   6 | Wait For It              | Calculate optimal toy button press                  | Quadratic roots formula                                  |      1 ms |
|   7 | Camel Cards              | Rank hands by strength then cards                   | Custom total order, joker plays as mode card             |      2 ms |
|   8 | Haunted Wasteland        | Simultaneous graph traversal                        | Cycle finding, least common multiple of path length      |     11 ms |
|   9 | Mirage Maintenance       | Triangular sequence extension                       | Reverse lines for part 2                                 |      2 ms |
|  10 | Pipe Maze                | Traverse loop of pipe in a grid                     | Observe neighbouring pipes to find ways available        |      4 ms |
|  11 | Cosmic Expansion         | Expand space between galaxies                       | Sparse 2D grid with HashSet                              |      1 ms |
|  12 | Hot Springs              | Damaged spring pattern matching                     | Recursive DP, fast FxHashMap memoization                 |     31 ms |
|  13 | Point of Incidence       | Find reflections in landscape                       | Simple comparisons with short-circuiting                 |      2 ms |
|  14 | Parabolic Reflector Dish | Roll rocks around a grid                            | Find a cycle in the state and extrapolate                |     60 ms |
|  15 | Lens Library             | Implement a HashMap                                 | As given                                                 |      2 ms |
|  16 | The Floor Will Be Lava   | Shine a beam through grid of mirrors                | Depth-first search                                       |     23 ms |
|  17 | Clumsy Crucible          | Minimize loss along a path with restricted movement | Dijkstra's shortest path with modifications              |    350 ms |
|  18 | Lavaduct Lagoon          | Calculate lagoon area from movement instructions    | Shoelace formula for polygon area, plus perimeter        |      1 ms |
|  19 | Aplenty                  | Filter parts through interconnected workflows       | Model workflows with range intersection                  |      3 ms |
|  20 | Pulse Propagation        | Calculate pulses sent after pushing a button        | Model system and find cycles in the state                |     20 ms |
|  21 | Step Counter             | Count walks in an infinitely-repeating garden       | Fit a quadratic equation to specially-crafted input      |  1,662 ms |
|  22 | Sand Slabs               | Determine falling bricks in a stack                 | Process bricks bottom-up tracking a 2D height map        |     40 ms |
|  23 | A Long Walk              | Find longest walk in a grid (part 2 with cycles)    | Reduce to junction graph, exhaustive DFS with [PetGraph] |    810 ms |
|  24 | Never Tell Me The Odds   | Calculate intersection of hailstone paths           | Solve system of quadratic equations with [Z3]            |     95 ms |
|  25 | Snowverload              | Cut 3 wires to divide a system in two               | Parallel randomized Karger's algorithm                   |   ~300 ms |

*Rough wall clock time on my PC, single-threaded except for Day 25
(`--release` mode, directly executed, not via Cargo)

[aoc]: https://adventofcode.com/2023/

[Nom]: https://docs.rs/crate/nom/latest

[PetGraph]: https://docs.rs/crate/petgraph/latest

[Z3]: https://en.wikipedia.org/wiki/Z3_Theorem_Prover
