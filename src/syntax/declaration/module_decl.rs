use chumsky::{
  Parser,
  extra::ParserExtra,
};
use crate::syntax::{
  name::Name,
  types::TypeName,
  declaration::{ Declaration, DeclarationBlock },
  util::{ whitespace_parser, whitespace1_parser },
};

/**
 * A module declaration.
 */
#[derive(Debug, Clone)]
pub struct ModuleDecl<'a> {
  pub name: Name<'a>,
  pub params: Vec<ModuleDeclParam<'a>>,
  pub body: DeclarationBlock<'a>,
}

#[derive(Debug, Clone)]
pub enum ModuleDeclParam<'a> {
  Buffer(ModuleDeclBufferParam<'a>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferParamMode {
  Read,
  Write,
  ReadWrite,
}

#[derive(Debug, Clone)]
pub struct ModuleDeclBufferParam<'a> {
  pub mode: BufferParamMode,
  pub name: Name<'a>,
  pub ty: TypeName<'a>,
}

#[derive(Debug, Clone)]
pub struct ModuleDeclTypeParam<'a> {
  pub name: Name<'a>,
}

pub(crate) fn module_decl_parser<'a, E>(
  decl_parser: impl 'a + Clone + Parser<'a, &'a str, Declaration<'a>, E>,
) -> impl Clone + Parser<'a, &'a str, ModuleDecl<'a>, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;

  just("module").then(whitespace1_parser())
    .ignore_then(Name::parser())
    .then(
      module_decl_param_parser()
        .separated_by(just(',').padded_by(whitespace_parser()))
        .allow_trailing()
        .collect::<Vec<_>>()
        .delimited_by(
          just('(').padded_by(whitespace_parser()),
          just(')').padded_by(whitespace_parser()),
        )
    )
    .then(DeclarationBlock::parser(decl_parser))
    .map(|((name, params), body)| {
      ModuleDecl { name, params, body }
    })
    .boxed()
}

pub(crate) fn module_decl_param_parser<'a, E>()
  -> impl Clone + Parser<'a, &'a str, ModuleDeclParam<'a>, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;

  module_decl_buffer_param_parser().map(ModuleDeclParam::Buffer)
}

pub(crate) fn module_decl_buffer_param_parser<'a, E>()
  -> impl Clone + Parser<'a, &'a str, ModuleDeclBufferParam<'a>, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;

  just("buffer")
    .ignore_then(
      choice((
        just("rw").map(|_| BufferParamMode::ReadWrite),
        just("r").map(|_| BufferParamMode::Read),
        just("w").map(|_| BufferParamMode::Write),
      ))
      .delimited_by(
        just('(').padded_by(whitespace_parser()),
        just(')').padded_by(whitespace_parser())
      )
    )
    .then(Name::parser())
    .then_ignore(just(':').padded_by(whitespace_parser()))
    .then(TypeName::parser())
    .map(|((mode, name), ty)| {
      ModuleDeclBufferParam { mode, name, ty }
    })
}
