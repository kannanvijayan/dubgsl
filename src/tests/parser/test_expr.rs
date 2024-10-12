use core::str;

use chumsky::{
  Parser,
  extra::Default,
};
use crate::syntax::expression::{
  Expression,
  terminal_expr_parser,
};

#[test]
fn test_exprs() {
  test_terminal_expr_str("hello");
  test_terminal_expr_str("55");
  test_terminal_expr_str(" (0x30  )");
  test_terminal_expr_str("
  (
      (  - 0b100000000001   )
        )");
}

fn test_terminal_expr_str(s: &str) {
  let parsed = expr_parser().parse(s);
  match parsed.into_result() {
    Ok(_) => {},
    Err(e) => panic!("Failed to parse: {} - {:?}", s, e),
  }
}

fn expr_parser<'a>()
  -> impl Clone + Parser<'a, &'a str, Expression<'a>, Default>
{
  use chumsky::prelude::*;
  recursive(|base_expr| {
    terminal_expr_parser(base_expr)
  })
}
