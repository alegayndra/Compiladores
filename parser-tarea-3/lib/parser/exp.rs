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
use crate::parser::termino::*;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug, PartialEq, Eq)]
pub struct EXP<'a> {
    pub termino: TERMINO<'a>,
    pub sumaresta: SumaResta,
    pub termino2: &'a str,
}

pub fn exp(input: &str) -> Res<&str, EXP> {
    context(
        "exp",
        tuple((
            termino,
            space,
            sumaresta,
            space,
            url_code_points
        )),
    )(input)
    .map(|(next_input, res)| {
        let (termino, _, sumaresta, _, termino2) = res;
        (
            next_input,
            EXP {
                termino,
                sumaresta,
                termino2
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
    fn test_exp() {
        assert_eq!(
            exp("aaa * aaa + aaa"),
            Ok((
                "",
                EXP {
                    termino: TERMINO {
                        factor: "aaa",
                        multdiv: MultDiv::MULT,
                        factor2: "aaa"
                    },
                    sumaresta: SumaResta::SUM,
                    termino2: "aaa"
                },
            ))
        );

        assert_eq!(
            exp("aaa * aaa - aaa"),
            Ok((
                "",
                EXP {
                    termino: TERMINO {
                        factor: "aaa",
                        multdiv: MultDiv::MULT,
                        factor2: "aaa"
                    },
                    sumaresta: SumaResta::SUB,
                    termino2: "aaa"
                },
            ))
        );
    }
}