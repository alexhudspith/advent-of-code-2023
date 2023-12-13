use std::io::{BufReader, Read};

use itertools::Itertools;

use aoc::grid::{Grid, read_grid_ascii};

const BLANK: u8 = b'.';
const GEAR: u8 = b'*';

pub struct ColSpan {
    pub row: usize,
    pub start: usize,
    pub end: usize,
}

pub fn number_spans<'a, I>(row: usize, iter: I) -> impl Iterator<Item=ColSpan>
    where I: Iterator<Item=(usize, &'a u8)>
{
    iter.batching(move |it| {
        let mut digits = it
            .skip_while(|(_, chr)| !chr.is_ascii_digit())
            .take_while(|(_, chr)| chr.is_ascii_digit())
            .map(|(c, _)| c);

        let start = digits.next()?;
        let end = digits.last().unwrap_or(start) + 1;
        Some(ColSpan { row, start, end })
    })
}

pub fn is_symbol(c: u8) -> bool {
    c != BLANK && !c.is_ascii_digit()
}

pub fn maybe_gear(c: u8) -> bool {
    c == GEAR
}

pub type Schematic = Grid<u8>;

pub fn frame(col_span: ColSpan) -> impl Iterator<Item=(usize, usize)> {
    let row = col_span.row;
    let (top, bottom, left, right) = (row - 1, row + 1, col_span.start - 1, col_span.end);
    let horiz = [top, bottom].into_iter().cartesian_product(left..=right);
    let vert = [(row, left), (row, right)];
    horiz.chain(vert)
}

pub fn find_in_frame<F>(grid: &Schematic, mut predicate: F, col_span: ColSpan) -> Option<(usize, usize)>
    where F: FnMut(u8) -> bool
{
    frame(col_span).find(|&(r, c)| predicate(grid[r][c]))
}

pub fn read_schematic<R: Read>(input: R) -> Result<Schematic, aoc::Error> {
    let mut reader = BufReader::new(input);
    read_grid_ascii(&mut reader, Some(BLANK))
}
