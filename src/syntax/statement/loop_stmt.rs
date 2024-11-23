use chumsky::{
  Parser,
  extra::ParserExtra,
};
use crate::syntax::{
  statement::{ Statement, StatementBlock },
  util::whitespace_parser,
};

/**
 * A loop statement.
 */
#[derive(Debug, Clone)]
pub struct LoopStmt<'a> {
  pub block: StatementBlock<'a>,
}

pub(crate) fn loop_stmt_parser<'a, E>(
  stmt_parser: impl 'a + Clone + Parser<'a, &'a str, Statement<'a>, E>,
) -> impl 'a + Clone + Parser<'a, &'a str, LoopStmt<'a>, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;

  text::keyword("loop").then(whitespace_parser())
    .ignore_then(StatementBlock::parser(stmt_parser.clone()))
    .map(|block| {
      LoopStmt { block }
    })
    .boxed()
}
