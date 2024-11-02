use chumsky::{
  Parser,
  extra::Default,
};
use crate::syntax::expression::IntLiteralExpr;

#[test]
fn test_int_literals() {
  check_int_literal("55");
  check_int_literal("0x30");
  check_int_literal("- 0b101110111");
  check_int_literal("0_1_0");
  check_int_literal("+ 0o707");
  check_int_literal("-
3309");
}

fn check_int_literal<'a>(str: &'static str) {
  let parsed = IntLiteralExpr::parser::<Default>().parse(str);
  match parsed.into_result() {
    Ok(_) => {},
    Err(e) => panic!("Failed to parse: {} - {:?}", str, e),
  }
}
