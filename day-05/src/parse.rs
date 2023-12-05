use std::error::Error;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::num::ParseIntError;

use itertools::Itertools;

use crate::{SeedMapEntry, SeedMap};

fn io_err<E: Error + Send + Sync + 'static>(e: E) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, e)
}

fn invalid_data() -> io::Error {
    io::Error::from(io::ErrorKind::InvalidData)
}

fn parse_error() -> ParseIntError {
    "".parse::<i32>().unwrap_err()
}

fn numbers(line: &str) -> Vec<u64> {
    line.split_ascii_whitespace().map(|n| n.parse()).try_collect().unwrap()
}

fn parse_seed_entry(line: &str) -> Result<SeedMapEntry, ParseIntError> {
    if let &[dest, src, len] = numbers(line).as_slice() {
        Ok(SeedMapEntry::new(src, dest, len))
    } else {
        Err(parse_error())
    }
}

pub fn parse_seed_map(mut lines: impl Iterator<Item=String>) -> Result<SeedMap, ParseIntError> {
    let line = lines.next().ok_or_else(parse_error)?;
    let name = line.strip_suffix(" map:").ok_or_else(parse_error)?.to_string();

    let entries: Vec<_> = lines
        .take_while(|line| !line.is_empty())
        .map(|line| parse_seed_entry(&line))
        .try_collect()?;

    Ok(SeedMap::new(name, entries))
}


fn line(it: &mut impl Iterator<Item=io::Result<String>>) -> io::Result<String> {
    it.next().transpose()?.ok_or_else(invalid_data)
}

pub fn read_seed_maps<R: Read>(input: R) -> io::Result<(Vec<u64>, Vec<SeedMap>)> {
    let mut lines = BufReader::new(input).lines();

    let first = line(&mut lines)?;
    let seeds = first.strip_prefix("seeds: ").ok_or_else(invalid_data)?;
    let seed_numbers = numbers(seeds);
    let _blank = line(&mut lines)?;

    let maps: Vec<_> = lines
        .map(|line_res| line_res.unwrap())
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter(|(empty, _)| !empty)
        .map(|(_, group)| parse_seed_map(group.into_iter()))
        .try_collect()
        .map_err(io_err)?;

    Ok((seed_numbers, maps))
}
