use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take},
    character::complete::{alpha1, alphanumeric1, one_of},
    combinator::opt,
    error::{context, ErrorKind, VerboseError},
    multi::{count, many0, many1, many_m_n},
    sequence::{preceded, separated_pair, terminated, tuple},
    AsChar, Err as NomErr, IResult, InputTakeAtPosition,
};

pub fn colon(input: &str) -> Res<&str, &str> {
    context(
        "colon",
        tag_no_case(":"),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

pub fn semicolon(input: &str) -> Res<&str, &str> {
    context(
        "semicolon",
        tag_no_case(";"),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

pub fn space(input: &str) -> Res<&str, &str> {
    context(
        "space",
        tag_no_case(" "),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}