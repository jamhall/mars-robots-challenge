use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline, space0};
use nom::combinator::{map, map_res, recognize, verify};
use nom::multi::{many0, many1, many_m_n};
use nom::sequence::{preceded, separated_pair, terminated, tuple};
use nom::IResult;

use crate::coordinate::Coordinate;
use crate::error::ApplicationError;
use crate::instruction::Instruction;
use crate::orientation::Orientation;
use crate::rover::Rover;
use crate::Houston;

type ParsedDefinition = (Houston, Vec<(Rover, Vec<Instruction>)>);

pub fn parse(input: &str) -> Result<ParsedDefinition, ApplicationError> {
    let (input, houston) = houston(input)?;
    let (_, rovers) = rovers(input)?;

    Ok((houston, rovers))
}

fn houston(input: &str) -> IResult<&str, Houston> {
    let (input, houston) = map(terminated(coordinate, newline), Houston::new)(input)?;
    Ok((input, houston))
}

fn number(input: &str) -> IResult<&str, i64> {
    map_res(recognize(digit1), str::parse)(input)
}

fn rovers(input: &str) -> IResult<&str, Vec<(Rover, Vec<Instruction>)>> {
    many1(tuple((
        rover,
        preceded(newline, terminated(instructions, many0(newline))),
    )))(input)
}

fn rover(input: &str) -> IResult<&str, Rover> {
    let (input, (coordinate, orientation)) = separated_pair(coordinate, space0, orientation)(input)?;
    let rover = Rover::new(coordinate, orientation);
    Ok((input, rover))
}

fn coordinate(input: &str) -> IResult<&str, Coordinate> {
    verify(
        map(separated_pair(number, space0, number), |(x, y)| Coordinate::new(x, y)),
        |coordinate| {
            let (x, y) = coordinate.tuple();
            if (0..=50).contains(&x) && (0..=50).contains(&y) {
                return true;
            }
            false
        },
    )(input)
}

fn orientation(input: &str) -> IResult<&str, Orientation> {
    alt((
        map(tag("N"), |_| Orientation::North),
        map(tag("E"), |_| Orientation::East),
        map(tag("S"), |_| Orientation::South),
        map(tag("W"), |_| Orientation::West),
    ))(input)
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many_m_n(0, 100,instruction)(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        map(tag("L"), |_| Instruction::Left),
        map(tag("R"), |_| Instruction::Right),
        map(tag("F"), |_| Instruction::Forward),
    ))(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_orientation() {
        assert!(matches!(orientation("N"), Ok((_, Orientation::North))));
        assert!(matches!(orientation("E"), Ok((_, Orientation::East))));
        assert!(matches!(orientation("S"), Ok((_, Orientation::South))));
        assert!(matches!(orientation("W"), Ok((_, Orientation::West))));
    }

    #[test]
    fn test_parse_instruction() {
        assert!(matches!(instruction("L"), Ok((_, Instruction::Left))));
        assert!(matches!(instruction("R"), Ok((_, Instruction::Right))));
        assert!(matches!(instruction("F"), Ok((_, Instruction::Forward))));
    }

    #[test]
    fn test_parse_coordinate() {
        assert_eq!(coordinate("15 25"), Ok(("", Coordinate::new(15, 25))));
        assert!(coordinate("1525").is_err());
        assert!(coordinate("15,25").is_err());
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(number("20"), Ok(("", 20)));
        assert!(number("HH").is_err());
    }

    #[test]
    fn test_parse_houston() {
        assert_eq!(houston("15 25\n"), Ok(("", Houston::new(Coordinate::new(15, 25)))));
        assert!(houston("15 25").is_err());
    }
}
