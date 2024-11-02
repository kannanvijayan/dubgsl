use chumsky::{
  Parser,
  extra::Default,
};
use crate::syntax::name::{ Name, NamePath };

#[test]
fn test_names() {
  check_name("hello");
  check_name("a");
  check_name("_");
  check_name("_abc");
  check_name("_abc_def");
  check_name("_9bc_def");
  check_name("X9999");
  check_name("_9999");
}

#[test]
fn test_name_paths() {
  check_name_path("hello :: there");
  check_name_path("std:: string\n::bang");
}

fn check_name<'a>(str: &'static str) {
  let parsed = Name::parser::<Default>().parse(str);
  match parsed.into_result() {
    Ok(_) => {},
    Err(e) => panic!("Failed to parse: {} - {:?}", str, e),
  }
}

fn check_name_path<'a>(str: &'static str) {
  let parsed = NamePath::parser::<Default>().parse(str);
  match parsed.into_result() {
    Ok(_) => {},
    Err(e) => panic!("Failed to parse: {} - {:?}", str, e),
  }
}
