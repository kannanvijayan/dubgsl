use chumsky::{
  Parser,
  extra::ParserExtra,
};
use crate::syntax::{
  statement::{ Statement, StatementBlock },
  util::whitespace1_parser,
};

/**
 * A bitwise (and, or, xor) binary expression.
 */
#[derive(Debug, Clone)]
pub struct LoopStmt<'a> {
  pub block: StatementBlock<'a>,
}

pub(crate) fn loop_stmt_parser<'a, E>(
  stmt_parser: impl Clone + Parser<'a, &'a str, Statement<'a>, E>,
) -> impl Clone + Parser<'a, &'a str, LoopStmt<'a>, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;

  just("loop").then(whitespace1_parser())
    .ignore_then(StatementBlock::parser(stmt_parser.clone()))
    .map(|block| {
      LoopStmt { block }
    })
}