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
 * A dot-expression accesses a component of a value expression.
 */
#[derive(Debug, Clone)]
pub struct MulExpr<'a> {
  pub lhs: Box<Expression<'a>>,
  pub op: MulExprOp,
  pub rhs: Box<Expression<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MulExprOp {
  Mul,
  Div,
  Mod,
}

pub(crate) fn mul_expr_parser<'a, E>(
  base_expr: impl Clone + Parser<'a, &'a str, Expression<'a>, E>
) -> impl Clone + Parser<'a, &'a str, Expression<'a>, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;

  let mul_op_parser = choice((
    just('*').map(|_| MulExprOp::Mul),
    just('/').map(|_| MulExprOp::Div),
    just('%').map(|_| MulExprOp::Mod),
  ));

  unary_expr_parser(base_expr.clone())
    .then(
      mul_op_parser.padded_by(whitespace_parser())
        .then(unary_expr_parser(base_expr.clone()))
        .repeated()
        .collect::<Vec<_>>()
        .or_not()
    )
    .map(|(first, rest)| {
      if let Some(rest) = rest {
        rest.into_iter().fold(first, |lhs, (op, rhs)| {
          Expression::Mul(MulExpr { lhs: lhs.boxed(), op, rhs: rhs.boxed() })
        })
      } else {
        first
      }
    })
}
