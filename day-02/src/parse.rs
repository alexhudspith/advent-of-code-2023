use nom::{IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, space0, space1, u64 as nom_u64, u8 as nom_u8};
use nom::combinator::{all_consuming, map, map_res};
use nom::error::ParseError;
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, separated_pair, tuple};

use crate::{Cubes, CubesBuilder, CubesBuilderError, Game};

pub fn trim<'i, F, O, E>(parser: F) -> impl FnMut(&'i str) -> IResult<&'i str, O, E>
    where
        F: Parser<&'i str, O, E>,
        E: ParseError<&'i str>
{
    all_consuming(delimited(space0, parser, space0))
}

fn rgb(input: &str) -> IResult<&str, &str> {
    alt((tag("red"), tag("green"), tag("blue")))(input)
}

fn colour_count(input: &str) -> IResult<&str, (u8, &str)> {
    separated_pair(nom_u8, space1, rgb)(input)
}

fn build_cubes(colour_counts: Vec<(u8, &str)>) -> Result<Cubes, CubesBuilderError> {
    let mut cubes = CubesBuilder::default();
    for (count, rgb) in colour_counts {
        let field = match rgb {
            "red" => &mut cubes.red,
            "green" => &mut cubes.green,
            "blue" => &mut cubes.blue,
            _ => unreachable!(),
        };

        if field.is_some() {
            return Err(CubesBuilderError::ValidationError(
                format!("Duplicate colour: {rgb}")
            ));
        }

        *field = Some(count);
    }

    cubes.build()
}

fn draw(input: &str) -> IResult<&str, Cubes> {
    map_res(
        separated_list1(
            tuple((space0, char(','), space0)),
            colour_count,
        ),
        build_cubes
    )(input)
}

fn draws(input: &str) -> IResult<&str, Vec<Cubes>> {
    separated_list1(
        tuple((space0, char(';'), space0)),
        draw,
    )(input)
}

fn game_id(input: &str) -> IResult<&str, u64> {
    preceded(
        tuple((tag("Game"), space1)),
        nom_u64,
    )(input)
}

pub fn game(input: &str) -> IResult<&str, Game> {
    trim(
        map(
            separated_pair(
                game_id,
                tuple((space0, char(':'), space0)),
                draws,
            ),
            |(id, draws)| Game::new(id, draws),
        )
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_game() {
        let s = "Game 92: 3 blue, 4 red; 1 red, 2 green, 6 blue ;2 green";
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
        assert_eq!(game_id("Game 4:x").unwrap(), (":x", 4));
    }

    #[test]
    fn parse_draw() {
        let s = "3 blue, 4 red, 1 green";
        assert_eq!(draw(s).unwrap(), ("", Cubes::rgb(4, 1, 3)));
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
