use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take, take_while},
    character::complete::{alpha1, alphanumeric1, one_of},
    combinator::opt,
    error::{context, ErrorKind, VerboseError},
    multi::{count, many0, many1, many_m_n},
    sequence::{preceded, separated_pair, terminated, tuple},
    AsChar, Err as NomErr, IResult, InputTakeAtPosition,
};

type Res<T, U> = IResult<T, U, VerboseError<T>>;

pub fn colon(input: &str) -> Res<&str, &str> {
    context(
        "colon",
        tag_no_case(":"),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

pub fn coma(input: &str) -> Res<&str, &str> {
    context(
        "coma",
        tag_no_case(","),
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
        take_while(|c| c == ' '),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err as NomErr,
    };

    #[test]
    fn test_colon() {
        assert_eq!(colon(":"), Ok(("", ":")));
        assert_eq!(
            colon("a"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    ("a", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("a", VerboseErrorKind::Context("colon")),
                ]
            }))
        );
    }

    #[test]
    fn test_semicolon() {
        assert_eq!(semicolon(";"), Ok(("", ";")));
        assert_eq!(
            semicolon("a"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    ("a", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("a", VerboseErrorKind::Context("semicolon")),
                ]
            }))
        );
    }

    #[test]
    fn test_coma() {
        assert_eq!(coma(","), Ok(("", ",")));
        assert_eq!(
            coma("a"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    ("a", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("a", VerboseErrorKind::Context("coma")),
                ]
            }))
        );
    }

    #[test]
    fn test_space() {
        assert_eq!(space(" "), Ok(("", " ")));
        assert_eq!(space("    "), Ok(("", "    ")));
        assert_eq!(space(""), Ok(("", "")));
        assert_eq!(space("a"), Ok(("a", "")));
    }
}
