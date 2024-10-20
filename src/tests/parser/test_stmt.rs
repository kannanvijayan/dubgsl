use core::str;

use chumsky::{
  Parser,
  extra::Default,
};
use crate::syntax::statement::Statement;

#[test]
fn test_statements() {
  test_stmt_str("let x = 3");
}

fn test_stmt_str(s: &str) {
  let parsed = statement_parser().parse(s);
  match parsed.into_result() {
    Ok(_) => {},
    Err(e) => panic!("Failed to parse: {} - {:?}", s, e),
  }
}

fn statement_parser<'a>()
  -> impl Clone + Parser<'a, &'a str, Statement<'a>, Default>
{
  Statement::parser()
}