mod err;
pub use err::*;

pub mod parse;

pub enum Expr {
	Constant(f64),
	X,

	Add(Box<Expr>, Box<Expr>),
	Subtract(Box<Expr>, Box<Expr>),
	Multiply(Box<Expr>, Box<Expr>),
	Divide(Box<Expr>, Box<Expr>),
}

// how do we describe an equation

// expr = expr = expr
