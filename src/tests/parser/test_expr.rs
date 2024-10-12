use core::str;

use chumsky::{
  Parser,
  extra::Default,
};
use crate::syntax::expression::{
  Expression,
  terminal_expr_parser,
  primary_expr_parser,
  unary_expr_parser,
  bit_expr_parser,
};

#[test]
fn test_terminal_exprs() {
  test_terminal_expr_str("hello");
  test_terminal_expr_str("55");
  test_terminal_expr_str(" (0x30  )");
  test_terminal_expr_str("
  (
      (  - 0b100000000001   )
        )");
}

fn test_terminal_expr_str(s: &str) {
  let parsed = full_terminal_expr_parser().parse(s);
  match parsed.into_result() {
    Ok(_) => {},
    Err(e) => panic!("Failed to parse: {} - {:?}", s, e),
  }
}

fn full_terminal_expr_parser<'a>()
  -> impl Clone + Parser<'a, &'a str, Expression<'a>, Default>
{
  use chumsky::prelude::*;
  recursive(|base_expr| {
    terminal_expr_parser(base_expr)
  })
}


#[test]
fn test_primary_exprs() {
  test_primary_expr_str("hello.there");
  test_primary_expr_str("rgb.0");
  test_primary_expr_str("blend(rgb.0, rgb.1)");
}

fn test_primary_expr_str(s: &str) {
  let parsed = full_primary_expr_parser().parse(s);
  match parsed.into_result() {
    Ok(v) => {},
    Err(e) => panic!("Failed to parse: {} - {:?}", s, e),
  }
}

fn full_primary_expr_parser<'a>()
  -> impl Clone + Parser<'a, &'a str, Expression<'a>, Default>
{
  use chumsky::prelude::*;
  recursive(|base_expr| {
    primary_expr_parser(base_expr)
  })
}


#[test]
fn test_unary_exprs() {
  test_unary_expr_str("-hello.there");
  test_unary_expr_str("!rgb.0(+1, ~2, 3)");
}

fn test_unary_expr_str(s: &str) {
  let parsed = full_unary_expr_parser().parse(s);
  match parsed.into_result() {
    Ok(v) => {},
    Err(e) => panic!("Failed to parse: {} - {:?}", s, e),
  }
}

fn full_unary_expr_parser<'a>()
  -> impl Clone + Parser<'a, &'a str, Expression<'a>, Default>
{
  use chumsky::prelude::*;
  recursive(|base_expr| {
    unary_expr_parser(base_expr)
  })
}


#[test]
fn test_bit_exprs() {
  test_bit_expr_str("4|9");
  test_bit_expr_str("-foo.bar & !33");
  test_bit_expr_str("-foo.bar & !33 & bix(0x1, 0xb1 & 99)");
}

fn test_bit_expr_str(s: &str) {
  let parsed = full_bit_expr_parser().parse(s);
  match parsed.into_result() {
    Ok(v) => {},
    Err(e) => panic!("Failed to parse: {} - {:?}", s, e),
  }
}

fn full_bit_expr_parser<'a>()
  -> impl Clone + Parser<'a, &'a str, Expression<'a>, Default>
{
  use chumsky::prelude::*;
  recursive(|base_expr| {
    bit_expr_parser(base_expr)
  })
}
