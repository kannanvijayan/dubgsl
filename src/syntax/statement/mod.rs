mod exec_stmt;
mod let_stmt;
mod var_stmt;
mod ret_stmt;
mod if_stmt;
mod loop_stmt;

use crate::syntax::util::whitespace_parser;

pub use self::{
  exec_stmt::ExecStmt,
  let_stmt::{ LetStmt, LetStmtPiece },
  var_stmt::{ VarStmt, VarStmtPiece },
  ret_stmt::RetStmt,
  if_stmt::IfStmt,
  loop_stmt::LoopStmt,
};
pub(crate) use self::{
  exec_stmt::exec_stmt_parser,
  let_stmt::let_stmt_parser,
  var_stmt::var_stmt_parser,
  ret_stmt::ret_stmt_parser,
  if_stmt::if_stmt_parser,
  loop_stmt::loop_stmt_parser,
};

use chumsky::{
  Boxed,
  Parser,
  extra::ParserExtra,
};

/**
 * An expression in the language.
 */
#[derive(Debug, Clone)]
pub enum Statement<'a> {
  Exec(ExecStmt<'a>),
  Let(LetStmt<'a>),
  Var(VarStmt<'a>),
  Ret(RetStmt<'a>),
  If(IfStmt<'a>),
  Loop(LoopStmt<'a>),
}
impl<'a> Statement<'a> {
  pub fn boxed(self) -> Box<Self> {
    Box::new(self)
  }

  pub fn parser<E>()
    -> Boxed<'a, 'a, &'a str, Statement<'a>, E>
    where E: ParserExtra<'a, &'a str>
  {
    use chumsky::prelude::*;

    recursive(|stmt_parser| {
      choice((
        exec_stmt_parser().map(Statement::Exec),
        let_stmt_parser().map(Statement::Let),
        var_stmt_parser().map(Statement::Var),
        ret_stmt_parser().map(Statement::Ret),
        if_stmt_parser(stmt_parser.clone()).map(Statement::If),
        loop_stmt_parser(stmt_parser).map(Statement::Loop),
      ))
    }).boxed()
  }
}


/**
 * A block of statements.
 */
#[derive(Clone, Debug)]
pub struct StatementBlock<'a> {
  pub statements: Vec<Statement<'a>>,
}
impl<'a> StatementBlock<'a> {
  pub fn parser<E>(
    stmt_parser: impl 'a + Clone + Parser<'a, &'a str, Statement<'a>, E>
  ) -> impl 'a + Clone + Parser<'a, &'a str, StatementBlock<'a>, E>
    where E: ParserExtra<'a, &'a str>
  {
    use chumsky::prelude::*;

    stmt_parser
      .repeated()
      .collect::<Vec<_>>()
      .delimited_by(
        just('{').padded_by(whitespace_parser()),
        just('}').padded_by(whitespace_parser())
      )
      .map(|statements| StatementBlock { statements })
      .boxed()
  }
}

/**
 * A helper to terminate a statement with a semicolon.
 */
fn terminal_semicolon_parser<'a, E>()
  -> impl Clone + Parser<'a, &'a str, (), E>
  where E: ParserExtra<'a, &'a str>
{
  use chumsky::prelude::*;
  just(';').padded_by(whitespace_parser()).map(|_| ())
}