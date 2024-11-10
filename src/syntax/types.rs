/**
 * Parsers for types.
 */

use chumsky::{ Parser, extra::ParserExtra };
use crate::{
  model::{
    ModelSpace,
    TypeModelHandle,
    TypeModel,
    ScalarTypeModel,
    ScalarNumericTypeModel,
  },
  syntax::name::NamePath,
};

/**
 * A type name.
 */
#[derive(Debug, Clone)]
pub struct TypeName<'a> {
  pub name: NamePath<'a>
}
impl<'a> TypeName<'a> {
  /**
   * Parse a type name.
   */
  pub fn parser<E>()
    -> impl 'a + Clone + Parser<'a, &'a str, TypeName<'a>, E>
    where E: ParserExtra<'a, &'a str>
  {
    use chumsky::prelude::*;

    NamePath::parser()
      .map(|name| TypeName { name })
  }
}
