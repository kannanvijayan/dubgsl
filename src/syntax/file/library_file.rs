use chumsky::{
  Parser,
  extra::ParserExtra,
};
use crate::syntax::{
  declaration::{
    Declaration,
    ImportDecl,
    FuncDecl,
    ModuleDecl,
    StructDecl,
    import_decl_parser,
    func_decl_parser,
    module_decl_parser,
    struct_decl_parser,
  },
  util::whitespace_parser,
};

/**
 * A shader library file.
 */
#[derive(Debug, Clone)]
pub struct LibraryFile<'a> {
  pub declarations: Vec<LibraryFileDeclaration<'a>>,
}
impl<'a> LibraryFile<'a> {
  pub fn parser<E>() -> impl Clone + Parser<'a, &'a str, Self, E>
    where E: ParserExtra<'a, &'a str>
  {
    use chumsky::prelude::*;

    whitespace_parser()
      .ignore_then(
        LibraryFileDeclaration::parser()
          .separated_by(whitespace_parser())
          .collect::<Vec<_>>()
          .map(|declarations| {
            LibraryFile { declarations }
          })
      )
      .boxed()
  }
}

/**
 * A declaration in a shader library file.
 */
#[derive(Debug, Clone)]
pub enum LibraryFileDeclaration<'a> {
  Import(ImportDecl<'a>),
  Func(FuncDecl<'a>),
  Module(ModuleDecl<'a>),
  Struct(StructDecl<'a>),
}
impl<'a> LibraryFileDeclaration<'a> {
  pub fn parser<E>() -> impl Clone + Parser<'a, &'a str, Self, E>
    where E: ParserExtra<'a, &'a str>
  {
    use chumsky::prelude::*;

    choice((
      import_decl_parser().map(LibraryFileDeclaration::Import),
      func_decl_parser().map(LibraryFileDeclaration::Func),
      module_decl_parser(Declaration::parser_for_module())
        .map(LibraryFileDeclaration::Module),
      struct_decl_parser().map(LibraryFileDeclaration::Struct),
    ))
    .boxed()
  }
}
