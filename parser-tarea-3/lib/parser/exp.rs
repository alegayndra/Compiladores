use nom::{
    branch::alt,
    // bytes::complete::{tag, tag_no_case, take},
    bytes::complete::tag_no_case,
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

#[derive(Debug, PartialEq, Eq)]
pub enum SumaResta {
    SUM,
    SUB
}

impl From<&str> for SumaResta {
    fn from(i: &str) -> Self {
        match i.to_lowercase().as_str() {
            "+" => SumaResta::SUM,
            "-" => SumaResta::SUB,
            _ => unimplemented!("no other schemes supported"),
        }
    }
}

fn sumaresta(input: &str) -> Res<&str, SumaResta> {
    context(
        "sumaresta",
        alt((tag_no_case("+"), tag_no_case("-"))),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
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
    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err as NomErr,
    };

    #[test]
    fn test_sumaresta() {
        assert_eq!(sumaresta("+"), Ok(("", SumaResta::SUM)));
        assert_eq!(sumaresta("-"), Ok(("", SumaResta::SUB)));
        assert_eq!(
            sumaresta("laksl"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    ("laksl", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("laksl", VerboseErrorKind::Nom(ErrorKind::Alt)),
                    ("laksl", VerboseErrorKind::Context("sumaresta")),
                ]
            }))
        );
    }

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