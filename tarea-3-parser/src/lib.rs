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

type Res<T, U> = IResult<T, U, VerboseError<T>>;

// use tarea_3_parser::vars;

pub fn a(input: &str) {

}

// type Res<T, U> = IResult<T, U, VerboseError<T>>;

// fn method() -> str{take_while1(is_alpha);} 
// fn space() -> str{take_while1(|c| c == ' ');}
// fn url() -> str{take_while1(|c| c!= ' ');} 
// fn is_version() -> str{|c| c >= b'0' && c <= b'9' || c == b'.';} 
// fn http() -> str{tag("HTTP/");} 
// fn version() -> str{take_while1(is_version);}
// fn line_ending() -> str{tag("\r\n");}

// #[derive(Debug, PartialEq, Eq)]
// pub enum Tipo {
//     INT,
//     FLOAT,
// }

// impl From<&str> for Tipo {
//     fn from(i: &str) -> Self {
//         match i.to_lowercase().as_str() {
//             "int" => Tipo::INT,
//             "float" => Tipo::FLOAT,
//             _ => unimplemented!("no other schemes supported"),
//         }
//     }
// }

// -   VARS
//     VARS -> var DEF  
//     DEF -> DEFINICION DEF'  
//     DEF' -> DEF | vacío  
//     DEFINICION -> DEFID : TIPO ;  
//     DEFID -> id DEFID'  
//     DEFID' -> , DEFID | vacío

// pub type Authority<'a> = (&'a str, Option<&'a str>);

// #[derive(Debug, PartialEq, Eq)]
// pub enum Host {
//     HOST(String),
//     IP([u8; 4]),
// }

// pub type QueryParam<'a> = (&'a str, &'a str);
// pub type QueryParams<'a> = Vec<QueryParam<'a>>;

// fn semicolon()   -> IResult<&[u8], char> { char!(';'); }
// fn colon()       -> IResult<&[u8], char> { char!(':'); }
// fn coma()        -> IResult<&[u8], char> { char!(','); }
// fn varter()      -> IResult<&[T], &[T]> { tag!("var"); }
// fn intter()      -> IResult<&[T], &[T]> { tag!("int"); }
// fn floatter()    -> IResult<&[T], &[T]> { tag!("float"); }
// fn idter()       -> IResult<&[T], &[T]> { tag!("id"); }
// fn space()       -> IResult<&[T], &[T]> { take_while!(|c| c == ' '); }
// fn method()      -> IResult<&[T], &[T]> { take_while!(is_alphabetic); }
// fn line_ending() -> IResult<&[T], &[T]> { tag!("\r\n"); }

// // combine all previous parsers in one function
// fn request_line(i: &[u8]) -> IResult<&[u8], char> {

//     // tuple takes as argument a tuple of parsers and will return
//     // a tuple of their results
//     let (input, (varter, _, id, _, colon, _, intter, _, semicolon)) = tuple((varter, space, id, space, colon, space, intter, space, semicolon, line_ending))(i)?;
  
//     Ok((input, "a"))
// }