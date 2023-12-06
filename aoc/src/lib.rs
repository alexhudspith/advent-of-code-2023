use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::{fs, io};
use std::num::ParseIntError;
use std::path::PathBuf;
use std::str::{FromStr, Utf8Error};

use itertools::Itertools;

#[derive(Debug)]
pub struct ParseDataError {
    pub reason: String
}

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    ParseDataError(ParseDataError),
    ParseIntError(ParseIntError),
    Utf8Error(Utf8Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

impl std::error::Error for Error { }

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl From<ParseDataError> for Error {
    fn from(value: ParseDataError) -> Self {
        Self::ParseDataError(value)
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::from(ParseDataError { reason: value })
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::from(value.to_owned())
    }
}

impl From<Utf8Error> for Error {
    fn from(value: Utf8Error) -> Self {
        Self::Utf8Error(value)
    }
}

pub fn aoc_err<E>(value: E) -> Error where Error: From<E> {
    Error::from(value)
}

fn find_day_dir(day_dirname: &str) -> PathBuf {
    let cwd = std::env::current_dir().expect("Can't get current directory");
    if cwd.ends_with(day_dirname) {
        return cwd;
    }

    for d in fs::read_dir(cwd).expect("Can't read current directory") {
        let Ok(d) = d else { continue };
        if d.file_name() == day_dirname {
            return d.path();
        }
    }

    panic!("Can't find day directory {day_dirname}");
}

pub fn find_path(day_dirname: &str, filename: &str) -> PathBuf {
    let mut day_dir = find_day_dir(day_dirname);
    day_dir.push("data");
    day_dir.push(filename);
    day_dir
}

pub fn find_input_path(day_dirname: &str) -> PathBuf {
    find_path(day_dirname, "input.txt")
}

pub fn parse_spaced_vec<T>(line: &str) -> Result<Vec<T>, T::Err> where T: FromStr {
    parse_spaced(line)
}

pub fn parse_spaced<T, C>(line: &str) -> Result<C, T::Err>
    where T: FromStr, C: FromIterator<T>
{
    line.split_ascii_whitespace().map(|n| n.parse::<T>()).try_collect()
}

pub fn parse_lines<T, S>(lines: impl Iterator<Item=S>) -> Result<Vec<T>, T::Err>
    where T: FromStr, S: Borrow<String>
{
    lines.map(|line| line.borrow().parse()).try_collect()
}

pub fn expect_next_ok<T, E>(mut lines: impl Iterator<Item=Result<T, E>>, message: &str) -> Result<T, Error>
    where Error: From<E>
{
    let next: Result<T, E> = lines.next().ok_or_else(|| <Error as From<&str>>::from(message))?;
    next.map_err(Error::from)
}

#[inline]
#[must_use]
// Copied from #[unstable(feature = "is_sorted", reason = "new API", issue = "53485")]
pub fn is_sorted<T: PartialOrd>(slice: &[T]) -> bool {
    slice.iter().tuple_windows::<(_, _)>().all(|(a, b)| a.partial_cmp(b).map_or(false, Ordering::is_le))
}
