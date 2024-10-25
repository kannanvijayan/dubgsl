use chumsky::{
  Parser,
  extra::ParserExtra,
};
use crate::syntax::{
  name::{ Name, NamePath },
  terminal_semicolon_parser,
  types::TypeName,
  util::{ whitespace_parser, whitespace1_parser },
};

/**
 * An import declaration.
 */
#[derive(Debug, Clone)]
pub struct InstanceDecl<'a> {
  pub name: Name<'a>,
  pub module_name: NamePath<'a>,
  pub module_params: Vec<TypeName<'a>>,
}

pub(crate) fn instance_decl_parser<'a, E>()
  -> impl Clone + Parser<'a, &'a str, InstanceDecl<'a>, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;

  just("instance").then(whitespace1_parser())
    .ignore_then(Name::parser())
    .then_ignore(just('=').padded_by(whitespace_parser()))
    .then(NamePath::parser())
    .then(
      TypeName::parser()
        .separated_by(just(',').padded_by(whitespace_parser()))
        .allow_trailing()
        .collect::<Vec<_>>()
        .delimited_by(
          just('(').padded_by(whitespace_parser()),
          just(')').padded_by(whitespace_parser())
        )
    )
    .then_ignore(terminal_semicolon_parser())
    .map(|((name, module_name), module_params)| {
      InstanceDecl { name, module_name, module_params }
    })
    .boxed()
}
