use std::io::{BufRead, BufReader, Read};

use itertools::Itertools;
use aoc::{aoc_err, some_ok_or, parse_spaced_vec};

use crate::{SeedMapEntry, SeedMap};

fn parse_seed_entry(line: &str) -> Result<SeedMapEntry, aoc::Error> {
    if let &[dest, src, len] = parse_spaced_vec::<u64>(line)?.as_slice() {
        Ok(SeedMapEntry::new(src, dest, len))
    } else {
        Err(aoc_err("Incorrect format: expected 3 integers"))
    }
}

pub fn parse_seed_map(mut lines: impl Iterator<Item=String>) -> Result<SeedMap, aoc::Error> {
    let line = lines.next().ok_or_else(|| aoc_err(""))?;
    let name = line.strip_suffix(" map:").ok_or_else(|| aoc_err(""))?.to_string();

    let entries: Vec<_> = lines
        .take_while(|line| !line.is_empty())
        .map(|line| parse_seed_entry(&line))
        .try_collect()?;

    Ok(SeedMap::new(name, entries))
}

pub fn read_seed_maps<R: Read>(input: R) -> Result<(Vec<u64>, Vec<SeedMap>), aoc::Error> {
    let mut lines = BufReader::new(input).lines();

    let first = some_ok_or(lines.next(), "Empty file")?;
    let seeds = first.strip_prefix("seeds: ").ok_or_else(|| aoc_err("Expected seeds line"))?;
    let seed_numbers = parse_spaced_vec(seeds)?;
    let _blank = some_ok_or(lines.next(), "Expected blank line")?;

    let maps: Vec<_> = lines
        .map(|line_res| line_res.unwrap())
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter(|(empty, _)| !empty)
        .map(|(_, group)| parse_seed_map(group.into_iter()))
        .try_collect()?;

    Ok((seed_numbers, maps))
}
