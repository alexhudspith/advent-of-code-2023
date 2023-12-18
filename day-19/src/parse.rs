use std::io::BufRead;
use std::str::FromStr;
use itertools::Itertools;
use aoc::CollectArray;

use crate::workflow::{Op, Rule, PartsSystem, Target, Workflow};
use crate::parts::{Category, Part};

impl FromStr for Part {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut part = Part::default();
        let parts_str = s.strip_prefix('{').and_then(|s| s.strip_suffix('}')).unwrap();
        for cat_val in parts_str.split(',') {
            let [category, value] = cat_val.split('=').try_collect_array()?;
            let category: Category = category.parse()?;
            let value: u64 = value.parse().map_err(|_| s.to_owned())?;
            part[category] = value;
        }

        Ok(part)
    }
}

impl FromStr for Target {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Accept),
            "R" => Ok(Self::Reject),
            name => Ok(Self::Workflow(name.to_owned())),
        }
    }
}

impl FromStr for Category {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Self::X),
            "m" => Ok(Self::M),
            "a" => Ok(Self::A),
            "s" => Ok(Self::S),
            _ => Err(s.to_owned()),
        }
    }
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(Self::LessThan),
            ">" => Ok(Self::GreaterThan),
            "T" => Ok(Self::True),
            _ => Err(s.to_owned())
        }
    }
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse<T: FromStr>(s: &str) -> Result<T, String> {
            s.parse().map_err(|_| s.to_owned())
        }

        let rule_parts = s.split_inclusive(&['<', '>', ':']);

        let Ok([category_op, value_colon, target]) = rule_parts.try_collect_array() else {
            return Ok(Self::fallback(parse(s)?))
        };

        let category = &category_op[0..1];
        let op = &category_op[1..];
        let value = value_colon.strip_suffix(':').unwrap();
        let (category, op, value, target) = (
            parse(category)?,
            parse(op)?,
            parse(value)?,
            parse(target)?,
        );

        Ok(Self::new(category, op, value, target))
    }
}

impl FromStr for Workflow {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(&['{', '}']);
        let name = parts.next().unwrap().to_owned();
        let rules_str = parts.next().unwrap();
        let rule_str = rules_str.split(',');
        let rules = rule_str.map(|s| s.parse()).try_collect()?;

        Ok(Self::new(name, rules))
    }
}

pub fn read_parts<R: BufRead>(input: R) -> Result<Vec<Part>, aoc::Error> {
    let mut parts = Vec::new();
    for line in input.lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let part: Part = line.parse()?;
        parts.push(part);
    }

    Ok(parts)
}

pub fn read_system<R: BufRead>(input: R) -> Result<PartsSystem, aoc::Error> {
    let mut workflows = Vec::new();
    for line in input.lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let workflow: Workflow = line.parse()?;
        workflows.push(workflow);
    }

    let index = workflows.iter()
        .enumerate()
        .map(|(i, w)| (w.name().to_owned(), i))
        .collect();
    Ok(PartsSystem { workflows, index })
}
