use std::str::FromStr;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, space0, space1};
use nom::combinator::all_consuming;
use nom::error::ErrorKind;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;

use crate::{Cubes, Game};

// implements std::error::Error if I: Debug
pub type NomError<I> = nom::Err<nom::error::Error<I>>;

/// Returns a `std::error::Error` (impl) for a parsing error
#[inline(always)]
fn nom_parse_error<I>(input: I, kind: ErrorKind) -> NomError<I> {
    let inner: nom::error::Error<I> = nom::error::make_error(input, kind);
    nom::Err::Error(inner)
}

fn unsigned<N: FromStr>(i: &str) -> IResult<&str, N> {
    match digit1(i) {
        Ok((i, d)) => {
            let n = d.parse()
                .map_err(|e| nom_parse_error(i, ErrorKind::Digit))?;
            Ok((i, n))
        },
        Err(e) => Err(e),
    }
}

fn rgb(i: &str) -> IResult<&str, &str> {
    alt((tag("red"), tag("green"), tag("blue")))(i)
}

fn colour_count(i: &str) -> IResult<&str, (&str, u8)> {
    let mut parser = tuple((unsigned, space1, rgb));
    parser(i).map(|(i, (count, _, colour))| (i, (colour, count)))
}

fn cubes(i: &str) -> IResult<&str, Cubes> {
    let mut parser = separated_list1(
        tuple((char(','), space0)),
        colour_count
    );

    let (i, colours) = parser(i)?;
    let (mut r, mut g, mut b) = (None, None, None);
    for (rgb, count) in colours {
        let field = match rgb {
            "red" => &mut r,
            "green" => &mut g,
            "blue" => &mut b,
            _ => unreachable!(),
        };

        if field.is_some() {
            return Err(nom_parse_error(i, ErrorKind::Verify));
        }

        *field = Some(count);
    }

    let cubes = Cubes::rgb(r.unwrap_or(0), g.unwrap_or(0), b.unwrap_or(0));
    Ok((i, cubes))
}

fn game_id(i: &str) -> IResult<&str, u64> {
    let mut parser = tuple((
        tag("Game"), space1, unsigned, space0, char(':')
    ));

    parser(i).map(|(i, (_, _, id, _, _))| (i, id))
}

pub fn game(i: &str) -> IResult<&str, Game> {
    let mut parser = all_consuming(tuple((
        game_id, space0, separated_list1(tuple((char(';'), space0)), cubes)
    )));

    parser(i).map(|(i, (id, _, cubes))| (i, Game::new(id, cubes)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_game() {
        let s = "Game 92: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let (i, g) = game(s).unwrap();
        let draws = vec![
            Cubes::rgb(4, 0, 3),
            Cubes::rgb(1, 2, 6),
            Cubes::rgb(0, 2, 0),
        ];

        assert_eq!(i, "");
        assert_eq!(g, Game::new(92, draws));
    }

    #[test]
    fn parse_game_id() {
        assert_eq!(game_id("Game 4:x").unwrap(), ("x", 4));
    }

    #[test]
    fn parse_cubes() {
        let s = "3 blue, 4 red, 1 green";
        assert_eq!(cubes(s).unwrap(), ("", Cubes::rgb(4, 1, 3)));
    }

    #[test]
    fn parse_rgb() {
        for colour in ["red", "green", "blue"] {
            assert_eq!(rgb(colour).unwrap(), ("", colour));
        }
    }

    #[test]
    fn parse_colour() {
        assert_eq!(colour_count("4 blue").unwrap(), ("", ("blue", 4)));
    }
}
