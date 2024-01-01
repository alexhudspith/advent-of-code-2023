use std::fs::File;
use std::io::{BufReader, Read, Seek};
use aoc::range::Range;

use day_19::{read_parts, read_system};
pub use day_19::RangedPart;

fn part1<R: Read>(input: R) -> Result<u64, aoc::error::Error> {
    let mut input = BufReader::new(input);
    let system = read_system(&mut input)?;
    let parts = read_parts(&mut input)?;

    let answer = parts.iter().map(|p| system.value(p)).sum();
    Ok(answer)
}

fn part2<R: Read>(input: R) -> Result<u64, aoc::error::Error> {
    let mut input = BufReader::new(input);
    let system = read_system(&mut input)?;
    let part = RangedPart::all(Range::new(1, 4001));

    let answer = system.combinations(&part);
    Ok(answer)
}

fn main() -> Result<(), aoc::error::Error> {
    let path = aoc::find_input_path("day-19");
    let mut f = File::open(path)?;
    // Answer: 383682
    let answer = part1(&f)?;
    println!("Part 1: {answer}");
    f.rewind()?;
    // Answer: 117954800808317
    let answer = part2(&f)?;
    println!("Part 2: {answer}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use indoc::indoc;

    const EXAMPLE: &str = indoc!{r"
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}

        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}
    "};

    #[test]
    fn part1_example() {
        let answer = part1(Cursor::new(EXAMPLE)).unwrap();
        assert_eq!(answer, 19114);
    }

    #[test]
    fn part2_example() {
        let answer = part2(Cursor::new(EXAMPLE)).unwrap();
        assert_eq!(answer, 167409079868000);
    }
}
