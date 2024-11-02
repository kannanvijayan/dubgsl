use chumsky::{
  Parser,
  extra::ParserExtra,
};
use crate::syntax::{
  name::Name,
  types::TypeName,
  util::{ terminal_semicolon_parser, whitespace_parser },
};

/**
 * An import declaration.
 */
#[derive(Debug, Clone)]
pub struct BufferDecl<'a> {
  pub name: Name<'a>,
  pub mode: BufferDeclMode,
  pub ty: TypeName<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferDeclMode {
  Read,
  Write,
  ReadWrite,
}

pub(crate) fn buffer_decl_parser<'a, E>()
  -> impl Clone + Parser<'a, &'a str, BufferDecl<'a>, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;

  just("buffer")
    .ignore_then(
      choice((
        just("rw").map(|_| BufferDeclMode::ReadWrite),
        just("r").map(|_| BufferDeclMode::Read),
        just("w").map(|_| BufferDeclMode::Write),
      ))
      .delimited_by(
        just('(').padded_by(whitespace_parser()),
        just(')').padded_by(whitespace_parser())
      )
    )
    .then_ignore(whitespace_parser())
    .then(Name::parser())
    .then_ignore(just(':').padded_by(whitespace_parser()))
    .then(TypeName::parser())
    .then_ignore(terminal_semicolon_parser())
    .map(|((mode, name), ty)| {
      BufferDecl { name, mode, ty }
    })
    .boxed()
}
