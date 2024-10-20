use chumsky::{
  Parser,
  extra::ParserExtra,
};
use crate::syntax::{
  expression::{
    Expression,
    mul_expr_parser,
  },
  util::whitespace_parser,
};

/**
 * An additive (add, sub) binary expression.
 */
#[derive(Debug, Clone)]
pub struct AddExpr<'a> {
  pub lhs: Box<Expression<'a>>,
  pub op: AddExprOp,
  pub rhs: Box<Expression<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddExprOp {
  Add,
  Sub,
}

pub(crate) fn add_expr_parser<'a, E>(
  base_expr: impl 'a + Clone + Parser<'a, &'a str, Expression<'a>, E>
) -> impl Clone + Parser<'a, &'a str, Expression<'a>, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;

  let add_op_parser = choice((
    just('+').map(|_| AddExprOp::Add),
    just('-').map(|_| AddExprOp::Sub),
  ));

  mul_expr_parser(base_expr.clone())
    .then(
      add_op_parser.padded_by(whitespace_parser())
        .then(mul_expr_parser(base_expr.clone()))
        .repeated()
        .collect::<Vec<_>>()
        .or_not()
    )
    .map(|(first, rest)| {
      if let Some(rest) = rest {
        rest.into_iter().fold(first, |lhs, (op, rhs)| {
          Expression::Add(AddExpr { lhs: lhs.boxed(), op, rhs: rhs.boxed() })
        })
      } else {
        first
      }
    })
    .boxed()
}
