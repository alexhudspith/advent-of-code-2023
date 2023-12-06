# Advent of Code 2023

Solutions for [Advent of Code 2023][aoc], written in Rust.

| Day | Name          | Description                          | Strategy                                       | Run time* |
|----:|---------------|--------------------------------------|------------------------------------------------|-----------|
|   1 | Number Words  | Find first and last number words     | RegEx & HashMap                                | 4 ms      |
|   2 | Cubes         | Red, green, blue cube selection game | [Nom][nom] parse, partial order on RGB triples | 1 ms      |
|   3 | Gears         | Numbers and symbols in a grid        | Row-major scan with look-around                | 2 ms      |
|   4 | Lottery Cards | Winning numbers win more cards       | Set intersection & 1-pass array scan           | 3 ms      |
|   5 | Seed Maps     | Follow range-to-range mappings       | Range intersection via sort/merge join         | 2 ms      |
|   6 | Boat Race     | Calculate optimal button press       | Quadratic roots formula                        | 1 ms      |

*Rough wall clock time on my PC, all single-threaded (`--release` mode, directly executed, not via Cargo)

[aoc]: https://adventofcode.com/2023/
[nom]: https://docs.rs/crate/nom/latest
