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
pub struct ExecStmt<'a> {
  pub expr: Box<Expression<'a>>,
}

pub(crate) fn exec_stmt_parser<'a, E>()
  -> impl Clone + Parser<'a, &'a str, ExecStmt<'a>, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;

  just("exec")
    .then(whitespace1_parser())
    .ignore_then(
      Expression::parser()
        .map(|expr| ExecStmt { expr: expr.boxed() })
    )
}
