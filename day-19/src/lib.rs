mod parse;
mod parts;
mod workflow;

pub use parse::read_system;
pub use parse::read_parts;
pub use parts::{Part, RangedPart};
pub type Range = aoc::range::Range<u64>;
