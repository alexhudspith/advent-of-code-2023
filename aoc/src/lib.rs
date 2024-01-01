#![allow(clippy::redundant_field_names)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(new_uninit)]
#![feature(step_trait)]

use std::{env, fs};
use std::cmp::Ordering;
use std::convert::Infallible;
use std::mem::MaybeUninit;
use std::ops::{Add, AddAssign};
use std::panic::panic_any;
use std::path::PathBuf;
use itertools::Itertools;

pub mod cycle;
pub mod grid;
pub mod parse;
pub mod range;
pub mod error;

fn find_dir(dirname: &str) -> PathBuf {
    let cwd = env::current_dir().expect("Can't get current directory");
    if cwd.ends_with(dirname) {
        return cwd;
    }

    let dir_list = fs::read_dir(cwd).expect("Can't read current directory");
    // flatten: skip unreadable directories
    for d in dir_list.flatten() {
        if d.file_name() == dirname {
            return d.path();
        }
    }

    panic!("Can't find directory {dirname}");
}

pub fn find_path(filename: &str) -> PathBuf {
    let mut day_dir = find_dir("data");
    day_dir.push(filename);
    day_dir
}

pub fn find_input_path(day_dirname: &str) -> PathBuf {
    find_path(&format!("{day_dirname}.txt"))
}

pub fn infallible<F, T, R>(mut f: F) -> impl FnMut(T) -> Result<R, Infallible>
    where F: FnMut(T) -> R
{
    move |t| Ok::<_, Infallible>(f(t))
}

#[inline]
#[must_use]
// Copied from #[unstable(feature = "is_sorted", reason = "new API", issue = "53485")]
pub fn is_sorted<T: PartialOrd>(slice: &[T]) -> bool {
    slice.iter().tuple_windows::<(_, _)>().all(|(a, b)| a.partial_cmp(b).map_or(false, Ordering::is_le))
}

pub trait CollectArray<const N: usize> where Self: Iterator + Sized {
    fn try_collect_array(self) -> Result<[Self::Item; N], &'static str>;

    fn collect_array(self) -> [Self::Item; N] {
        self.try_collect_array().unwrap_or_else(|e| panic_any(e))
    }
}

impl<const N: usize, I: Iterator> CollectArray<N> for I {
    fn try_collect_array(mut self) -> Result<[Self::Item; N], &'static str> {
        let mut result: [MaybeUninit<Self::Item>; N] = MaybeUninit::uninit_array();
        for r in &mut result {
            let next = self.next().ok_or("Two few items for array")?;
            r.write(next);
        }

        match self.next() {
            // Safety: All elements have been written by the above loop
            None => Ok(unsafe { MaybeUninit::array_assume_init(result) }),
            Some(_) => Err("Too many items for array"),
        }
    }
}

pub trait TupleSum: Sized {
    fn tuple_sum<I: Iterator<Item=Self>>(iter: I) -> Self;
}

impl<X> TupleSum for (X,)
    where
        X: Add<Output=X> + Default,
{
    fn tuple_sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold((X::default(),), |(x1,), (x2,)|
            (x1 + x2,)
        )
    }
}

impl<X, Y> TupleSum for (X, Y)
    where
        X: Add<Output=X> + Default,
        Y: Add<Output=Y> + Default,
{
    fn tuple_sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold((X::default(), Y::default()), |(x1, y1), (x2, y2)|
            (x1 + x2, y1 + y2)
        )
    }
}

impl<X, Y, Z> TupleSum for (X, Y, Z)
    where
        X: Add<Output=X> + Default,
        Y: Add<Output=Y> + Default,
        Z: Add<Output=Z> + Default,
{
    fn tuple_sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold((X::default(), Y::default(), Z::default()), |(x1, y1, z1), (x2, y2, z2)|
            (x1 + x2, y1 + y2, z1 + z2)
        )
    }
}

pub trait TupleSumExt<T: TupleSum>: Iterator<Item=T> {
    fn tuple_sum(self) -> Self::Item
        where
            Self: Sized
    {
        TupleSum::tuple_sum(self)
    }
}

impl<T: TupleSum, I: Iterator<Item=T>> TupleSumExt<T> for I {}


pub trait CumulativeExt<T>: Iterator<Item=T>
    where T: AddAssign + Copy + Default
{
    fn cumulative_sum(&mut self) -> impl Iterator<Item=T> {
        self.scan(T::default(), |acc, v| {
            *acc += v;
            Some(*acc)
        })
    }
}

impl<T, I> CumulativeExt<T> for I
    where
        T: AddAssign + Copy + Default,
        I: Iterator<Item=T>
{
}
