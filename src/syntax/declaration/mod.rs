mod func_decl;
mod import_decl;
mod struct_decl;

use crate::syntax::util::whitespace_parser;

pub use self::{
  func_decl::FuncDecl,
  import_decl::ImportDecl,
  struct_decl::StructDecl,
};
pub(crate) use self::{
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
      import_decl_parser().map(Declaration::Import),
      func_decl_parser().map(Declaration::Func),
      struct_decl_parser().map(Declaration::Struct),
    ))
    .padded_by(whitespace_parser())
    .boxed()
  }
}
