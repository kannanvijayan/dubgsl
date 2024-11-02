use chumsky::{
  Parser,
  extra::Default,
};
use crate::syntax::statement::Statement;

#[test]
fn test_statements() {
  test_stmt_str("let x = 3;");
  test_stmt_str("let y = !(9 + foo(33) == 4) ;");
  test_stmt_str("var z = !(9 + foo(33) == 4) ;  ");
  test_stmt_str("var z = !(9 ^ foo(33) == 4) ;  ");
  test_stmt_str("var z = !(9 | foo(33) == 4) ;  ");
  test_stmt_str("var z = !(9 & foo(33) == 4) ;  ");
  test_stmt_str("var q = zang.trib(33), nn = !false;");

  test_exec_ret_expr("3
  ;");
  test_exec_ret_expr("!(9 + foo(33) == 4) ;");
  test_exec_ret_expr("zang.trib(33)
     ;");
  test_exec_ret_expr("zang.trib(33)

;   ");
  test_exec_ret_expr("a << 9   ;");

  test_stmt_str("if 3 { let x = 3; } else { let y = 4; }");
  test_stmt_str("if 3 { let x = 3; } else { }");
  test_stmt_str("if x.bang == 3 {} else { let y = 4; }");
  test_stmt_str("if foo(bar >> 9, 3, abc.def(33)) == abc.q { let x = 3; }");
  test_stmt_str("loop  {
    let a = 9;
    if (a == foo.bar >> (22 * q + 5 * (-z.q(9)))) {
      exec doThing(a + 2);
    } else {
       ret blah(foo(a)) ;
    }
  }
    ");
}

fn test_exec_ret_expr(s: &str) {
  test_stmt_str(&format!("exec {}", s));
  test_stmt_str(&format!("ret {}", s));
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
