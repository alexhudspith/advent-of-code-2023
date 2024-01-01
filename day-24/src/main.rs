use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek};
use std::str::FromStr;
use itertools::Itertools;
use aoc::CollectArray;
use aoc::parse::OkOrErr;
use z3::ast::{Ast, Int, Real};

type Vec2 = [f64; 2];
type Vec3 = [f64; 3];

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Motion {
    pos: Vec3,
    vel: Vec3,
}

fn parse_vec3(s: &str) -> Result<Vec3, aoc::error::Error> {
    let result = s.split(',')
        .map(str::trim)
        .map(f64::from_str)
        .process_results(|fs| fs.try_collect_array())??;

    Ok(result)
}

fn read_hailstones<R: Read>(input: R) -> Result<Vec<Motion>, aoc::error::Error> {
    let lines = BufReader::new(input).lines();
    let mut result = Vec::new();
    for line in lines {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        let (pos_str, vel_str) = line.split('@')
            .map(str::trim)
            .collect_tuple()
            .ok_or_err(&line)?;

        let pos = parse_vec3(pos_str)?;
        let vel = parse_vec3(vel_str)?;
        result.push(Motion { pos, vel });
    }

    Ok(result)
}

fn line_intersection(a: &Motion, b: &Motion) -> Option<Vec2> {
    // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line_segment

    let [x1, y1, _] = a.pos;
    let [x2, y2] = [a.pos[0] + a.vel[0], a.pos[1] + a.vel[1]];
    let [x3, y3, _] = b.pos;
    let [x4, y4] = [b.pos[0] + b.vel[0], b.pos[1] + b.vel[1]];

    let mut t = (x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4);
    let mut u = (x1 - x3) * (y1 - y2) - (y1 - y3) * (x1 - x2);
    let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    if denom == 0.0 {
        return None;
    }

    t /= denom;
    u /= denom;
    let (px, py) = (x1 + t * (x2 - x1), y1 + t * (y2 - y1));
    (t > 0.0 && u > 0.0).then_some([px, py])
}

fn part1<R: Read>(input: R, min_pos: f64, max_pos: f64) -> Result<u64, aoc::error::Error> {
    let hailstones = read_hailstones(input)?;
    let range = min_pos..=max_pos;

    let total = hailstones.iter().tuple_combinations()
        .flat_map(|(s1, s2)| line_intersection(s1, s2))
        .filter(|&[px, py]| range.contains(&px) && range.contains(&py))
        .count();

    Ok(total as u64)
}

fn part2<R: Read>(input: R) -> Result<i64, aoc::error::Error> {
    let hailstones = read_hailstones(input)?;

    let ctx = &z3::Context::new(&z3::Config::new());
    let var = |s: &str| Real::fresh_const(ctx, s);
    let val = |v: f64| {
        assert!(v <= 2_u64.pow(53) as f64, "f64 as i64 loss");
        Int::from_i64(ctx, v as i64).to_real()
    };

    // Rock trajectory
    let [ref rx, ref ry, ref rz] = ["rx", "ry", "rz"].map(var);
    let [ref rdx, ref rdy, ref rdz] = ["rdx", "rdy", "rdz"].map(var);

    let zero = val(0.0);
    let solver = z3::Solver::new(ctx);

    // Only 3 hailstones are required to give 9 equations in 9 unknowns:
    // 6 for the rock position and velocity, and a t for each hailstone impact
    for hailstone in &hailstones[..3] {
        // Hailstone trajectory
        let [hx, hy, hz] = hailstone.pos.map(val);
        let [hdx, hdy, hdz] = hailstone.vel.map(val);
        let t = &var("t");

        for constraint in [
            t.ge(&zero),
            (hx + hdx * t)._eq(&(rx + rdx * t)),
            (hy + hdy * t)._eq(&(ry + rdy * t)),
            (hz + hdz * t)._eq(&(rz + rdz * t))
        ] {
            solver.assert(&constraint);
        }
    }

    if solver.check() != z3::SatResult::Sat {
        return Err("Unsatisfiable".into());
    };

    let model = solver.get_model().ok_or("No model")?;
    let result = model.eval(&(rx + ry + rz), true)
        .and_then(|r| r.as_real())
        .map(|(n, d)| n as f64 / d as f64)
        .ok_or("Failed to evaluate")?;

    Ok(result.round() as i64)
}

fn main() -> Result<(), aoc::error::Error> {
    let path = aoc::find_input_path("day-24");
    let mut f = File::open(path)?;
    // Answer: 14799
    let answer = part1(&f, 2e14, 4e14)?;
    println!("Part 1: {answer}");
    f.rewind()?;
    // Answer: 1007148211789625
    let answer = part2(&f)?;
    println!("Part 2: {answer}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {r"
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3
    "};

    #[test]
    fn part1_example() {
        let answer = part1(Cursor::new(EXAMPLE), 7.0, 27.0).unwrap();
        assert_eq!(answer, 2);
    }

    #[test]
    fn part2_example() {
        let answer = part2(Cursor::new(EXAMPLE)).unwrap();
        assert_eq!(answer, 47);
    }
}
