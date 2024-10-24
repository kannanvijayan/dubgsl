mod buffer_decl;
mod entrypoint_decl;
mod func_decl;
mod import_decl;
mod struct_decl;

use crate::syntax::util::whitespace_parser;

pub use self::{
  buffer_decl::BufferDecl,
  entrypoint_decl::EntrypointDecl,
  func_decl::FuncDecl,
  import_decl::ImportDecl,
  struct_decl::StructDecl,
};
pub(crate) use self::{
  buffer_decl::buffer_decl_parser,
  entrypoint_decl::entrypoint_decl_parser,
  func_decl::func_decl_parser,
  import_decl::import_decl_parser,
  struct_decl::struct_decl_parser,
};

use chumsky::{
  Boxed,
  extra::ParserExtra,
};

/**
 * An expression in the language.
 */
#[derive(Debug, Clone)]
pub enum Declaration<'a> {
  Buffer(BufferDecl<'a>),
  Entrypoint(EntrypointDecl<'a>),
  Import(ImportDecl<'a>),
  Func(FuncDecl<'a>),
  Struct(StructDecl<'a>),
}
impl<'a> Declaration<'a> {
  pub fn boxed(self) -> Box<Self> {
    Box::new(self)
  }

  pub fn parser<E>()
    -> Boxed<'a, 'a, &'a str, Declaration<'a>, E>
    where E: ParserExtra<'a, &'a str>
  {
    use chumsky::prelude::*;

    choice((
      entrypoint_decl_parser().map(Declaration::Entrypoint),
      buffer_decl_parser().map(Declaration::Buffer),
      import_decl_parser().map(Declaration::Import),
      func_decl_parser().map(Declaration::Func),
      struct_decl_parser().map(Declaration::Struct),
    ))
    .padded_by(whitespace_parser())
    .boxed()
  }
}
