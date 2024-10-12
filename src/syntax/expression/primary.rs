use chumsky::{
  Parser,
  extra::ParserExtra,
};
use crate::syntax::{
  name::Name,
  expression::{
    Expression,
    terminal::terminal_expr_parser,
  },
  util::whitespace_parser,
};

/**
 * A dot-expression accesses a component of a value expression.
 * 
 * Either `struct_value.field` or `vec_value.i`.
 * E.g. `unit.health` or `point_vec.0`
 */
#[derive(Debug, Clone)]
pub struct DotExpr<'a> {
  pub target: Box<Expression<'a>>,
  pub name: DotExprSuffix<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DotExprSuffix<'a> {
  Name(Name<'a>),
  Number(u32),
}

/**
 * A function call expression.
 */
#[derive(Debug, Clone)]
pub struct CallExpr<'a> {
  pub callee: Box<Expression<'a>>,
  pub args: Vec<Expression<'a>>,
}

pub(crate) fn primary_expr_parser<'a, E>(
  base_expr: impl Clone + Parser<'a, &'a str, Expression<'a>, E>
) -> impl Clone + Parser<'a, &'a str, Expression<'a>, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;
  enum PrimaryTail<'a> {
    Dot(DotExprSuffix<'a>),
    Call(Vec<Expression<'a>>),
  }

  let dot_tail_parser = 
    just(".").padded_by(whitespace_parser()).ignored()
      .ignore_then(
        choice((
          Name::parser().map(DotExprSuffix::Name),
          choice((
            just('0').map(|_| 0_u32),
            just('1').map(|_| 1_u32),
            just('2').map(|_| 2_u32),
            just('3').map(|_| 3_u32),
          )).map(DotExprSuffix::Number),
        ))
        .map(PrimaryTail::Dot)
      );

  let call_tail_parser =
    just("(").padded_by(whitespace_parser())
      .ignore_then(
        base_expr.clone()
          .separated_by(just(",").padded_by(whitespace_parser()))
          .allow_trailing()
          .collect::<Vec<_>>()
      )
      .then_ignore(just(")").padded_by(whitespace_parser()))
      .map(PrimaryTail::Call);

  terminal_expr_parser(base_expr)
    .then(
      choice((call_tail_parser, dot_tail_parser))
        .repeated()
        .collect::<Vec<_>>()
    ).map(|(target, tails)| {
      tails.into_iter().fold(target, |target, tail| match tail {
        PrimaryTail::Dot(name) =>
          Expression::Dot(DotExpr { target: Box::new(target), name }),

        PrimaryTail::Call(args) =>
          Expression::Call(CallExpr { callee: Box::new(target), args }),
      })
    })
}
