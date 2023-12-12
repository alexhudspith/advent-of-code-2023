use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek};
use std::iter::repeat;

use itertools::Itertools;
use aoc::TupleSumExt;

use crate::cache::Cache;

mod cache;

#[allow(non_camel_case_types)]
type uint = usize;
type Memo = Cache<(uint, uint, uint), uint>;

fn solve(vents: &[u8], groups: &[uint], group_usage: uint, memo: &mut Memo) -> (uint, usize) {
    if vents.is_empty() {
        // At the end: a solution exists only if all groups are exhausted
        let solutions = if groups.is_empty() { 1 } else { 0 };
        return (solutions, 1)
    }

    // Memoization: vents & groups are only sliced so this len-based key is unique
    let key = (vents.len() as uint, groups.len() as uint, group_usage);
    if let Some((&value, calc_count)) = memo.get(&key) {
        return (value, calc_count);
    }

    let alternatives = match vents[0] {
        b'?' => &[b'.', b'#'],
        _ => &vents[0..1]
    };

    let group_free = !groups.is_empty() && group_usage < groups[0];
    let (solutions, calc_count) = alternatives.iter().map(|&vent| {
        match (vent, group_free, group_usage) {
            (b'#', true, _) => {
                // Current group has space. Consume from it.
                solve(&vents[1..], groups, group_usage + 1, memo)
            },
            (b'.', _, 0) => {
                // No groups left; or current group is not yet used. Stay on it.
                solve(&vents[1..], groups, 0, memo)
            },
            (b'.', false, _) => {
                // Current group exists but is exhausted. Move to next group.
                solve(&vents[1..], &groups[1..], 0, memo)
            }
            (_, _, _) => (0, 0),
        }
    })
    .tuple_sum();

    let calc_count = calc_count + 1;
    memo.insert(key, solutions, calc_count);
    (solutions, calc_count)
}

fn run<R: Read>(input: R, repeats: usize) -> Result<uint, aoc::Error> {
    let mut total = 0;
    for line in BufReader::new(input).lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        let (vents, groups) = parse_line(&line)?;
        // Pad the end with '.' so the last group of #'s is not special
        let unfolded_vents = repeat(vents).take(repeats).join("?") + ".";
        let unfolded_groups = repeat(groups).take(repeats).flatten().collect_vec();

        let mut memo = Memo::new();
        let (solutions, _) = solve(unfolded_vents.as_bytes(), &unfolded_groups, 0, &mut memo);
        // eprintln!("{:?}", memo.stats());
        total += solutions;
    }

    Ok(total)
}

fn parse_line(line: &str) -> Result<(&str, Vec<uint>), aoc::Error> {
    let Some((vents, groups)) = line.split_ascii_whitespace().collect_tuple::<(_, _)>() else {
        return Err(format!("Bad line: {line}").into());
    };

    let groups: Vec<_> = groups.split(',').map(str::parse).try_collect()?;
    Ok((vents, groups))
}

// Answer: 7251
fn part1<R: Read>(input: R) -> Result<uint, aoc::Error> {
    run(input, 1)
}

// Answer: 2128386729962
fn part2<R: Read>(input: R) -> Result<uint, aoc::Error> {
    run(input, 5)
}

fn main() -> Result<(), aoc::Error> {
    let path = aoc::find_input_path("day-12");
    let mut f = File::open(path)?;

    let answer = part1(&f)?;
    println!("Part 1: {answer}");
    f.rewind()?;
    let answer = part2(&f)?;
    println!("Part 2: {answer}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
    "};

    #[test]
    fn part1_example() {
        let r = Cursor::new(EXAMPLE);
        let answer = part1(r).unwrap();
        assert_eq!(answer, 21);
    }

    #[test]
    fn part2_example() {
        let r = Cursor::new(EXAMPLE);
        let answer = part2(r).unwrap();
        assert_eq!(answer, 525152);
    }
}
