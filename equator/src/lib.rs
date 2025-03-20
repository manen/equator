mod err;
pub use err::*;

pub mod parse;
pub mod token;

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
	Constant(f64),
	X,

	Add(Box<Expr>, Box<Expr>),
	Subtract(Box<Expr>, Box<Expr>),
	Multiply(Box<Expr>, Box<Expr>),
	Divide(Box<Expr>, Box<Expr>),
}
impl Expr {
	pub fn add(a: Expr, b: Expr) -> Self {
		Self::Add(Box::new(a), Box::new(b))
	}
	pub fn subtract(a: Expr, b: Expr) -> Self {
		Self::Subtract(Box::new(a), Box::new(b))
	}
	pub fn multiply(a: Expr, b: Expr) -> Self {
		Self::Multiply(Box::new(a), Box::new(b))
	}
	pub fn divide(a: Expr, b: Expr) -> Self {
		Self::Divide(Box::new(a), Box::new(b))
	}
}

// how do we describe an equation

// expr = expr = expr
