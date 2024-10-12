/**
 * Terminal expressions.
 */

use chumsky::{ Parser, extra::ParserExtra };
use crate::syntax::{
  expression::Expression,
  name::NamePath,
  util::{
    dec_digit_parser,
    bin_digit_parser,
    oct_digit_parser,
    hex_digit_parser,
    whitespace_parser,
  },
};


/**
 * Name expression.
 */
#[derive(Clone, Debug)]
pub struct NameExpr<'a> {
  pub name: NamePath<'a>,
}
impl<'a> NameExpr<'a> {
  pub fn parser<E>() -> impl Clone + Parser<'a, &'a str, NameExpr<'a>, E>
    where E: ParserExtra<'a, &'a str>
  {
    use chumsky::prelude::*;
    NamePath::parser()
      .map(|name| NameExpr { name })
  }
}

/**
 * Integer literal expression.
 */
#[derive(Clone, Debug)]
pub struct IntLiteralExpr<'a> {
  pub sign: Option<IntLiteralExprSign>,
  pub base: Option<IntLiteralExprBase>,
  pub ty: Option<IntLiteralExprType>,
  pub value: &'a str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IntLiteralExprSign {
  Negative,
  Positive,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IntLiteralExprBase {
  Decimal,
  Hexadecimal,
  Binary,
  Octal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IntLiteralExprType {
  U32,
  I32,
}

impl<'a> IntLiteralExpr<'a> {
  fn digits_run_parser<X, E>(
    digit: impl Clone + Parser<'a, &'a str, X, E>
  ) -> impl Clone + Parser<'a, &'a str, &'a str, E>
    where E: ParserExtra<'a, &'a str>
  {
    use chumsky::prelude::*;
    digit.repeated().at_least(1)
      .separated_by(just("_"))
      .at_least(1)
      .to_slice()
  }

  pub fn parser<E>() -> impl Clone + Parser<'a, &'a str, IntLiteralExpr<'a>, E>
    where E: ParserExtra<'a, &'a str>
  {
    use chumsky::prelude::*;

    let sign_parser = choice((
      just('-').map(|_| IntLiteralExprSign::Negative),
      just('+').map(|_| IntLiteralExprSign::Positive),
    ));

    let dec_main_parser =
      just("0d")
        .ignore_then(
          Self::digits_run_parser(dec_digit_parser())
        )
        .map(|digits| (Some(IntLiteralExprBase::Hexadecimal), digits));
    let bin_main_parser =
      just("0b")
        .ignore_then(
          Self::digits_run_parser(bin_digit_parser())
        )
        .map(|digits| (Some(IntLiteralExprBase::Binary), digits));
    let oct_main_parser =
      just("0o")
        .ignore_then(
          Self::digits_run_parser(oct_digit_parser())
        )
        .map(|digits| (Some(IntLiteralExprBase::Octal), digits)); 
    let hex_main_parser =
      just("0x")
        .ignore_then(
          Self::digits_run_parser(hex_digit_parser())
        )
        .map(|digits| (Some(IntLiteralExprBase::Hexadecimal), digits));
    let simple_main_parser =
      Self::digits_run_parser(dec_digit_parser())
        .map(|digits| (None, digits));

    let ty_parser = choice((
      just("_u32").map(|_| IntLiteralExprType::U32),
      just("_i32").map(|_| IntLiteralExprType::I32),
    ));

    sign_parser.padded_by(whitespace_parser()).or_not()
      .then(choice((
        dec_main_parser,
        bin_main_parser,
        oct_main_parser,
        hex_main_parser,
        simple_main_parser,
      )))
      .then(ty_parser.or_not())
      .map(|((sign, (base, value)), ty)| {
        IntLiteralExpr { sign, base, ty, value }
      })
  }
}

/**
 * Parentheses-enclosed expression.
 * 
 * E.g. `(EXPR)`
 */
#[derive(Clone, Debug)]
pub struct ParenExpr<'a> {
  pub subexpr: Box<Expression<'a>>,
}
impl<'a> ParenExpr<'a> {
  pub fn parser<E>(
    base_expr: impl Clone + Parser<'a, &'a str, Expression<'a>, E>
  ) -> impl Clone + Parser<'a, &'a str, ParenExpr<'a>, E>
    where E: ParserExtra<'a, &'a str>
  {
    use chumsky::prelude::*;
    just("(").padded_by(whitespace_parser())
      .ignore_then(base_expr)
      .then_ignore(just(")").padded_by(whitespace_parser()))
      .map(|subexpr| ParenExpr { subexpr: Box::new(subexpr) })
  }
}

pub(crate) fn terminal_expr_parser<'a, E>(
  base_expr: impl Clone + Parser<'a, &'a str, Expression<'a>, E>
) -> impl Clone + Parser<'a, &'a str, Expression<'a>, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;
  choice((
    NameExpr::parser().map(Expression::Name),
    IntLiteralExpr::parser().map(Expression::IntLiteral),
    ParenExpr::parser(base_expr).map(Expression::Paren),
  ))
}
