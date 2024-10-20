use chumsky::{
  Parser,
  extra::ParserExtra,
};
use crate::syntax::{
  expression::{
    Expression,
    primary_expr_parser,
  },
  util::whitespace_parser,
};

/**
 * A dot-expression accesses a component of a value expression.
 */
#[derive(Debug, Clone)]
pub struct UnaryExpr<'a> {
  pub op: UnaryExprOp,
  pub subexpr: Box<Expression<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryExprOp {
  Negate,
  Positive,
  Not,
  Complement,
}

pub(crate) fn unary_expr_parser<'a, E>(
  base_expr: impl 'a + Clone + Parser<'a, &'a str, Expression<'a>, E>
) -> impl 'a + Clone + Parser<'a, &'a str, Expression<'a>, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;
  let unary_op_parser = choice((
    just('-').map(|_| UnaryExprOp::Negate),
    just('+').map(|_| UnaryExprOp::Positive),
    just('!').map(|_| UnaryExprOp::Not),
    just('~').map(|_| UnaryExprOp::Complement),
  ));

  unary_op_parser.padded_by(whitespace_parser())
    .repeated()
    .collect::<Vec<_>>()
    .then(primary_expr_parser(base_expr))
    .map(|(ops, expr)| {
      if ops.len() == 0 {
        expr
      } else {
        ops.into_iter().rev().fold(expr, |expr, op| {
          Expression::Unary(UnaryExpr { op, subexpr: Box::new(expr) })
        })
      }
    })
    .boxed()
}
