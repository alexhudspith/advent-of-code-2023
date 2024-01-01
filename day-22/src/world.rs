use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};
use std::str::FromStr;
use aoc::CollectArray;

type Range = aoc::range::Range<u64>;

pub const X: usize = 0;
pub const Y: usize = 1;
pub const Z: usize = 2;

#[derive(Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vec3([u64; 3]);

impl Vec3 {
    pub const ZERO: Self = Self::new(0, 0, 0);
    pub const MAX: Self = Self::new(u64::MAX, u64::MAX, u64::MAX);

    pub const fn new(x: u64, y: u64, z: u64) -> Self {
        Self([x, y, z])
    }

    pub fn min_all(&self, other: &Self) -> Self {
        Self::new(
            self.0[X].min(other.0[X]),
            self.0[Y].min(other.0[Y]),
            self.0[Z].min(other.0[Z])
        )
    }

    pub fn max_all(&self, other: &Self) -> Self {
        Self::new(
            self.0[X].max(other.0[X]),
            self.0[Y].max(other.0[Y]),
            self.0[Z].max(other.0[Z])
        )
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self::new(
            self.0[X] + rhs.0[X],
            self.0[Y] + rhs.0[Y],
            self.0[Z] + rhs.0[Z]
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0[X] += rhs.0[X];
        self.0[Y] += rhs.0[Y];
        self.0[Z] += rhs.0[Z];
    }
}

impl Add<u64> for Vec3 {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        Self::new(self.0[X] + rhs, self.0[Y] + rhs, self.0[Z] + rhs)
    }
}

impl AddAssign<u64> for Vec3 {
    fn add_assign(&mut self, rhs: u64) {
        self.0[X] += rhs;
        self.0[Y] += rhs;
        self.0[Z] += rhs;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.0[X] - rhs.0[X], self.0[Y] - rhs.0[Y], self.0[Z] - rhs.0[Z])
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0[X] -= rhs.0[X];
        self.0[Y] -= rhs.0[Y];
        self.0[Z] -= rhs.0[Z];
    }
}

impl Sub<u64> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: u64) -> Self::Output {
        Self::new(self.0[X] - rhs, self.0[Y] - rhs, self.0[Z] - rhs)
    }
}

impl SubAssign<u64> for Vec3 {
    fn sub_assign(&mut self, rhs: u64) {
        self.0[X] -= rhs;
        self.0[Y] -= rhs;
        self.0[Z] -= rhs;
    }
}

impl Mul<u64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        Self::new(self.0[X] * rhs, self.0[Y] * rhs, self.0[Z] * rhs)
    }
}

impl MulAssign<u64> for Vec3 {
    fn mul_assign(&mut self, rhs: u64) {
        self.0[X] *= rhs;
        self.0[Y] *= rhs;
        self.0[Z] *= rhs;
    }
}

impl Index<usize> for Vec3 {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Debug for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0[X], self.0[Y], self.0[Z])
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoundingBox([Vec3; 2]);

impl BoundingBox {
    pub fn new(bottom_left: Vec3, top_right: Vec3) -> Self {
        Self([bottom_left, top_right])
    }

    pub fn expand(&mut self, other: &Self) {
        if self.is_empty() {
            *self = BoundingBox::new(Vec3::MAX, Vec3::ZERO);
        }

        self[0] = self[0].min_all(&other[0]);
        self[1] = self[1].max_all(&other[1]);
    }

    pub fn extents(&self) -> [Range; 3] {
        [
            Range::new(self[0][X], self[1][X]),
            Range::new(self[0][Y], self[1][Y]),
            Range::new(self[0][Z], self[1][Z]),
        ]
    }

    pub fn overlaps(&self, other: &Self, axis: usize) -> bool {
        let ext1 = self.extents()[axis];
        let ext2 = other.extents()[axis];
        ext1.intersects(&ext2)
    }

    pub fn is_empty(&self) -> bool {
        self[0][X] == self[1][X] ||
            self[0][Y] == self[1][Y] ||
            self[0][Z] == self[1][Z]
    }
}

impl Index<usize> for BoundingBox {
    type Output = Vec3;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for BoundingBox {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Add<Vec3> for BoundingBox {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self::new(self[0] + rhs, self[1] + rhs)
    }
}

impl AddAssign<Vec3> for BoundingBox {
    fn add_assign(&mut self, rhs: Vec3) {
        self[0] += rhs;
        self[1] += rhs;
    }
}

impl Sub<Vec3> for BoundingBox {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::new(self[0] - rhs, self[1] - rhs)
    }
}

impl SubAssign<Vec3> for BoundingBox {
    fn sub_assign(&mut self, rhs: Vec3) {
        self[0] -= rhs;
        self[1] -= rhs;
    }
}

impl FromStr for BoundingBox {
    type Err = aoc::error::Error;

    fn from_str(s: &str) -> Result<Self, aoc::error::Error> {
        let [b0, b1] = s.splitn(2, '~')
            .try_collect_array()?;
        let bottom_left = parse_coord(b0)? - Vec3::new(0, 0, 1);
        let top_right = parse_coord(b1)? + Vec3::new(1, 1, 0);
        Ok(BoundingBox::new(bottom_left, top_right))
    }
}

fn parse_coord(s: &str) -> Result<Vec3, aoc::error::Error> {
    let coord_str: [_; 3] = s.splitn(3, ',')
        .try_collect_array()?;

    Ok(Vec3::new(
        coord_str[X].parse()?,
        coord_str[Y].parse()?,
        coord_str[Z].parse()?,
    ))
}

impl Display for BoundingBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (bb0, bb1) = (self[0], self[1] - 1);
        write!(f, "{},{},{}~{},{},{}",
            bb0[X], bb0[Y], bb0[Z],
            bb1[X], bb1[Y], bb1[Z],
        )
    }
}

impl From<&[BoundingBox]> for BoundingBox {
    fn from(value: &[BoundingBox]) -> Self {
        value.iter().copied().collect()
    }
}

impl FromIterator<BoundingBox> for BoundingBox {
    fn from_iter<T: IntoIterator<Item=BoundingBox>>(iter: T) -> Self {
        iter.into_iter()
            .reduce(|mut a, b| { a.expand(&b); a })
            .unwrap_or_else(|| BoundingBox::new(Vec3::ZERO, Vec3::ZERO))
    }
}
