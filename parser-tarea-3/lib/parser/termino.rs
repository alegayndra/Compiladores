use nom::{
    // branch::alt,
    // bytes::complete::{tag, tag_no_case, take},
    // character::complete::{alpha1, alphanumeric1, one_of},
    // combinator::opt,
    // error::{context, ErrorKind, VerboseError},
    error::{context, VerboseError},
    // multi::{count, many0, many1, many_m_n},
    // sequence::{preceded, separated_pair, terminated, tuple},
    sequence::tuple,
    // AsChar, Err as NomErr, IResult, InputTakeAtPosition,
    IResult,
};

use crate::lexer::*;

// // use lex::*;
// // mod lex;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TERMINO<'a> {
    pub factor: &'a str,
    pub multdiv: MultDiv,
    pub factor2: &'a str,
}

pub fn termino(input: &str) -> Res<&str, TERMINO> {
    context(
        "termino",
        tuple((
            url_code_points,
            space,
            multdiv,
            space,
            url_code_points
        )),
    )(input)
    .map(|(next_input, res)| {
        let (factor, _, multdiv, _, factor2) = res;
        (
            next_input,
            TERMINO {
                factor,
                multdiv,
                factor2
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
    fn test_termino() {
        assert_eq!(
            termino("aaa * aaa"),
            Ok((
                "",
                TERMINO {
                    factor: "aaa",
                    multdiv: MultDiv::MULT,
                    factor2: "aaa"
                },
            ))
        );

        assert_eq!(
            termino("aaa / aaa"),
            Ok((
                "",
                TERMINO {
                    factor: "aaa",
                    multdiv: MultDiv::DIV,
                    factor2: "aaa"
                },
            ))
        );
    }
}