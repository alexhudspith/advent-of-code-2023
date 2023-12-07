# Advent of Code 2023

Solutions for [Advent of Code 2023][aoc], written in Rust.

| Day | Name           | Description                          | Strategy                                       | Run time* |
|----:|----------------|--------------------------------------|------------------------------------------------|-----------|
|   1 | Trebuchet?!    | Find first and last number words     | RegEx & HashMap                                | 4 ms      |
|   2 | Cube Conundrum | Red, green, blue cube selection game | [Nom][nom] parse, partial order on RGB triples | 1 ms      |
|   3 | Gear Ratios    | Numbers and symbols in a grid        | Row-major scan with look-around                | 2 ms      |
|   4 | Scratchcards   | Winning numbers win more cards       | Set intersection & 1-pass array scan           | 3 ms      |
|   5 | Fertilizer     | Follow range-to-range mappings       | Range intersection via sort/merge join         | 2 ms      |
|   6 | Wait For It    | Calculate optimal toy button press   | Quadratic roots formula                        | 1 ms      |
|   7 | Camel Cards    | Rank hands by strength then cards    | Custom total order, joker plays as mode card   | 2 ms      |

*Rough wall clock time on my PC, all single-threaded (`--release` mode, directly executed, not via Cargo)

[aoc]: https://adventofcode.com/2023/
[nom]: https://docs.rs/crate/nom/latest
