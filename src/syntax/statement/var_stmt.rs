use chumsky::{
  Parser,
  extra::ParserExtra,
};
use crate::syntax::{
  expression::Expression,
  name::Name,
  util::{ terminal_semicolon_parser, whitespace_parser, whitespace1_parser },
};

/**
 * A var statement.
 */
#[derive(Debug, Clone)]
pub struct VarStmt<'a> {
  pub pieces: Vec<VarStmtPiece<'a>>,
}

#[derive(Debug, Clone)]
pub struct VarStmtPiece<'a> {
  pub name: Name<'a>,
  pub value: Option<Expression<'a>>,
}

pub(crate) fn var_stmt_parser<'a, E>()
  -> impl Clone + Parser<'a, &'a str, VarStmt<'a>, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;

  just("var").then(whitespace1_parser())
    .ignore_then(
      Name::parser()
        .then(
          just('=').padded_by(whitespace_parser())
            .ignore_then(Expression::parser())
            .or_not()
        )
        .separated_by(just(',').padded_by(whitespace_parser()))
        .collect::<Vec<_>>()
    )
    .map(|pieces| {
      let pieces =
        pieces
          .into_iter()
          .map(|(name, value)| VarStmtPiece { name, value })
          .collect::<Vec<_>>();
      VarStmt { pieces }
    })
    .then_ignore(terminal_semicolon_parser())
}
