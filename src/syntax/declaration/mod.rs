mod buffer_decl;
mod entrypoint_decl;
mod func_decl;
mod import_decl;
mod instance_decl;
mod module_decl;
mod struct_decl;

use crate::syntax::util::whitespace_parser;

pub use self::{
  buffer_decl::BufferDecl,
  entrypoint_decl::EntrypointDecl,
  func_decl::FuncDecl,
  import_decl::ImportDecl,
  instance_decl::InstanceDecl,
  module_decl::ModuleDecl,
  struct_decl::StructDecl,
};
pub(crate) use self::{
  buffer_decl::buffer_decl_parser,
  entrypoint_decl::entrypoint_decl_parser,
  func_decl::func_decl_parser,
  import_decl::import_decl_parser,
  instance_decl::instance_decl_parser,
  module_decl::module_decl_parser,
  struct_decl::struct_decl_parser,
};

use chumsky::{
  Boxed,
  Parser,
  extra::ParserExtra,
};

/**
 * A single declaration.
 */
#[derive(Debug, Clone)]
pub enum Declaration<'a> {
  Buffer(BufferDecl<'a>),
  Entrypoint(EntrypointDecl<'a>),
  Import(ImportDecl<'a>),
  Instance(InstanceDecl<'a>),
  Func(FuncDecl<'a>),
  Module(ModuleDecl<'a>),
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

    recursive(|decl_parser| {
      choice((
        entrypoint_decl_parser().map(Declaration::Entrypoint),
        buffer_decl_parser().map(Declaration::Buffer),
        import_decl_parser().map(Declaration::Import),
        instance_decl_parser().map(Declaration::Instance),
        func_decl_parser().map(Declaration::Func),
        module_decl_parser(decl_parser).map(Declaration::Module),
        struct_decl_parser().map(Declaration::Struct),
      ))
      .padded_by(whitespace_parser())
    }).boxed()
  }

  pub fn parser_for_module<E>()
    -> Boxed<'a, 'a, &'a str, Declaration<'a>, E>
    where E: ParserExtra<'a, &'a str>
  {
    use chumsky::prelude::*;

    recursive(|decl_parser| {
      choice((
        func_decl_parser().map(Declaration::Func),
      ))
      .padded_by(whitespace_parser())
    }).boxed()
  }
}



/**
 * A sequential collection of declarations.
 */
#[derive(Clone, Debug)]
pub struct DeclarationBlock<'a> {
  pub statements: Vec<Declaration<'a>>,
}
impl<'a> DeclarationBlock<'a> {
  pub fn parser<E>(
    decl_parser: impl 'a + Clone + Parser<'a, &'a str, Declaration<'a>, E>
  ) -> impl 'a + Clone + Parser<'a, &'a str, DeclarationBlock<'a>, E>
    where E: ParserExtra<'a, &'a str>
  {
    use chumsky::prelude::*;

    decl_parser
      .repeated()
      .collect::<Vec<_>>()
      .delimited_by(
        just('{').padded_by(whitespace_parser()),
        just('}').padded_by(whitespace_parser())
      )
      .map(|statements| DeclarationBlock { statements })
      .boxed()
  }
}
