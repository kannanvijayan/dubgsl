use chumsky::{
  Parser,
  extra::ParserExtra,
};
use crate::syntax::{
  expression::{
    Expression,
    unary_expr_parser,
  },
  util::whitespace_parser,
};

/**
 * A shift (shl, shr) binary expression.
 */
#[derive(Debug, Clone)]
pub struct ShiftExpr<'a> {
  pub lhs: Box<Expression<'a>>,
  pub op: ShiftExprOp,
  pub rhs: Box<Expression<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShiftExprOp {
  Shl,
  Shr,
}

pub(crate) fn shift_expr_parser<'a, E>(
  base_expr: impl 'a + Clone + Parser<'a, &'a str, Expression<'a>, E>
) -> impl 'a + Clone + Parser<'a, &'a str, Expression<'a>, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;

  let shift_op_parser = choice((
    just("<<").map(|_| ShiftExprOp::Shl),
    just(">>").map(|_| ShiftExprOp::Shr),
  ));

  unary_expr_parser(base_expr.clone())
    .then(
      shift_op_parser.padded_by(whitespace_parser())
        .then(unary_expr_parser(base_expr.clone()))
        .or_not()
    )
    .map(|(lhs, maybe_rest)| {
      match maybe_rest {
        Some((op, rhs)) =>
          Expression::Shift(ShiftExpr { lhs: lhs.boxed(), op, rhs: rhs.boxed() }),
        None => lhs
      }
    })
    .boxed()
}
