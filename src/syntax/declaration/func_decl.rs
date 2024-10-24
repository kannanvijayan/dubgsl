use chumsky::{
  Parser,
  extra::ParserExtra,
};
use crate::syntax::{
  name::Name,
  types::TypeName,
  statement::{ Statement, StatementBlock },
  util::{ whitespace_parser, whitespace1_parser },
};

/**
 * A function declaration.
 */
#[derive(Debug, Clone)]
pub struct FuncDecl<'a> {
  pub name: Name<'a>,
  pub arguments: Vec<FuncDeclArgument<'a>>,
  pub return_type: Option<TypeName<'a>>,
  pub body: StatementBlock<'a>,
}

#[derive(Debug, Clone)]
pub struct FuncDeclArgument<'a> {
  pub name: Name<'a>,
  pub ty: TypeName<'a>,
}

pub(crate) fn func_decl_parser<'a, E>()
  -> impl Clone + Parser<'a, &'a str, FuncDecl<'a>, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;

  just("func")
    .then(whitespace1_parser())
    .ignore_then(Name::parser())
    .then(
      func_decl_argument_parser()
        .separated_by(just(',').padded_by(whitespace_parser()))
        .allow_trailing()
        .collect::<Vec<_>>()
        .delimited_by(
          just('(').padded_by(whitespace_parser()),
          just(')').padded_by(whitespace_parser()),
        )
    )
    .then(
      just("->").padded_by(whitespace_parser())
        .ignore_then(TypeName::parser())
        .or_not()
    )
    .then(StatementBlock::parser(Statement::parser()))
    .map(|(((name, arguments), return_type), body)| {
      FuncDecl { name, arguments, return_type, body }
    })
    .boxed()
}

pub(crate) fn func_decl_argument_parser<'a, E>()
  -> impl Clone + Parser<'a, &'a str, FuncDeclArgument<'a>, E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;

  Name::parser()
    .then_ignore(just(':').padded_by(whitespace_parser()))
    .then(TypeName::parser())
    .map(|(name, ty)| FuncDeclArgument { name, ty })
}
