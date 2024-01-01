use std::io::BufRead;
use std::str::FromStr;

use itertools::Itertools;

use aoc::CollectArray;
use aoc::parse::{OkOrErr, ParseExt};

use crate::parts::{Category, Part};
use crate::workflow::{Op, PartsSystem, Rule, Target, Workflow};

impl FromStr for Part {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut part = Part::default();

        let parts_str = s.strip_prefix('{')
            .and_then(|p| p.strip_suffix('}'))
            .ok_or_err(s)?;

        for cat_val in parts_str.split(',') {
            let [category, value] = cat_val.split('=')
                .try_collect_array()
                .ok_or_err(s)?;
            let category: Category = category.please(s)?;
            let value = value.please(s)?;
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
        let rule_parts = s.split_inclusive(&['<', '>', ':']);

        let Ok([category_op, value_colon, target]) = rule_parts.try_collect_array() else {
            let target = s.please(s)?;
            return Ok(Self::fallback(target))
        };

        let cat_ix = category_op.chars().next().ok_or_err(s)?.len_utf8();
        let (category, op) = category_op.split_at(cat_ix);
        let value = value_colon.strip_suffix(':').ok_or_err(s)?;

        let (category, op, value, target) = (
            category.please(s)?,
            op.please(s)?,
            value.please(s)?,
            target.please(s)?,
        );

        Ok(Self::new(category, op, value, target))
    }
}

impl FromStr for Workflow {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(&['{', '}']);
        let name = parts.next().ok_or_err(s)?.to_owned();
        let rules_str = parts.next().ok_or_err(s)?;
        let rule_str = rules_str.split(',');
        let rules = rule_str.map(|s| s.parse()).try_collect().ok_or_err(s)?;

        Ok(Self::new(name, rules))
    }
}

fn read_many<R, T>(input: R) -> Result<Vec<T>, aoc::error::Error>
    where
        R: BufRead,
        T: FromStr,
        aoc::error::Error: From<T::Err>
{
    let mut values = Vec::new();
    for line in input.lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        values.push(line.parse()?);
    }

    Ok(values)
}

pub fn read_parts<R: BufRead>(input: R) -> Result<Vec<Part>, aoc::error::Error> {
    read_many(input)
}

pub fn read_system<R: BufRead>(input: R) -> Result<PartsSystem, aoc::error::Error> {
    read_many(input).map(PartsSystem::new)
}
