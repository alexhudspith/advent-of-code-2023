use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufReader, Read, Seek};

use fibonacii_heap::Heap;
use fxhash::FxHashMap;

use aoc::grid::{Way, Ways, read_grid};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    pos: (usize, usize),
    way_in: Way,
    cost: u8,
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    heat_loss: u8,
}

impl TryFrom<u8> for Tile {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, u8> {
        if value.is_ascii_digit() {
            Ok(Self { heat_loss: value - b'0' })
        } else {
            Err(value)
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.heat_loss)
    }
}

type Grid = aoc::grid::Grid<Tile>;

fn neighbours<'g>(grid: &'g Grid, old_state: &State) -> impl Iterator<Item=State> + 'g {
    let State { pos: old_pos, way_in: old_way_in, cost: old_cost } = *old_state;

    Ways::all().iter()
        .filter(move |&new_way| new_way != old_way_in.flipped())
        .flat_map(move |new_way| grid.step(old_pos, new_way).map(|new_pos|
            State {
                pos: new_pos,
                way_in: new_way,
                cost: if new_way == old_way_in { old_cost + 1 } else { 1 },
            }
        ))
}

fn edge(grid: &Grid, _old_state: State, new_state: State) -> u32 {
    grid[new_state.pos].heat_loss as u32
}

fn dijkstra<F, G>(grid: &mut Grid, start: (usize, usize), mut accept: F, mut is_end: G) -> u32
where
    F: FnMut(&State, &State) -> bool,
    G: FnMut(&State) -> bool,
{
    let start1 = State { pos: start, way_in: Way::Down, cost: 1 };
    let start2 = State { pos: start, way_in: Way::Right, cost: 1 };

    let mut seen_cost_by_state: FxHashMap<State, u32> = FxHashMap::default();
    seen_cost_by_state.insert(start1, 0);
    seen_cost_by_state.insert(start2, 0);

    let mut prio_queue: Heap<(u32, State)> = Heap::new();
    prio_queue.push((0, start1));
    prio_queue.push((0, start2));

    while let Some((m, u)) = prio_queue.pop() {
        let n = neighbours(grid, &u);
        for v in n.filter(|v| accept(&u, v)) {
            let new_distance = m + edge(grid, u, v);
            if is_end(&v) {
                return new_distance;
            }

            if new_distance < seen_cost_by_state.get(&v).copied().unwrap_or(u32::MAX) {
                prio_queue.push((new_distance, v));
                seen_cost_by_state.insert(v, new_distance);
            }
        }
    }

    panic!("End not reached")
}

fn part1(grid: &mut Grid) -> usize {
    let start = (0, 0);
    let end = (grid.shape().0 - 1, grid.shape().1 - 1);
    dijkstra(
        grid, start,
        |old, new| old.cost < 3 || new.way_in != old.way_in,
        |new| new.pos == end
    ) as usize
}

fn part2(grid: &mut Grid) -> usize {
    let start = (0, 0);
    let end = (grid.shape().0 - 1, grid.shape().1 - 1);
    dijkstra(
        grid, start,
        |old, new| (old.cost > 3 || new.way_in == old.way_in) && (old.cost < 10 || new.way_in != old.way_in),
        |new| new.pos == end && new.cost > 3
    ) as usize
}

fn run<R: Read, F>(input: R, solve: F) -> Result<usize, aoc::Error>
    where
        F: FnOnce(&mut Grid) -> usize
{
    let mut reader = BufReader::new(input);
    let mut grid = read_grid(&mut reader, None)?;
    Ok(solve(&mut grid))
}

fn main() -> Result<(), aoc::Error> {
    let path = aoc::find_input_path("day-17");
    let mut f = File::open(path)?;

    // Answer: 698
    let answer = run(&f, part1)?;
    println!("Part 1: {answer}");
    f.rewind()?;
    // Answer: 825
    let answer = run(&f, part2)?;
    println!("Part 2: {answer}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use indoc::indoc;

    const EXAMPLE1: &str = indoc! {r"
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533
    "};

    const EXAMPLE2: &str = indoc! {r"
        111111111111
        999999999991
        999999999991
        999999999991
        999999999991
    "};

    #[test]
    fn part1_example() {
        let answer = run(Cursor::new(EXAMPLE1), part1).unwrap();
        assert_eq!(answer, 102);
    }

    #[test]
    fn part2_example1() {
        let answer = run(Cursor::new(EXAMPLE1), part2).unwrap();
        assert_eq!(answer, 94);
    }

    #[test]
    fn part2_example2() {
        let answer = run(Cursor::new(EXAMPLE2), part2).unwrap();
        assert_eq!(answer, 71);
    }
}
