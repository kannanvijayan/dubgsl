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
  mul_expr_parser,
  add_expr_parser,
  shift_expr_parser,
  relational_expr_parser,
  logical_expr_parser,
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
    Ok(_) => {},
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
    Ok(_) => {},
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
  test_bit_expr_str("-foo.bar & !33 & bix(0x1, 0xb1 ^ 99)");
  test_bit_expr_str("-foo.bar & (1 ^ -0b0 ^ zang(-33, ~55) ^ +99) & 55 & 55 & 55");
}

fn test_bit_expr_str(s: &str) {
  let parsed = full_bit_expr_parser().parse(s);
  match parsed.into_result() {
    Ok(_) => {},
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

#[test]
fn test_mul_exprs() {
  test_mul_expr_str("3 * 9");
  test_mul_expr_str("3 / 5 * 9");
  test_mul_expr_str("foo(bang) / bang(bar * 9, 5 / 3 % 2) * -33.bang % 9 * 5 / -0x3");
  test_mul_expr_str("-foo.bar * (1 / -0b0 * zang(-33, ~55) % +99) % 55 % 55 * 55");
}

fn test_mul_expr_str(s: &str) {
  let parsed = full_mul_expr_parser().parse(s);
  match parsed.into_result() {
    Ok(_) => {},
    Err(e) => panic!("Failed to parse: {} - {:?}", s, e),
  }
}

fn full_mul_expr_parser<'a>()
  -> impl Clone + Parser<'a, &'a str, Expression<'a>, Default>
{
  use chumsky::prelude::*;
  recursive(|base_expr| {
    mul_expr_parser(base_expr)
  })
}

#[test]
fn test_add_exprs() {
  test_add_expr_str("3 * 9 + foo(x) - -2");
  test_add_expr_str("-22 + 3 / 5 * 9");
  test_add_expr_str("-0x3 - foo(bang - 4 + 5 + -0x33) / bang(bar * 9 + 0x9, 2 + 5 / 3 + 9 % 2) * -33.bang % 9 * 5 / -0x3 + 55");
}

fn test_add_expr_str(s: &str) {
  let parsed = full_add_expr_parser().parse(s);
  match parsed.into_result() {
    Ok(_) => {},
    Err(e) => panic!("Failed to parse: {} - {:?}", s, e),
  }
}

fn full_add_expr_parser<'a>()
  -> impl Clone + Parser<'a, &'a str, Expression<'a>, Default>
{
  use chumsky::prelude::*;
  recursive(|base_expr| {
    add_expr_parser(base_expr)
  })
}

#[test]
fn test_shift_exprs() {
  test_shift_expr_str("foo << bar");
  test_shift_expr_str("foo >> bar");
  test_shift_expr_str("foo >> !bar");
  test_shift_expr_str("~foo >> bar(bang << 3)");
}

fn test_shift_expr_str(s: &str) {
  let parsed = full_shift_expr_parser().parse(s);
  match parsed.into_result() {
    Ok(res) => {
      println!("KVKV - SHIFT-EXPR: {:?}", res);
    },
    Err(e) => panic!("Failed to parse: {} - {:?}", s, e),
  }
}

fn full_shift_expr_parser<'a>()
  -> impl Clone + Parser<'a, &'a str, Expression<'a>, Default>
{
  use chumsky::prelude::*;
  recursive(|base_expr| {
    shift_expr_parser(base_expr)
  })
}

#[test]
fn test_relational_exprs() {
  test_relational_expr_str("foo <= bar");
  // test_relational_expr_str("bang + 9 == 9");
  test_relational_expr_str("bang + 9 == bax(car > +cdr.this)")
}

fn test_relational_expr_str(s: &str) {
  let parsed = full_relational_expr_parser().parse(s);
  match parsed.into_result() {
    Ok(_) => {},
    Err(e) => panic!("Failed to parse: {} - {:?}", s, e),
  }
}

fn full_relational_expr_parser<'a>()
  -> impl Clone + Parser<'a, &'a str, Expression<'a>, Default>
{
  use chumsky::prelude::*;
  recursive(|base_expr| {
    relational_expr_parser(base_expr)
  })
}

#[test]
fn test_logical_exprs() {
  test_logical_expr_str("foo <= bar && bang == 9*3 + 2 && fox(33 || -44) != 9");
  test_logical_expr_str("foo <= bar || (bang && 33 && car.bar) == 9*3 + 2 || fox(33 && -44) != 9");
}

fn test_logical_expr_str(s: &str) {
  let parsed = full_logical_expr_parser().parse(s);
  match parsed.into_result() {
    Ok(_) => {},
    Err(e) => panic!("Failed to parse: {} - {:?}", s, e),
  }
}

fn full_logical_expr_parser<'a>()
  -> impl Clone + Parser<'a, &'a str, Expression<'a>, Default>
{
  use chumsky::prelude::*;
  recursive(|base_expr| {
    logical_expr_parser(base_expr)
  })
}
