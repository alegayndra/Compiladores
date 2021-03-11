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
use crate::parser::exp::*;
use crate::parser::termino::*;

// // use lex::*;
// // mod lex;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug, PartialEq, Eq)]
pub struct EXPRESION<'a> {
    pub exp: EXP<'a>,
    pub signo: Signos,
    pub exp2: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Signos {
    GT,
    LT,
    NE
}

impl From<&str> for Signos {
    fn from(i: &str) -> Self {
        match i.to_lowercase().as_str() {
            "<>" => Signos::NE,
            ">" => Signos::GT,
            "<" => Signos::LT,
            _ => unimplemented!("no other schemes supported"),
        }
    }
}

fn signos(input: &str) -> Res<&str, Signos> {
    context(
        "signos",
        alt((tag_no_case("<>"), tag_no_case("<"), tag_no_case(">"))),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

pub fn expresion(input: &str) -> Res<&str, EXPRESION> {
    context(
        "expresion",
        tuple((
            exp,
            space,
            signos,
            space,
            url_code_points
        )),
    )(input)
    .map(|(next_input, res)| {
        let (exp, _, signo, _, exp2) = res;
        (
            next_input,
            EXPRESION {
                exp,
                signo,
                exp2
            },
        )
    })
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn is_working() {
    //     assert_eq!(2+2, 4);
    // }

    use super::*;
    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err as NomErr,
    };

    #[test]
    fn test_signos() {
        assert_eq!(signos(">"), Ok(("", Signos::GT)));
        assert_eq!(signos("<"), Ok(("", Signos::LT)));
        assert_eq!(signos("<>"), Ok(("", Signos::NE)));
        assert_eq!(
            signos("laksl"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    ("laksl", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("laksl", VerboseErrorKind::Nom(ErrorKind::Alt)),
                    ("laksl", VerboseErrorKind::Context("signos")),
                ]
            }))
        );
    }

    #[test]
    fn test_expresion() {
        assert_eq!(
            expresion("aaa * aaa + aaaa < aaa"),
            Ok((
                "",
                EXPRESION {
                    exp: EXP {
                        termino: TERMINO {
                            factor: "aaa",
                            multdiv: MultDiv::MULT,
                            factor2: "aaa"
                        },
                        sumaresta: SumaResta::SUM,
                        termino2: "aaaa",
                    },
                    signo: Signos::LT,
                    exp2: "aaa"
                },
            ))
        );

        assert_eq!(
            expresion("aaa * aaa + aaaa > aaa"),
            Ok((
                "",
                EXPRESION {
                    exp: EXP {
                        termino: TERMINO {
                            factor: "aaa",
                            multdiv: MultDiv::MULT,
                            factor2: "aaa"
                        },
                        sumaresta: SumaResta::SUM,
                        termino2: "aaaa",
                    },
                    signo: Signos::GT,
                    exp2: "aaa"
                },
            ))
        );

        assert_eq!(
            expresion("aaa * aaa - aaaa <> aaa"),
            Ok((
                "",
                EXPRESION {
                    exp: EXP {
                        termino: TERMINO {
                            factor: "aaa",
                            multdiv: MultDiv::MULT,
                            factor2: "aaa"
                        },
                        sumaresta: SumaResta::SUB,
                        termino2: "aaaa",
                    },
                    signo: Signos::NE,
                    exp2: "aaa"
                },
            ))
        );
    }
}