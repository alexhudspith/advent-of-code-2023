use std::borrow::Borrow;
use std::str::FromStr;

use itertools::Itertools;

use crate::error::Error;

pub fn parse_spaced_vec<T>(line: &str) -> Result<Vec<T>, T::Err> where T: FromStr {
    parse_spaced(line)
}

pub fn parse_spaced<T, C>(line: &str) -> Result<C, T::Err>
    where T: FromStr, C: FromIterator<T>
{
    line.split_ascii_whitespace().map(|n| n.parse::<T>()).try_collect()
}

pub fn parse_lines<T, S>(lines: impl Iterator<Item=S>) -> Result<Vec<T>, T::Err>
    where T: FromStr, S: Borrow<str>
{
    lines.map(|line| line.borrow().parse()).try_collect()
}

pub fn some_ok_or<T, E>(item: Option<Result<T, E>>, message: &str) -> Result<T, Error>
    where Error: From<E>
{
    let result: Result<T, E> = item.ok_or_else(|| <Error as From<&str>>::from(message))?;
    result.map_err(Error::from)
}

pub trait ParseExt<T: ?Sized> where T: FromStr {
    fn please(&self, s: &str) -> Result<T, String>;
}

impl<T: ?Sized> ParseExt<T> for str where T: FromStr {
    fn please(&self, s: &str) -> Result<T, String> {
        self.parse::<T>().map_err(|_| s.to_string())
    }
}

impl<T: ?Sized> ParseExt<T> for &str where T: FromStr {
    fn please(&self, s: &str) -> Result<T, String> {
        self.parse::<T>().map_err(|_| s.to_string())
    }
}

pub trait OkOrErr<T> {
    fn ok_or_err(self, s: &str) -> Result<T, String>;
}

impl<T> OkOrErr<T> for Option<T> {
    fn ok_or_err(self, s: &str) -> Result<T, String> {
        self.ok_or_else(|| s.to_owned())
    }
}

impl<T, E> OkOrErr<T> for Result<T, E> {
    fn ok_or_err(self, s: &str) -> Result<T, String> {
        self.map_err(|_| s.to_owned())
    }
}
