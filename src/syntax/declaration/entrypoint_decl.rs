use chumsky::{
  Parser,
  extra::ParserExtra,
};
use crate::syntax::{
  name::Name,
  statement::{ Statement, StatementBlock },
  util::whitespace_parser,
};

/**
 * An entrypoint declaration.
 */
#[derive(Debug, Clone)]
pub struct EntrypointDecl<'a> {
  pub name: Name<'a>,
  pub dims: EntrypointDeclDims,
  pub arg_name: Name<'a>,
  pub body: StatementBlock<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EntrypointDeclDims {
  D1 = 1,
  D2 = 2,
  D3 = 3,
}

pub(crate) fn entrypoint_decl_parser<'a, E>()
  -> impl Clone + Parser<'a, &'a str, EntrypointDecl<'a>, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;

  text::keyword("entrypoint").then(whitespace_parser())
    .ignore_then(
      choice((
        just("1d").map(|_| EntrypointDeclDims::D1),
        just("2d").map(|_| EntrypointDeclDims::D2),
        just("3d").map(|_| EntrypointDeclDims::D3),
      ))
      .delimited_by(
        just('(').padded_by(whitespace_parser()),
        just(')').padded_by(whitespace_parser()),
      )
    )
    .then(Name::parser())
    .then(
      Name::parser()
        .delimited_by(
          just('(').padded_by(whitespace_parser()),
          just(')').padded_by(whitespace_parser()),
        )
    )
    .then(StatementBlock::parser(Statement::parser()))
    .map(|(((dims, name), arg_name), body)| {
      EntrypointDecl { dims, name, arg_name, body }
    })
    .boxed()
}
