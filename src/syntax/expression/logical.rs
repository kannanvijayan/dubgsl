use chumsky::{
  Parser,
  extra::ParserExtra,
};
use crate::syntax::{
  expression::{
    Expression,
    relational_expr_parser,
  },
  util::whitespace_parser,
};

/**
 * A logical (and, or) binary expression.
 */
#[derive(Debug, Clone)]
pub struct LogicalExpr<'a> {
  pub lhs: Box<Expression<'a>>,
  pub op: LogicalExprOp,
  pub rhs: Box<Expression<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalExprOp {
  Or,
  And,
}

pub(crate) fn logical_expr_parser<'a, E>(
  base_expr: impl 'a + Clone + Parser<'a, &'a str, Expression<'a>, E>
) -> impl 'a + Clone + Parser<'a, &'a str, Expression<'a>, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;

  relational_expr_parser(base_expr.clone())
    .then(
      choice((
        just("&&").padded_by(whitespace_parser())
          .ignore_then(
            relational_expr_parser(base_expr.clone())
              .separated_by(just("&&").padded_by(whitespace_parser()))
              .at_least(1)
              .collect::<Vec<_>>()
              .map(|exprs| { (LogicalExprOp::And, exprs) })
          ),
        just("||").padded_by(whitespace_parser())
          .ignore_then(
            relational_expr_parser(base_expr.clone())
              .separated_by(just("||").padded_by(whitespace_parser()))
              .at_least(1)
              .collect::<Vec<_>>()
              .map(|exprs| { (LogicalExprOp::Or, exprs) })
          ),
      ))
      .or_not()
    )
    .map(|(lhs, maybe_op_exprs)| {
      if let Some((op, exprs)) = maybe_op_exprs {
        exprs.into_iter().fold(lhs, |lhs, rhs| {
          Expression::Logical(LogicalExpr { lhs: lhs.boxed(), op, rhs: rhs.boxed() })
        })
      } else {
        lhs
      }
    })
    .boxed()
}
