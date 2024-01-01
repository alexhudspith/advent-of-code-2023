use std::collections::BTreeMap;
use std::fmt::Debug;
use std::io::{BufRead, BufReader, Read};

use itertools::Itertools;

use aoc::grid::Grid;

use crate::world::{BoundingBox, Vec3, X, Y, Z};

mod world;

type Brick = BoundingBox;

#[derive(Debug, Clone)]
pub struct Bricks {
    bricks_base: BTreeMap<u64, Vec<Brick>>,
    bricks_top: BTreeMap<u64, Vec<Brick>>,
    bounding_box: BoundingBox,
}

impl Bricks {
    pub fn new() -> Self {
        Self::new_with(&[])
    }

    pub fn new_with(bricks: &[Brick]) -> Self {
        let bricks_base = group_by_z(bricks, false);
        let bricks_top = group_by_z(bricks, true);
        let bounding_box = BoundingBox::from(bricks);
        Bricks { bricks_base, bricks_top, bounding_box }
    }

    pub fn is_empty(&self) -> bool {
        self.bounding_box.is_empty()
    }

    fn base_top(brick: &Brick) -> (u64, u64) {
        (brick[0][Z], brick[1][Z])
    }

    pub fn settle(&mut self) {
        let mut result = Self::new();

        for (mut brick, fall) in self.falling(None).map(|(b, f)| (*b, f)) {
            brick -= Vec3::new(0, 0, fall);
            result.add(brick);
        }

        *self = result;
    }

    pub fn falling<'b>(&'b self, without: Option<&'b Brick>) -> impl Iterator<Item=(&Brick, u64)> {
        let bb = self.bounding_box;
        let mut height_map = Grid::new((
            (bb[1][X] - bb[0][X]) as usize,
            (bb[1][Y] - bb[0][Y]) as usize
        ));

        let offset = move |(x, y): (u64, u64)| (
            (x - bb[0][X]) as usize,
            (y - bb[0][Y]) as usize
        );

        self.iter()
            .filter(move |&brick| Some(brick) != without)
            .map(move |brick| {
                let ext = brick.extents();
                let zmax = ext[X].into_iter().cartesian_product(ext[Y])
                    .map(|xy| height_map[offset(xy)])
                    .max()
                    .unwrap();

                let fall = ext[Z].start() - zmax;
                for xy in ext[X].into_iter().cartesian_product(ext[Y]) {
                    height_map[offset(xy)] = brick[1][Z] - fall;
                }

                (brick, fall)
            })
    }

    fn overlapping_xy<'b>(&self, map: &'b BTreeMap<u64, Vec<Brick>>, z: u64, brick: &Brick) -> Vec<&'b Brick> {
        map.get(&z)
            .map(|bricks| bricks.iter()
                .filter(|&b| brick.overlaps(b, X) && brick.overlaps(b, Y))
                .collect_vec()
            )
            .unwrap_or_default()
    }

    pub fn supporting(&self, brick: &Brick) -> Vec<&Brick> {
        self.overlapping_xy(&self.bricks_top, Self::base_top(brick).0, brick)
    }

    pub fn supported_by(&self, brick: &Brick) -> Vec<&Brick> {
        self.overlapping_xy(&self.bricks_base, Self::base_top(brick).1, brick)
    }

    pub fn falls_without(&self, support: &Brick) -> Vec<&Brick> {
        let supported = self.supported_by(support);
        let mut result = Vec::new();
        for may_fall in supported {
            let supporting = self.supporting(may_fall);
            assert!(supporting.contains(&support));
            if supporting.len() == 1 {
                result.push(may_fall);
            }
        }

        result
    }

    pub fn add(&mut self, brick: Brick) {
        self.bounding_box.expand(&brick);

        let base_top = Self::base_top(&brick);
        self.bricks_base.entry(base_top.0)
            .or_default()
            .push(brick);

        self.bricks_top.entry(base_top.1)
            .or_default()
            .push(brick);
    }

    pub fn iter(&self) -> impl Iterator<Item=&Brick> {
        self.bricks_base.values().flatten()
    }
}

impl Default for Bricks {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for Bricks {
    type Item = Brick;
    type IntoIter = std::vec::IntoIter<Brick>;

    fn into_iter(self) -> Self::IntoIter {
        self.bricks_base.values()
            .flatten()
            .copied()
            .collect_vec()
            .into_iter()
    }
}

fn group_by_z(bricks: &[Brick], top: bool) -> BTreeMap<u64, Vec<Brick>> {
    let i = if top { 1 } else { 0 };
    let key = |b: &Brick| b[i][Z];

    let mut bricks = bricks.to_owned();
    bricks.sort_by_key(key);

    let group = bricks.into_iter().group_by(key);
    group.into_iter()
        .map(|(k, v)| (k, v.collect()))
        .collect()
}

pub fn read_bricks<R: Read>(input: R) -> Result<Bricks, aoc::error::Error> {
    let lines = BufReader::new(input).lines();
    let mut bricks = Bricks::new();
    for line in lines {
        let line = line?;
        let brick: Brick = line.parse()?;
        bricks.add(brick);
    }

    Ok(bricks)
}
