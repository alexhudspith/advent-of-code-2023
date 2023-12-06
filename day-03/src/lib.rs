use std::io::{BufRead, BufReader, Read};
use std::iter::{once, repeat};

use itertools::Itertools;
use aoc::aoc_err;

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

#[derive(Debug, Default, Clone)]
pub struct Schematic {
    rows: Vec<Vec<u8>>,
}

impl Schematic {
    pub fn read<R: Read>(reader: R) -> Result<Self, aoc::Error> {
        // Pad the schematic edges with '.'' rows and columns for easier processing
        let mut rows = vec!["".to_string().into_bytes()];

        for line in BufReader::new(reader).lines() {
            let line = line?;
            if !line.is_empty() {
                let padded_row: Vec<u8> = once(BLANK)
                    .chain(line.bytes())
                    .chain(once(BLANK))
                    .collect();
                rows.push(padded_row);
            }
        }

        let col_count = rows.get(1).ok_or_else(|| aoc_err("Empty schematic"))?.len();
        let padding_row: Vec<u8> = repeat(BLANK).take(col_count).collect();
        rows[0] = padding_row.clone();
        rows.push(padding_row);

        Ok(Self { rows })
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows.len(), self.rows[0].len())
    }

    pub fn iter_rows(&self) -> impl Iterator<Item=&Vec<u8>> {
        self.rows.iter()
    }

    pub fn frame(&self, col_span: ColSpan) -> impl Iterator<Item=(usize, usize)> + '_ {
        let row = col_span.row;
        let (top, bottom, left, right) = (row - 1, row + 1, col_span.start - 1, col_span.end);
        let horiz = [top, bottom].into_iter().cartesian_product(left..=right);
        let vert = [(row, left), (row, right)];
        horiz.chain(vert)
    }

    pub fn find_in_frame<F>(&self, mut predicate: F, col_span: ColSpan) -> Option<(usize, usize)>
        where F: FnMut(u8) -> bool
    {
        self.frame(col_span).find(|&(r, c)| predicate(self.rows[r][c]))
    }
}
