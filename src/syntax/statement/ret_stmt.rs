use chumsky::{
  Parser,
  extra::ParserExtra,
};
use crate::syntax::{
  expression::Expression,
  util::whitespace1_parser,
};

/**
 * A bitwise (and, or, xor) binary expression.
 */
#[derive(Debug, Clone)]
pub struct RetStmt<'a> {
  pub value: Option<Box<Expression<'a>>>,
}

pub(crate) fn ret_stmt_parser<'a, E>()
  -> impl Clone + Parser<'a, &'a str, RetStmt<'a>, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;

  just("ret").then(whitespace1_parser())
    .ignore_then(Expression::parser().or_not())
    .map(|expr| {
      RetStmt { value: expr.map(Box::new) }
    })
}
