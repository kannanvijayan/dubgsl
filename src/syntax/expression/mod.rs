mod unary;
mod terminal;
mod primary;
mod bit;
mod mul;
mod add;
mod shift;
mod relational;
mod logical;

pub use self::{
  primary::{ CallExpr, DotExpr, DotExprSuffix },
  terminal::{ IntLiteralExpr, NameExpr, ParenExpr },
  unary::{ UnaryExpr, UnaryExprOp },
  bit::{ BitExpr, BitExprOp },
  mul::{ MulExpr, MulExprOp },
  add::{ AddExpr, AddExprOp },
  shift::{ ShiftExpr, ShiftExprOp },
  relational::{ RelationalExpr, RelationalExprOp },
  logical::{ LogicalExpr, LogicalExprOp }
};
pub(crate) use self::{
  primary::primary_expr_parser,
  terminal::terminal_expr_parser,
  unary::unary_expr_parser,
  bit::bit_expr_parser,
  mul::mul_expr_parser,
  add::add_expr_parser,
  shift::shift_expr_parser,
  relational::relational_expr_parser,
  logical::logical_expr_parser,
};

use chumsky::{
  extra::ParserExtra,
  Boxed,
};

/**
 * An expression in the language.
 */
#[derive(Debug, Clone)]
pub enum Expression<'a> {
  Name(NameExpr<'a>),
  IntLiteral(IntLiteralExpr<'a>),
  Paren(ParenExpr<'a>),
  Dot(DotExpr<'a>),
  Call(CallExpr<'a>),
  Unary(UnaryExpr<'a>),
  Bit(BitExpr<'a>),
  Mul(MulExpr<'a>),
  Add(AddExpr<'a>),
  Shift(ShiftExpr<'a>),
  Relational(RelationalExpr<'a>),
  Logical(LogicalExpr<'a>)
}
impl<'a> Expression<'a> {
  pub fn boxed(self) -> Box<Self> {
    Box::new(self)
  }

  pub fn parser<E>()
    -> Boxed<'a, 'a, &'a str, Expression<'a>, E>
    where E: ParserExtra<'a, &'a str>,
  {
    use chumsky::prelude::*;

    recursive(|expr_parser| {
      choice((
        logical_expr_parser(expr_parser.clone()),
        bit_expr_parser(expr_parser),
      ))
    }).boxed()
  }
}