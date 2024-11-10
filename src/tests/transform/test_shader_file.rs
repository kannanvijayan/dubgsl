use std::path::PathBuf;
use crate::transform::{ SessionConfigBuilder, SyntaxIngester };

const TEST_SHADER_FILE: &'static str = "
  struct Position {
    x: u32,
    y: u32,
  }

  struct Color {
    r: f32,
    g: f32,
    b: f32,
  }

  struct BirdInfo {
    position: Position,
    color: Color,
  }

  buffer(w) birds: BirdInfo;

  entrypoint(2d) init_birds(i) {
    if i >= birds.length {
      ret ;
    }
    mutate birds(i).position.x = 0;
    mutate birds(i).position.y = 0;
    mutate birds(i).color.r = 1.0;
    mutate birds(i).color.g = 0.0;
    mutate birds(i).color.b = 0.0;
  }
";

const TEST_SHADER_FILE_2: &'static str = "
  import hello;
  import world;
  struct MyStruct {
    a: i32,
    b: f32,
  }
  func my_func() {
    ret 5;
  }
";

#[test]
fn test_simple_shader_file() {
  let root_path = PathBuf::from("/test");
  let session_config =
    SessionConfigBuilder::new()
      .project_root(root_path)
      .build();
  let mut _syntax_ingester = SyntaxIngester::parse_shader_file(
    &session_config,
    "foo.dubgsl.shader",
    TEST_SHADER_FILE,
  );
}
