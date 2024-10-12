mod unary;
mod terminal;
mod primary;

pub use self::{
  primary::{ CallExpr, DotExpr, DotExprSuffix },
  terminal::{ IntLiteralExpr, NameExpr, ParenExpr },
  unary::{ UnaryExpr, UnaryExprOp },
};
pub(crate) use self::{
  primary::primary_expr_parser,
  terminal::terminal_expr_parser,
  unary::unary_expr_parser,
};

use chumsky::{
  Parser,
  extra::ParserExtra,
};
use crate::syntax::{
  name::Name,
  util::whitespace_parser,
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
}
