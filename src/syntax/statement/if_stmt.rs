use chumsky::{
  Parser,
  extra::ParserExtra,
};
use crate::syntax::{
  expression::Expression,
  statement::{ Statement, StatementBlock },
  util::{ whitespace_parser, whitespace1_parser },
};

/**
 * A bitwise (and, or, xor) binary expression.
 */
#[derive(Debug, Clone)]
pub struct IfStmt<'a> {
  pub cond: Box<Expression<'a>>,
  pub if_block: StatementBlock<'a>,
  pub else_block: Option<StatementBlock<'a>>,
}

pub(crate) fn if_stmt_parser<'a, E>(
  stmt_parser: impl 'a + Clone + Parser<'a, &'a str, Statement<'a>, E>,
) -> impl 'a + Clone + Parser<'a, &'a str, IfStmt<'a>, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;

  just("if").then(whitespace1_parser())
    .ignore_then(Expression::parser())
    .then_ignore(whitespace_parser())
    .then(StatementBlock::parser(stmt_parser.clone()))
    .then(
      whitespace_parser()
        .ignore_then(just("else").then(whitespace1_parser()))
        .ignore_then(StatementBlock::parser(stmt_parser.clone()))
        .or_not()
    )
    .map(|((cond, if_block), else_block)| {
      IfStmt { cond: cond.boxed(), if_block, else_block }
    })
    .boxed()
}