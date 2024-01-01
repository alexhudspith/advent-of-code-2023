use std::io::{BufRead, BufReader, Read, Seek};
use std::str::FromStr;

use aoc::parse::{OkOrErr, ParseExt};

use crate::{CommsModule, CommsModuleType, CommsSystem};

impl FromStr for CommsModuleType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "%" => Ok(Self::FlipFlop),
            "&" => Ok(Self::Conjunction),
            _ => Err(s.to_owned()),
        }
    }
}

impl FromStr for CommsModule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ix = s.chars().next().ok_or_err(s)?.len_utf8();
        let (first, rest) = s.split_at(ix);
        let (name, ty) = if let Ok(t) = first.parse() {
            (rest, t)
        } else if s == "broadcaster" {
            (s, CommsModuleType::Broadcast)
        } else {
            (s, CommsModuleType::Output)
        };

        Ok(CommsModule::new(name, ty))
    }
}

pub fn read_system<R: Read + Seek>(input: R) -> Result<CommsSystem, aoc::error::Error> {
    let input = &mut BufReader::new(input);
    let mut system = CommsSystem::new();

    for line in input.lines() {
        let line = &line?;
        if line.is_empty() {
            continue;
        }

        let mut parts = line.splitn(2, "->");
        let comms: CommsModule = parts.next()
            .ok_or_err(line)?
            .trim()
            .please(line)?;
        system.add(comms);
    }

    input.rewind()?;

    for line in input.lines() {
        let line = &line?;
        if line.is_empty() {
            continue;
        }

        let mut parts = line.splitn(2, "->");
        let comms: CommsModule = parts.next()
            .ok_or_err(line)?
            .trim()
            .please(line)?;
        let connections: Vec<&str> = parts.next()
            .ok_or_err(line)?
            .split(',')
            .map(|s| s.trim())
            .collect();

        let name = comms.name.to_owned();
        for c in connections {
            if !system.index.contains_key(c) {
                system.add(CommsModule::new(c, CommsModuleType::Output));
            }
            system.connect(&name, c);
        }
    }

    Ok(system)
}
