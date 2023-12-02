use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, space0, space1, u8 as nom_u8, u64 as nom_u64};
use nom::combinator::all_consuming;
use nom::error::ErrorKind;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair, tuple};

use crate::{Cubes, Game};

// implements std::error::Error if I: Debug
pub type NomError<I> = nom::Err<nom::error::Error<I>>;

/// Returns a `std::error::Error` (impl) for a parsing error
fn nom_parse_error<I>(input: I, kind: ErrorKind) -> NomError<I> {
    let inner: nom::error::Error<I> = nom::error::make_error(input, kind);
    nom::Err::Error(inner)
}

fn rgb(input: &str) -> IResult<&str, &str> {
    alt((tag("red"), tag("green"), tag("blue")))(input)
}

fn colour_count(input: &str) -> IResult<&str, (u8, &str)> {
    separated_pair(nom_u8, space1, rgb)(input)
}

fn cubes(input: &str) -> IResult<&str, Cubes> {
    let (input, colour_counts) = separated_list1(
        tuple((space0, char(','), space0)),
        colour_count
    )(input)?;

    let (mut r, mut g, mut b) = (None, None, None);
    for (count, rgb) in colour_counts {
        let field = match rgb {
            "red" => &mut r,
            "green" => &mut g,
            "blue" => &mut b,
            _ => unreachable!(),
        };

        if field.is_some() {
            return Err(nom_parse_error(input, ErrorKind::Verify));
        }

        *field = Some(count);
    }

    let cubes = Cubes::rgb(r.unwrap_or(0), g.unwrap_or(0), b.unwrap_or(0));
    Ok((input, cubes))
}

fn game_id(input: &str) -> IResult<&str, u64> {
    delimited(
        tuple((tag("Game"), space1)),
        nom_u64,
        tuple((space0, char(':')))
    )(input)
}

pub fn game(input: &str) -> IResult<&str, Game> {
    let (input, (id, cubes)) = all_consuming(
        separated_pair(
            game_id,
            space0,
            separated_list1(
                tuple((char(';'), space0)),
                cubes
            )
        )
    )(input)?;

    Ok((input, Game::new(id, cubes)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_game() {
        let s = "Game 92: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let (input, g) = game(s).unwrap();
        let draws = vec![
            Cubes::rgb(4, 0, 3),
            Cubes::rgb(1, 2, 6),
            Cubes::rgb(0, 2, 0),
        ];

        assert_eq!(input, "");
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
        assert_eq!(colour_count("4 blue").unwrap(), ("", (4, "blue")));
    }
}
