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
 * A bitwise (and, or, xor) binary expression.
 */
#[derive(Debug, Clone)]
pub struct BitExpr<'a> {
  pub lhs: Box<Expression<'a>>,
  pub op: BitExprOp,
  pub rhs: Box<Expression<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitExprOp {
  Xor,
  Or,
  And,
}

pub(crate) fn bit_expr_parser<'a, E>(
  base_expr: impl 'a + Clone + Parser<'a, &'a str, Expression<'a>, E>
) -> impl 'a + Clone + Parser<'a, &'a str, Expression<'a>, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;

  unary_expr_parser(base_expr.clone())
    .then(
      choice((
        just('^').padded_by(whitespace_parser())
          .ignore_then(
            unary_expr_parser(base_expr.clone())
              .separated_by(just('^').padded_by(whitespace_parser()))
              .at_least(1)
              .collect::<Vec<_>>()
              .map(|exprs| { (BitExprOp::Xor, exprs) })
          ),
        just('|').padded_by(whitespace_parser())
          .ignore_then(
            unary_expr_parser(base_expr.clone())
              .separated_by(just('|').padded_by(whitespace_parser()))
              .at_least(1)
              .collect::<Vec<_>>()
              .map(|exprs| { (BitExprOp::Or, exprs) })
          ),
        just('&').padded_by(whitespace_parser())
          .ignore_then(
            unary_expr_parser(base_expr.clone())
              .separated_by(just('&').padded_by(whitespace_parser()))
              .at_least(1)
              .collect::<Vec<_>>()
              .map(|exprs| { (BitExprOp::And, exprs) })
          ),
      ))
      .or_not()
    )
    .map(|(lhs, maybe_op_exprs)| {
      if let Some((op, exprs)) = maybe_op_exprs {
        exprs.into_iter().fold(lhs, |lhs, rhs| {
          Expression::Bit(BitExpr { lhs: lhs.boxed(), op, rhs: rhs.boxed() })
        })
      } else {
        lhs
      }
    }).boxed()
}
