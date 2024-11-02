use chumsky::{
  Parser,
  extra::ParserExtra,
};
use crate::syntax::{
  name::{ Name, NamePath },
  util::{ terminal_semicolon_parser, whitespace_parser, whitespace1_parser },
};

/**
 * An import declaration.
 */
#[derive(Debug, Clone)]
pub struct ImportDecl<'a> {
  pub name_path: NamePath<'a>,
  pub maybe_alias: Option<Name<'a>>,
}

pub(crate) fn import_decl_parser<'a, E>()
  -> impl Clone + Parser<'a, &'a str, ImportDecl<'a>, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;

  just("import")
    .then(whitespace1_parser())
    .ignore_then(NamePath::parser())
    .then(
      whitespace_parser()
      .then(just("as"))
      .then(whitespace1_parser())
      .ignore_then(Name::parser())
      .or_not()
    )
    .then_ignore(terminal_semicolon_parser())
    .map(|(name_path, maybe_alias)| {
      ImportDecl { name_path, maybe_alias }
    })
    .boxed()
}
