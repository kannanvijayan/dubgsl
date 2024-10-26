
pub mod name;
pub mod types;
pub mod expression;
pub mod statement;
pub mod declaration;
pub mod file;

pub(crate) mod util;

use chumsky::{
  Parser,
  extra::ParserExtra,
};
use self::util::whitespace_parser;

/**
 * A helper to terminate a statement with a semicolon.
 */
pub(crate) fn terminal_semicolon_parser<'a, E>()
  -> impl Clone + Parser<'a, &'a str, (), E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;
  just(';').padded_by(whitespace_parser()).map(|_| ())
}
