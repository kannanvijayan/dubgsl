use chumsky::{
  Parser,
  extra::ParserExtra,
};
use crate::syntax::{
  expression::Expression,
  name::Name,
  util::{ terminal_semicolon_parser, whitespace_parser },
};

/**
 * A let statement.
 */
#[derive(Debug, Clone)]
pub struct LetStmt<'a> {
  pub pieces: Vec<LetStmtPiece<'a>>,
}

#[derive(Debug, Clone)]
pub struct LetStmtPiece<'a> {
  pub name: Name<'a>,
  pub value: Expression<'a>,
}

pub(crate) fn let_stmt_parser<'a, E>()
  -> impl Clone + Parser<'a, &'a str, LetStmt<'a>, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;

  text::keyword("let").then(whitespace_parser())
    .ignore_then(
      Name::parser()
        .then_ignore(just('=').padded_by(whitespace_parser()))
        .then(Expression::parser())
        .separated_by(just(',').padded_by(whitespace_parser()))
        .collect::<Vec<_>>()
    )
    .map(|pieces| {
      let pieces =
        pieces
          .into_iter()
          .map(|(name, value)| LetStmtPiece { name, value })
          .collect::<Vec<_>>();
      LetStmt { pieces }
    })
    .then_ignore(terminal_semicolon_parser())
}
