# Advent of Code 2023

Solutions for [Advent of Code 2023][aoc], written in Rust.

| Day | Name          | Description                          | Strategy                               | Run time* |
|----:|---------------|--------------------------------------|----------------------------------------|-----------|
|   1 | Number Words  | Find first and last number words     | RegEx & HashMap                        | -         |
|   2 | Cubes         | Red, green, blue cube selection game | Partial order on RGB triples           | -         |
|   3 | Gears         | Numbers and symbols in a grid        | Row-major scan with look-around        | -         |
|   4 | Lottery Cards | Winning numbers win more cards       | Set intersection & 1-pass array scan   | -         |
|   5 | Seed Maps     | Follow range-to-range mappings       | Range intersection via sort/merge join | -         |
|   6 | Boat Race     | Calculate optimal button press       | Quadratic roots formula                | -         |

*Rough wall clock time on my PC, all single-threaded

[aoc]: https://adventofcode.com/2023/
