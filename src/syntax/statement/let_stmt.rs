use chumsky::{
  Parser,
  extra::ParserExtra,
};
use crate::syntax::{
  expression::Expression,
  name::Name,
  util::whitespace_parser,
  util::whitespace1_parser,
};

/**
 * A bitwise (and, or, xor) binary expression.
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

  just("let").then(whitespace1_parser())
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
}
