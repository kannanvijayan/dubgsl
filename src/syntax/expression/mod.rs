mod unary;
mod terminal;
mod primary;
mod bit;
mod mul;
mod add;
mod shift;
mod relational;
mod logical;

pub use self::{
  primary::{ CallExpr, DotExpr, DotExprSuffix },
  terminal::{ IntLiteralExpr, NameExpr, ParenExpr },
  unary::{ UnaryExpr, UnaryExprOp },
  bit::{ BitExpr, BitExprOp },
  mul::{ MulExpr, MulExprOp },
  add::{ AddExpr, AddExprOp },
  shift::{ ShiftExpr, ShiftExprOp },
  relational::{ RelationalExpr, RelationalExprOp },
  logical::{ LogicalExpr, LogicalExprOp }
};
pub(crate) use self::{
  primary::primary_expr_parser,
  terminal::terminal_expr_parser,
  unary::unary_expr_parser,
  bit::bit_expr_parser,
  mul::mul_expr_parser,
  add::add_expr_parser,
  shift::shift_expr_parser,
  relational::relational_expr_parser,
  logical::logical_expr_parser,
};

use chumsky::{
  Parser,
  extra::ParserExtra,
  Boxed,
};
use crate::syntax::util::whitespace_parser;

/**
 * An expression in the language.
 */
#[derive(Debug, Clone)]
pub enum Expression<'a> {
  Name(NameExpr<'a>),
  IntLiteral(IntLiteralExpr<'a>),
  Paren(ParenExpr<'a>),
  Dot(DotExpr<'a>),
  Call(CallExpr<'a>),
  Unary(UnaryExpr<'a>),
  Bit(BitExpr<'a>),
  Mul(MulExpr<'a>),
  Add(AddExpr<'a>),
  Shift(ShiftExpr<'a>),
  Relational(RelationalExpr<'a>),
  Logical(LogicalExpr<'a>)
}
impl<'a> Expression<'a> {
  pub fn boxed(self) -> Box<Self> {
    Box::new(self)
  }

  pub fn parser<E>()
    -> Boxed<'a, 'a, &'a str, Expression<'a>, E>
    where E: ParserExtra<'a, &'a str>,
  {
    Self::shift_reduce_parser()
  }

  pub fn lvalue_parser<E>()
    -> Boxed<'a, 'a, &'a str, Expression<'a>, E>
    where E: ParserExtra<'a, &'a str>,
  {
    use chumsky::prelude::*;

    recursive(|expr_parser| {
      primary_expr_parser(expr_parser)
    }).boxed()
  }

  pub fn shift_reduce_parser<E>()
    -> Boxed<'a, 'a, &'a str, Expression<'a>, E>
    where E: ParserExtra<'a, &'a str>,
  {
    use chumsky::prelude::*;

    recursive(|expr_parser| {
      unary_expr_parser(expr_parser.clone())
        .map(ShiftReduceExpressionState::new)
        .then(
          Self::binary_op_parser().padded_by(whitespace_parser())
            .then(unary_expr_parser(expr_parser))
            .repeated()
            .collect::<Vec<_>>()
        )
        .try_map(|(state, op_expr_vec), span| {
          let result = op_expr_vec.into_iter().fold(
            state,
            |mut state, (next_op, next_expr)| {
              state.push_op_expr(next_op, next_expr);
              state
            }
          ).finalize();
          result.map_err(|_err| E::Error::expected_found([], None, span)) 
        })
    }).boxed()
  }

  fn binary_op_parser<E>()
    -> impl 'a + Clone + Parser<'a, &'a str, ExpressionPrecedenceContext, E>
    where E: ParserExtra<'a, &'a str>,
  {
    use chumsky::prelude::*;

    choice((
      just("||").map(|_| ExpressionPrecedenceContext::Logical(LogicalExprOp::Or)),
      just("&&").map(|_| ExpressionPrecedenceContext::Logical(LogicalExprOp::And)),
      just("<<").map(|_| ExpressionPrecedenceContext::Shift(ShiftExprOp::Shl)),
      just(">>").map(|_| ExpressionPrecedenceContext::Shift(ShiftExprOp::Shr)),
      just("<=").map(|_| ExpressionPrecedenceContext::Relational(RelationalExprOp::LessThanOrEqual)),
      just(">=").map(|_| ExpressionPrecedenceContext::Relational(RelationalExprOp::GreaterThanOrEqual)),
      just("==").map(|_| ExpressionPrecedenceContext::Relational(RelationalExprOp::Equal)),
      just("!=").map(|_| ExpressionPrecedenceContext::Relational(RelationalExprOp::NotEqual)),
      just("<").map(|_| ExpressionPrecedenceContext::Relational(RelationalExprOp::LessThan)),
      just(">").map(|_| ExpressionPrecedenceContext::Relational(RelationalExprOp::GreaterThan)),
      just("+").map(|_| ExpressionPrecedenceContext::Add(AddExprOp::Add)),
      just("-").map(|_| ExpressionPrecedenceContext::Add(AddExprOp::Sub)),
      just("*").map(|_| ExpressionPrecedenceContext::Mul(MulExprOp::Mul)),
      just("/").map(|_| ExpressionPrecedenceContext::Mul(MulExprOp::Div)),
      just("%").map(|_| ExpressionPrecedenceContext::Mul(MulExprOp::Mod)),
      just("^").map(|_| ExpressionPrecedenceContext::Bit(BitExprOp::Xor)),
      just("|").map(|_| ExpressionPrecedenceContext::Bit(BitExprOp::Or)),
      just("&").map(|_| ExpressionPrecedenceContext::Bit(BitExprOp::And)),
    ))
  }
}

/**
 * The shift-reduce result.
 */
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShiftReduceResult {
  Reduce,
  Shift,
  Error(String)
}

/**
 * The precedence context for an expression.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpressionPrecedenceContext {
  Top,
  Logical(LogicalExprOp),
  Relational(RelationalExprOp),
  Shift(ShiftExprOp),
  Add(AddExprOp),
  Mul(MulExprOp),
  Bit(BitExprOp),
}
impl ExpressionPrecedenceContext {
  fn name(self) -> &'static str {
    match self {
      ExpressionPrecedenceContext::Top => "top",
      ExpressionPrecedenceContext::Logical(LogicalExprOp::And) => "logical and",
      ExpressionPrecedenceContext::Logical(LogicalExprOp::Or) => "logical or",
      ExpressionPrecedenceContext::Relational(_) => "relational",
      ExpressionPrecedenceContext::Shift(_) => "shift",
      ExpressionPrecedenceContext::Add(_) => "add",
      ExpressionPrecedenceContext::Mul(_) => "mul",
      ExpressionPrecedenceContext::Bit(_) => "bit",
    }
  }

  pub fn shift_reduce(self, op: ExpressionPrecedenceContext) -> ShiftReduceResult {
    let make_err_result = |next: ExpressionPrecedenceContext| {
      ShiftReduceResult::Error(format!(
        "Cannot next {} operator within a {} operator without parenthesis.",
        next.name(),
        self.name(),
      ))
    };

    // Returns Ok(true) if push is ok.
    // Returns Ok(false) if reduce is needed.
    // Returns Err(error) if the operation is invalid.
    match self {
      ExpressionPrecedenceContext::Top => ShiftReduceResult::Shift,
      // Logical ands only associate with themselves, and allow
      // implicit nesting of only relationals.
      ExpressionPrecedenceContext::Logical(LogicalExprOp::And) => match op {
        ExpressionPrecedenceContext::Top => make_err_result(op),

        ExpressionPrecedenceContext::Logical(LogicalExprOp::And) =>
          ShiftReduceResult::Reduce,
        ExpressionPrecedenceContext::Logical(LogicalExprOp::Or) =>
          make_err_result(op),

        ExpressionPrecedenceContext::Relational(_) => ShiftReduceResult::Shift,
        ExpressionPrecedenceContext::Shift(_) => make_err_result(op),
        ExpressionPrecedenceContext::Add(_) => make_err_result(op),
        ExpressionPrecedenceContext::Mul(_) => make_err_result(op),
        ExpressionPrecedenceContext::Bit(_) => make_err_result(op),
      },
      ExpressionPrecedenceContext::Logical(LogicalExprOp::Or) => match op {
        ExpressionPrecedenceContext::Top => make_err_result(op),
        ExpressionPrecedenceContext::Logical(LogicalExprOp::And) =>
          make_err_result(op),
        ExpressionPrecedenceContext::Logical(LogicalExprOp::Or) =>
          ShiftReduceResult::Shift,

        ExpressionPrecedenceContext::Relational(_) => ShiftReduceResult::Shift,
        ExpressionPrecedenceContext::Shift(_) => make_err_result(op),
        ExpressionPrecedenceContext::Add(_) => make_err_result(op),
        ExpressionPrecedenceContext::Mul(_) => make_err_result(op),
        ExpressionPrecedenceContext::Bit(_) => make_err_result(op),
      },
      ExpressionPrecedenceContext::Relational(_) => match op {
        ExpressionPrecedenceContext::Top => make_err_result(op),
        ExpressionPrecedenceContext::Logical(LogicalExprOp::And) =>
          ShiftReduceResult::Reduce,
        ExpressionPrecedenceContext::Logical(LogicalExprOp::Or) =>
          ShiftReduceResult::Reduce,

        ExpressionPrecedenceContext::Relational(_) => make_err_result(op),
        ExpressionPrecedenceContext::Shift(_) => ShiftReduceResult::Shift,
        ExpressionPrecedenceContext::Add(_) => ShiftReduceResult::Shift,
        ExpressionPrecedenceContext::Mul(_) => ShiftReduceResult::Shift,
        ExpressionPrecedenceContext::Bit(_) => ShiftReduceResult::Shift,
      },
      ExpressionPrecedenceContext::Shift(_) => match op {
        ExpressionPrecedenceContext::Top => make_err_result(op),
        ExpressionPrecedenceContext::Logical(LogicalExprOp::And) =>
          make_err_result(op),
        ExpressionPrecedenceContext::Logical(LogicalExprOp::Or) =>
          make_err_result(op),

        ExpressionPrecedenceContext::Relational(_) => ShiftReduceResult::Reduce,
        ExpressionPrecedenceContext::Shift(_) => make_err_result(op),
        ExpressionPrecedenceContext::Add(_) => make_err_result(op),
        ExpressionPrecedenceContext::Mul(_) => make_err_result(op),
        ExpressionPrecedenceContext::Bit(_) => ShiftReduceResult::Shift,
      },
      ExpressionPrecedenceContext::Add(_) => match op {
        ExpressionPrecedenceContext::Top => make_err_result(op),
        ExpressionPrecedenceContext::Logical(LogicalExprOp::And) =>
          make_err_result(op),
        ExpressionPrecedenceContext::Logical(LogicalExprOp::Or) =>
          make_err_result(op),

        ExpressionPrecedenceContext::Relational(_) => ShiftReduceResult::Reduce,
        ExpressionPrecedenceContext::Shift(_) => make_err_result(op),
        ExpressionPrecedenceContext::Add(_) => ShiftReduceResult::Reduce,
        ExpressionPrecedenceContext::Mul(_) => ShiftReduceResult::Shift,
        ExpressionPrecedenceContext::Bit(_) => make_err_result(op),
      },
      ExpressionPrecedenceContext::Mul(_) => match op {
        ExpressionPrecedenceContext::Top => make_err_result(op),
        ExpressionPrecedenceContext::Logical(LogicalExprOp::And) =>
          make_err_result(op),
        ExpressionPrecedenceContext::Logical(LogicalExprOp::Or) =>
          make_err_result(op),

        ExpressionPrecedenceContext::Relational(_) => ShiftReduceResult::Reduce,
        ExpressionPrecedenceContext::Shift(_) => make_err_result(op),
        ExpressionPrecedenceContext::Add(_) => ShiftReduceResult::Reduce,
        ExpressionPrecedenceContext::Mul(_) => make_err_result(op),
        ExpressionPrecedenceContext::Bit(_) => ShiftReduceResult::Shift,
      },
      ExpressionPrecedenceContext::Bit(_) => match op {
        ExpressionPrecedenceContext::Top => make_err_result(op),
        ExpressionPrecedenceContext::Logical(LogicalExprOp::And) =>
          make_err_result(op),
        ExpressionPrecedenceContext::Logical(LogicalExprOp::Or) =>
          make_err_result(op),

        ExpressionPrecedenceContext::Relational(_) => ShiftReduceResult::Reduce,
        ExpressionPrecedenceContext::Shift(_) => ShiftReduceResult::Reduce,
        ExpressionPrecedenceContext::Add(_) => make_err_result(op),
        ExpressionPrecedenceContext::Mul(_) => make_err_result(op),
        ExpressionPrecedenceContext::Bit(_) => ShiftReduceResult::Shift,
      },
    }
  }

  fn make_binary_expression<'a>(self, lhs: Expression<'a>, rhs: Expression<'a>)
    -> Expression<'a>
  {
    let lhs = lhs.boxed();
    let rhs = rhs.boxed();
    match self {
      ExpressionPrecedenceContext::Top => {
        panic!("Cannot make binary expression with top precedence context")
      },
      ExpressionPrecedenceContext::Logical(op) =>
        Expression::Logical(LogicalExpr { lhs, op, rhs }),
      ExpressionPrecedenceContext::Relational(op) =>
        Expression::Relational(RelationalExpr { lhs, op, rhs }),
      ExpressionPrecedenceContext::Shift(op) =>
        Expression::Shift(ShiftExpr { lhs, op, rhs }),
      ExpressionPrecedenceContext::Add(op) =>
        Expression::Add(AddExpr { lhs, op, rhs }),
      ExpressionPrecedenceContext::Mul(op) =>
        Expression::Mul(MulExpr { lhs, op, rhs }),
      ExpressionPrecedenceContext::Bit(op) =>
        Expression::Bit(BitExpr { lhs, op, rhs }),
    }
  }
}

/**
 * Shift-reduce parser state.
 */
struct ShiftReduceExpressionState<'a> {
  pub stack: Vec<Expression<'a>>,
  pub precs: Vec<ExpressionPrecedenceContext>,
  pub error: Option<String>,
}
impl<'a> ShiftReduceExpressionState<'a> {
  fn new(init_expr: Expression<'a>) -> Self {
    let stack = vec![init_expr];
    Self { stack, precs: Vec::new(), error: None }
  }

  fn current_prec(&self) -> ExpressionPrecedenceContext {
    *self.precs.last().unwrap_or(&ExpressionPrecedenceContext::Top)
  }

  fn push_op_expr(&mut self,
    op: ExpressionPrecedenceContext,
    expr: Expression<'a>
  ) {
    if self.error.is_some() {
      return;
    }

    loop {
      let current_prec = self.current_prec();
      match current_prec.shift_reduce(op) {
        ShiftReduceResult::Error(err) => {
          self.error = Some(err);
          return;
        },
        ShiftReduceResult::Shift => {
          self.precs.push(op);
          self.stack.push(expr);
          return;
        },
        ShiftReduceResult::Reduce => {
          self.reduce();
          continue;
        },
      }
    }
  }

  fn finalize(mut self) -> Result<Expression<'a>, String> {
    if self.error.is_some() {
      return Err(self.error.take().unwrap());
    }
    while self.stack.len() > 1 {
      self.reduce();
    }
    Ok(self.stack.pop().expect("Empty stack at finalize"))
  }

  fn reduce(&mut self) {
    let rhs = self.stack.pop().expect("Empty stack");
    let prec = self.precs.pop().expect("Empty precs");
    let lhs = self.stack.pop().expect("Empty stack");

    let new_expr = prec.make_binary_expression(lhs, rhs);
    self.stack.push(new_expr);
  }
}
