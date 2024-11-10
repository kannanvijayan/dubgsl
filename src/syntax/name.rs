use chumsky::{ Parser, extra::ParserExtra };
use crate::syntax::util::{
  lowercase_parser,
  uppercase_parser,
  dec_digit_parser,
  whitespace_parser,
};

/**
 * A syntactic name (identifier)
 */
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name<'a> {
    pub contents: &'a str,
}
impl<'a> Name<'a> {
  pub fn parser<E>() -> impl Clone + Parser<'a, &'a str, Name<'a>, E>
    where E: ParserExtra<'a, &'a str>
  {
    use chumsky::prelude::*;
    ident_start_parser()
      .then(ident_follow_parser().repeated())
      .to_slice()
      .map(|contents| Name { contents })
  }
}


/**
 * A namespace path, separated by '::'
 */
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamePath<'a> {
  pub parts: Vec<Name<'a>>,
}
impl<'a> NamePath<'a> {
  pub fn parser<E>() -> impl Clone + Parser<'a, &'a str, NamePath<'a>, E>
    where E: ParserExtra<'a, &'a str>
  {
    use chumsky::prelude::*;
    Name::parser()
      .separated_by(
        just("::").padded_by(whitespace_parser())
      )
      .at_least(1)
      .collect::<Vec<_>>()
      .map(|mut parts| {
        NamePath { parts }
      })
      .boxed()
  }

  pub fn is_single(&self) -> bool {
    self.parts.len() == 1
  }
}

/**
 * Parser for identifier start character.
 */
fn ident_start_parser<'a, E>() -> impl Clone + Parser<'a, &'a str, char, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;
  choice((
    lowercase_parser(),
    uppercase_parser(),
    just('_'),
  ))
}

/**
 * Parser for identifier follow character.
 */
fn ident_follow_parser<'a, E>() -> impl Clone + Parser<'a, &'a str, char, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;
  choice((
    ident_start_parser(),
    dec_digit_parser(),
  ))
}
