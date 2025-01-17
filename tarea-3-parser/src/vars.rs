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

// use lex::*;
mod lex;
// use lexer::*;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug, PartialEq, Eq)]
pub struct VARS<'a> {
    varter: &'a str,
    ids: Vec<&'a str>,
    colon: &'a str,
    tipo: Tipo,
    semicolon: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Tipo {
    INT,
    FLOAT,
}

impl From<&str> for Tipo {
    fn from(i: &str) -> Self {
        match i.to_lowercase().as_str() {
            "int" => Tipo::INT,
            "float" => Tipo::FLOAT,
            _ => unimplemented!("no other schemes supported"),
        }
    }
}

fn tipo(input: &str) -> Res<&str, Tipo> {
    context(
        "tipo",
        alt((tag_no_case("int"), tag_no_case("float"))),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

fn varter(input: &str) -> Res<&str, &str> {
    context(
        "varter",
        tag_no_case("var"),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

pub fn vars(input: &str) -> Res<&str, VARS> {
    context(
        "vars",
        tuple((
            varter,
            space,
            ids,
            space,
            colon,
            space,
            tipo,
            space,
            semicolon,
        )),
    )(input)
    .map(|(next_input, res)| {
        let (varter, _, ids, _, colon, _, tipo, _, semicolon) = res;
        (
            next_input,
            VARS {
                varter,
                ids,
                colon,
                tipo,
                semicolon,
            },
        )
    })
}

fn ids(input: &str) -> Res<&str, Vec<&'a str>> {
    context(
        "ids",
        tuple((
            url_code_points,
            many0(tuple((
                tag(","),
                url_code_points,
            ))),
        )),
    )(input)
    .map(|(next_input, res)| {
        let mut qps = Vec::new();
        qps.push(res.0);
        for qp in res.1 {
            qps.push(qp.1);
        }
        (next_input, qps)
    })
}

fn url_code_points(input: &str) -> Res<&str, &str> {
    input.split_at_position1_complete(
        |item| {
            let char_item = item.as_char();
            !(char_item == '-') && !char_item.is_alphanum() && !(char_item == '.')
        },
        ErrorKind::AlphaNumeric,
    )
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
    fn test_tipo() {
        assert_eq!(tipo("intaa"), Ok(("aa", Tipo::INT)));
        assert_eq!(tipo("floataa"), Ok(("aa", Tipo::FLOAT)));
        assert_eq!(
            tipo("laksl"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    ("laksl", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("laksl", VerboseErrorKind::Nom(ErrorKind::Alt)),
                    ("laksl", VerboseErrorKind::Context("tipo")),
                ]
            }))
        );
    }

    #[test]
    fn test_varter() {
        assert_eq!(varter("var"), Ok(("", "var")));
        assert_eq!(
            varter("a"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    ("a", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("a", VerboseErrorKind::Context("varter")),
                ]
            }))
        );
    }

    #[test]
    fn test_ids() {
        assert_eq!(ids("id"), Ok(("", vec!["id"])));
        assert_eq!(ids("id, id"), Ok(("", vec!["id", "id"])));
        assert_eq!(
            ids(":"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    (":", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    (":", VerboseErrorKind::Context("ids")),
                ]
            }))
        );
    }

    #[test]
    fn test_vars() {
        assert_eq!(
            vars("var id : int ;"),
            Ok((
                "",
                VARS {
                    varter: "var",
                    ids: vec!["id"],
                    colon: ":",
                    tipo: Tipo::INT,
                    semicolon: ";",
                }
            ))
        );

        // assert_eq!(
        //     uri("http://localhost"),
        //     Ok((
        //         "",
        //         URI {
        //             scheme: Scheme::HTTP,
        //             authority: None,
        //             host: Host::HOST("localhost".to_string()),
        //             port: None,
        //             path: None,
        //             query: None,
        //             fragment: None
        //         }
        //     ))
        // );

        // assert_eq!(
        //     uri("https://www.zupzup.org:443/about/?someVal=5#anchor"),
        //     Ok((
        //         "",
        //         URI {
        //             scheme: Scheme::HTTPS,
        //             authority: None,
        //             host: Host::HOST("www.zupzup.org".to_string()),
        //             port: Some(443),
        //             path: Some(vec!["about"]),
        //             query: Some(vec![("someVal", "5")]),
        //             fragment: Some("anchor")
        //         }
        //     ))
        // );

        // assert_eq!(
        //     uri("http://user:pw@127.0.0.1:8080"),
        //     Ok((
        //         "",
        //         URI {
        //             scheme: Scheme::HTTP,
        //             authority: Some(("user", Some("pw"))),
        //             host: Host::IP([127, 0, 0, 1]),
        //             port: Some(8080),
        //             path: None,
        //             query: None,
        //             fragment: None
        //         }
        //     ))
        // );
    }
}