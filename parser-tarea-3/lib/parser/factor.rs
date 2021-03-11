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
use crate::parser::expresion::*;
use crate::parser::varcte::*;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

// #[derive(Debug, PartialEq, Eq)]
// pub struct FACTOR<'a> {
//     pub exp: EXP<'a>,
//     pub signo: Signos,
//     pub exp2: &'a str,
// }

#[derive(Debug, PartialEq, Eq)]
pub struct FACTOR1<'a> {
    pub abrirllave: &'a str,
    pub expresion: EXPRESION<'a>,
    pub cerrarllave: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FACTOR2<'a> {
    pub sumaresta: SumaResta,
    pub varcte: VARCTE<'a>,
}

fn factor1(input: &str) -> Res<&str, FACTOR1> {
    context(
        "factor1",
        tuple((
            abrirllave,
            space,
            expresion,
            space,
            cerrarllave,
        )),
    )(input)
    .map(|(next_input, res)| {
        let (abrirllave, _, expresion, _, cerrarllave) = res;
        (
            next_input,
            FACTOR1 {
                abrirllave,
                expresion,
                cerrarllave,
            },
        )
    })
}

fn factor2(input: &str) -> Res<&str, FACTOR2> {
    context(
        "factor2",
        tuple((
            sumaresta,
            space,
            varcte
        )),
    )(input)
    .map(|(next_input, res)| {
        let (sumaresta, _, varcte) = res;
        (
            next_input,
            FACTOR2 {
                sumaresta,
                varcte
            },
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::termino::*;
    use crate::parser::exp::*;

    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err as NomErr,
    };

    #[test]
    fn test_factor1() {
        assert_eq!(
            factor1("{ aaa * aaa + aaaa < aaa }"),
            Ok((
                "",
                FACTOR1 {
                    abrirllave: "{",
                    expresion: EXPRESION {
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
                    cerrarllave: "}"
                }
            ))
        );
    }

    // #[test]
    // fn test_expresion() {
    //     assert_eq!(
    //         expresion("aaa * aaa + aaaa < aaa"),
    //         Ok((
    //             "",
    //             EXPRESION {
    //                 exp: EXP {
    //                     termino: TERMINO {
    //                         factor: "aaa",
    //                         multdiv: MultDiv::MULT,
    //                         factor2: "aaa"
    //                     },
    //                     sumaresta: SumaResta::SUM,
    //                     termino2: "aaaa",
    //                 },
    //                 signo: Signos::LT,
    //                 exp2: "aaa"
    //             },
    //         ))
    //     );

    //     assert_eq!(
    //         expresion("aaa * aaa + aaaa > aaa"),
    //         Ok((
    //             "",
    //             EXPRESION {
    //                 exp: EXP {
    //                     termino: TERMINO {
    //                         factor: "aaa",
    //                         multdiv: MultDiv::MULT,
    //                         factor2: "aaa"
    //                     },
    //                     sumaresta: SumaResta::SUM,
    //                     termino2: "aaaa",
    //                 },
    //                 signo: Signos::GT,
    //                 exp2: "aaa"
    //             },
    //         ))
    //     );

    //     assert_eq!(
    //         expresion("aaa * aaa - aaaa <> aaa"),
    //         Ok((
    //             "",
    //             EXPRESION {
    //                 exp: EXP {
    //                     termino: TERMINO {
    //                         factor: "aaa",
    //                         multdiv: MultDiv::MULT,
    //                         factor2: "aaa"
    //                     },
    //                     sumaresta: SumaResta::SUB,
    //                     termino2: "aaaa",
    //                 },
    //                 signo: Signos::NE,
    //                 exp2: "aaa"
    //             },
    //         ))
    //     );
    // }
}