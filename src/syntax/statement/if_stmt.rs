use chumsky::{
  Parser,
  extra::ParserExtra,
};
use crate::syntax::{
  expression::Expression,
  statement::{ Statement, StatementBlock },
  util::whitespace_parser,
};

/**
 * An if statement.
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

  text::keyword("if").then(whitespace_parser())
    .ignore_then(Expression::parser())
    .then_ignore(whitespace_parser())
    .then(StatementBlock::parser(stmt_parser.clone()))
    .then(
      text::keyword("else").padded_by(whitespace_parser())
        .ignore_then(StatementBlock::parser(stmt_parser.clone()))
        .or_not()
    )
    .map(|((cond, if_block), else_block)| {
      IfStmt { cond: cond.boxed(), if_block, else_block }
    })
    .boxed()
}
