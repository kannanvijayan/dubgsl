use chumsky::{
  Parser,
  extra::ParserExtra,
};
use crate::syntax::{
  declaration::{
    Declaration,
    EntrypointDecl,
    BufferDecl,
    ImportDecl,
    InstanceDecl,
    FuncDecl,
    ModuleDecl,
    StructDecl,
    entrypoint_decl_parser,
    buffer_decl_parser,
    import_decl_parser,
    instance_decl_parser,
    func_decl_parser,
    module_decl_parser,
    struct_decl_parser,
  },
  util::whitespace_parser,
};

/**
 * A shader file.
 */
#[derive(Debug, Clone)]
pub struct ShaderFile<'a> {
  pub declarations: Vec<ShaderFileDeclaration<'a>>,
}
impl<'a> ShaderFile<'a> {
  pub fn parser<E>() -> impl Clone + Parser<'a, &'a str, Self, E>
    where E: ParserExtra<'a, &'a str>
  {
    use chumsky::prelude::*;

    whitespace_parser()
    .ignore_then(
      ShaderFileDeclaration::parser()
        .separated_by(whitespace_parser())
        .collect::<Vec<_>>()
        .map(|declarations| {
          ShaderFile { declarations }
        })
    ).boxed()
  }
}

/**
 * A declaration in a shader file.
 */
#[derive(Debug, Clone)]
pub enum ShaderFileDeclaration<'a> {
  Entrypoint(EntrypointDecl<'a>),
  Buffer(BufferDecl<'a>),
  Import(ImportDecl<'a>),
  Instance(InstanceDecl<'a>),
  Func(FuncDecl<'a>),
  Module(ModuleDecl<'a>),
  Struct(StructDecl<'a>),
}
impl<'a> ShaderFileDeclaration<'a> {
  pub fn parser<E>() -> impl Clone + Parser<'a, &'a str, Self, E>
    where E: ParserExtra<'a, &'a str>
  {
    use chumsky::prelude::*;

    choice((
      entrypoint_decl_parser().map(ShaderFileDeclaration::Entrypoint),
      buffer_decl_parser().map(ShaderFileDeclaration::Buffer),
      import_decl_parser().map(ShaderFileDeclaration::Import),
      instance_decl_parser().map(ShaderFileDeclaration::Instance),
      func_decl_parser().map(ShaderFileDeclaration::Func),
      module_decl_parser(Declaration::parser_for_module())
        .map(ShaderFileDeclaration::Module),
      struct_decl_parser().map(ShaderFileDeclaration::Struct),
    ))
    .boxed()
  }
}
