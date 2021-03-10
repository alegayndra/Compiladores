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

use crate::lexer::*;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug, PartialEq, Eq)]
pub struct BLOQUE<'a> {
    pub abrirllave: &'a str,
    pub estatuto: Vec<&'a str>,
    pub cerrarllave: &'a str
}

pub fn bloque(input: &str) -> Res<&str, BLOQUE> {
    context(
        "bloque",
        tuple((
            abrirllave,
            space,
            many0(url_code_points),
            space,
            cerrarllave
        )),
    )(input)
    .map(|(next_input, res)| {
        let (abrirllave, _, estatuto, _, cerrarllave,) = res;
        (
            next_input,
            BLOQUE {
                abrirllave,
                estatuto,
                cerrarllave,
            },
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    // use nom::{
    //     error::{ErrorKind, VerboseError, VerboseErrorKind},
    //     Err as NomErr,
    // };

    #[test]
    fn test_bloque() {
        assert_eq!(
            bloque("{ aaaaa }"),
            Ok((
                "", 
                BLOQUE {
                    abrirllave: "{",
                    estatuto: vec!["aaaaa"],
                    cerrarllave: "}"
                },
            ))
        );

    }
}