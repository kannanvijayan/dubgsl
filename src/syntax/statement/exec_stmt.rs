use chumsky::{
  Parser,
  extra::ParserExtra,
};
use crate::syntax::{
  expression::Expression,
  util::{ terminal_semicolon_parser, whitespace_parser },
};

/**
 * An exec statement (executes a bare expression). 
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

  text::keyword("exec").then(whitespace_parser())
    .ignore_then(
      Expression::parser()
        .map(|expr| ExecStmt { expr: expr.boxed() })
    )
    .then_ignore(terminal_semicolon_parser())
}
