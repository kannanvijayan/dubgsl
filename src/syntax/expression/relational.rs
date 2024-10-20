use chumsky::{
  Parser,
  extra::ParserExtra,
};
use crate::syntax::{
  expression::{
    Expression,
    shift_expr_parser,
    add_expr_parser,
  },
  util::whitespace_parser,
};

/**
 * An additive (add, sub) binary expression.
 */
#[derive(Debug, Clone)]
pub struct RelationalExpr<'a> {
  pub lhs: Box<Expression<'a>>,
  pub op: RelationalExprOp,
  pub rhs: Box<Expression<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationalExprOp {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
}

pub(crate) fn relational_expr_parser<'a, E>(
  base_expr: impl 'a + Clone + Parser<'a, &'a str, Expression<'a>, E>
) -> impl 'a + Clone + Parser<'a, &'a str, Expression<'a>, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;

  let relational_op_parser = choice((
    just("<=").map(|_| RelationalExprOp::LessThanOrEqual),
    just(">=").map(|_| RelationalExprOp::GreaterThanOrEqual),
    just("==").map(|_| RelationalExprOp::Equal),
    just("!=").map(|_| RelationalExprOp::NotEqual),
    just("<").map(|_| RelationalExprOp::LessThan),
    just(">").map(|_| RelationalExprOp::GreaterThan),
  ));

  let subexpr_parser = choice((
    add_expr_parser(base_expr.clone()),
    shift_expr_parser(base_expr.clone()),
  ));

  subexpr_parser.clone()
    .then(
      relational_op_parser.padded_by(whitespace_parser())
        .then(subexpr_parser)
        .or_not()
    )
    .map(|(lhs, maybe_rest)| {
      if let Some((op, rhs)) = maybe_rest {
        Expression::Relational(RelationalExpr { lhs: lhs.boxed(), op, rhs: rhs.boxed() })
      } else {
        lhs
      }
    })
    .boxed()
}
