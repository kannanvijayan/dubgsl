use chumsky::{
  Parser,
  extra::ParserExtra,
};
use crate::syntax::{
  expression::{ Expression, primary_expr_parser },
  name::Name,
  util::{ terminal_semicolon_parser, whitespace1_parser, whitespace_parser },
};

/**
 * An exec statement (executes a bare expression). 
 */
#[derive(Debug, Clone)]
pub struct MutateStmt<'a> {
  pub lvalue: Box<Expression<'a>>,
  pub expr: Box<Expression<'a>>,
}

pub(crate) fn mutate_stmt_parser<'a, E>()
  -> impl Clone + Parser<'a, &'a str, MutateStmt<'a>, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;

  just("mutate").then(whitespace1_parser())
    .ignore_then(Expression::lvalue_parser())
    .then_ignore(just('=').padded_by(whitespace_parser()))
    .then(Expression::parser())
    .then_ignore(terminal_semicolon_parser())
    .map(|(lvalue, expr)| {
      MutateStmt { lvalue: lvalue.boxed(), expr: expr.boxed() }
    })
}
