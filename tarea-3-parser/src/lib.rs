#[macro_use]
extern crate nom;

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

mod lex;
mod vars;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

pub fn a(input: &str) {
    // vars::vars(input);
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_working() {
        assert_eq!(2+2, 4);
    }
}