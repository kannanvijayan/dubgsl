use chumsky::{
  Parser,
  extra::ParserExtra,
};
use crate::syntax::{
  name::Name,
  types::TypeName,
  util::{ whitespace_parser, whitespace1_parser },
};

/**
 * A struct declaration.
 */
#[derive(Debug, Clone)]
pub struct StructDecl<'a> {
  pub name: Name<'a>,
  pub fields: Vec<StructDeclField<'a>>,
}

#[derive(Debug, Clone)]
pub struct StructDeclField<'a> {
  pub name: Name<'a>,
  pub ty: TypeName<'a>,
}

pub(crate) fn struct_decl_parser<'a, E>()
  -> impl Clone + Parser<'a, &'a str, StructDecl<'a>, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;

  just("struct")
    .then(whitespace1_parser())
    .ignore_then(Name::parser())
    .then(
      struct_decl_field_parser()
        .separated_by(just(',').padded_by(whitespace_parser()))
        .allow_trailing()
        .collect::<Vec<_>>()
        .delimited_by(
          just('{').padded_by(whitespace_parser()),
          just('}').padded_by(whitespace_parser()),
        )
    )
    .map(|(name, fields)| {
      StructDecl { name, fields }
    })
    .boxed()
}


pub(crate) fn struct_decl_field_parser<'a, E>()
  -> impl Clone + Parser<'a, &'a str, StructDeclField<'a>, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;

  Name::parser()
    .then_ignore(just(':').padded_by(whitespace_parser()))
    .then(TypeName::parser())
    .map(|(name, ty)| StructDeclField { name, ty })
}
