use chumsky::{
  Parser,
  extra::Default,
};
use crate::syntax::declaration::Declaration;

#[test]
fn test_declarations() {
  test_decl_str("import foo::bar::bang ;");
  test_decl_str("import foo::bar::bang as baz;");
  test_decl_str("import foo::
  bar
::bang as
             baz;");
  test_decl_str("import Test123;");

  test_decl_str("func foo() -> int {}");
  test_decl_str("func foo() -> int {
    ret 9;
  }");
  test_decl_str("func
    bar(
      a: int,
      b: Thing::Zing,
      c: Something) -> A::B::C
        ::D {
    ret a + b * c + d;
  }");

  test_decl_str("struct Foo { a: int, b: bool }");

  test_decl_str("buffer(r) B: int;");
  test_decl_str("buffer(rw  ) B: int;");
  test_decl_str("buffer(w) elevations: Terrain::Elevations;");

  test_decl_str("entrypoint(2d) gen_terrain(point) {
    let x = point.x;
    let y = point.y;
  }");
  test_decl_str("module Foo(
    buffer(r) terrain: Terrain::Elevations
  ) {
    func bar() -> int {
      ret 9;
    }
  }");

  test_decl_str("instance T = Foo::Bar(terrain_buffer, Terrain::Elevation);");
}

fn test_decl_str(s: &str) {
  let parsed = declaration_parser().parse(s);
  match parsed.into_result() {
    Ok(_) => {},
    Err(e) => panic!("Failed to parse: {} - {:?}", s, e),
  }
}

fn declaration_parser<'a>()
  -> impl Clone + Parser<'a, &'a str, Declaration<'a>, Default>
{
  Declaration::parser()
}
