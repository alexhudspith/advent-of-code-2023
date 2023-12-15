use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use itertools::Itertools;

fn hash(value: &str) -> u8 {
    value.bytes().fold(0, |acc, b| {
        ((acc as usize + b as usize) * 17) as u8
    })
}

enum CommandType {
    Insert(usize),
    Delete,
}

struct Command {
    label: String,
    ty: CommandType,
}

impl FromStr for Command {
    type Err = aoc::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(label) = s.strip_suffix('-') {
            return Ok(Command { label: label.to_owned(), ty: CommandType::Delete });
        }

        let &[label, focal_str] = s.splitn(2, '=').collect_vec().as_slice() else {
            return Err(format!("Can't parse command: {s}").into());
        };

        let focal_length: usize = focal_str.parse()?;
        Ok(Command { label: label.to_owned(), ty: CommandType::Insert(focal_length) })
    }
}

fn focus_power(bucket_ix: usize, slot_ix: usize, focal_length: usize) -> usize {
    (bucket_ix + 1) * (slot_ix + 1) * focal_length
}

fn part1(input: &str) -> Result<usize, aoc::Error> {
    let total = input.split(',').map(|s| hash(s) as usize).sum();
    Ok(total)
}

fn part2(input: &str) -> Result<usize, aoc::Error> {
    let mut buckets = vec![vec![]; 256].into_boxed_slice();

    for s in input.split(',') {
        let command: Command = s.parse()?;
        let bucket = &mut buckets[hash(&command.label) as usize];
        let lens_ix_opt = bucket.iter().position(|(label, _)| *label == command.label);

        if let Some(lens_ix) = lens_ix_opt {
            // Found
            match command.ty {
                CommandType::Insert(focal) => { bucket[lens_ix] = (command.label, focal); }
                CommandType::Delete => { bucket.remove(lens_ix); }
            }
        } else if let CommandType::Insert(focal) = command.ty {
            bucket.push((command.label, focal));
        }
    }

    let total = buckets.iter()
        .enumerate()
        .flat_map(|(bucket_ix, bucket)|
            bucket.iter()
                .enumerate()
                .map(move |(lens_ix, (_label, focal))|
                    focus_power(bucket_ix, lens_ix, *focal)
                )
        )
        .sum();

    Ok(total)
}

fn main() -> Result<(), aoc::Error> {
    let path = aoc::find_input_path("day-15");
    let mut f = File::open(path)?;

    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    let buf = buf.replace('\n', "");

    // Answer: 497373
    let answer = part1(&buf)?;
    println!("Part 1: {answer}");
    // Answer: 259356
    let answer = part2(&buf)?;
    println!("Part 2: {answer}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn part1_example() {
        let answer = part1(EXAMPLE).unwrap();
        assert_eq!(answer, 1320);
    }

    #[test]
    fn part2_example() {
        let answer = part2(EXAMPLE).unwrap();
        assert_eq!(answer, 145);
    }
}
