use chumsky::{
  Parser,
  extra::Default,
};
use crate::syntax::file::{ LibraryFile, ShaderFile };

#[test]
fn test_files() {
  test_library_file_str("
    import hello;
    import world;
    struct MyStruct {
      a: i32,
      b: f32,
    }
    func my_func() {
      ret 5;
    }");

  test_shader_file_str("
    import hello;
    import world;
    struct MyStruct {
      a: i32,
      b: f32,
    }
    func my_func() {
      ret 5;
    }");
}

fn test_shader_file_str(s: &str) {
  let parsed = shader_file_parser().parse(s);
  match parsed.into_result() {
    Ok(_) => {},
    Err(e) => panic!("Failed to parse: {} - {:?}", s, e),
  }
}

fn test_library_file_str(s: &str) {
  let parsed = library_file_parser().parse(s);
  match parsed.into_result() {
    Ok(_) => {},
    Err(e) => panic!("Failed to parse: {} - {:?}", s, e),
  }
}

fn library_file_parser<'a>()
  -> impl Clone + Parser<'a, &'a str, LibraryFile<'a>, Default>
{
  LibraryFile::parser()
}


fn shader_file_parser<'a>()
  -> impl Clone + Parser<'a, &'a str, ShaderFile<'a>, Default>
{
  ShaderFile::parser()
}
