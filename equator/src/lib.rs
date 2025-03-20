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

	pub fn calculate(&self) -> Option<f64> {
		match self {
			&Self::Constant(num) => Some(num),
			Self::Add(a, b) => a
				.calculate()
				.map(|a| b.calculate().map(|b| a + b))
				.flatten(),
			Self::Subtract(a, b) => a
				.calculate()
				.map(|a| b.calculate().map(|b| a - b))
				.flatten(),
			Self::Multiply(a, b) => a
				.calculate()
				.map(|a| b.calculate().map(|b| a * b))
				.flatten(),
			Self::Divide(a, b) => a
				.calculate()
				.map(|a| b.calculate().map(|b| a / b))
				.flatten(),
			_ => None,
		}
	}

	pub fn simplify(self) -> Self {
		self.calculate()
			.map(|num| Self::Constant(num))
			.unwrap_or_else(|| match self {
				Self::Add(a, b) => Self::add(a.simplify(), b.simplify()),
				Self::Subtract(a, b) => Self::subtract(a.simplify(), b.simplify()),
				Self::Multiply(a, b) => Self::multiply(a.simplify(), b.simplify()),
				Self::Divide(a, b) => Self::divide(a.simplify(), b.simplify()),
				_ => self,
			})
	}
}

// how do we describe an equation

// expr = expr = expr

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_simplify_1() {
		let expr = "4 * 3";
		let expr = parse::parse(expr).expect("failed to parse");
		let expr = expr.simplify();
		if let Expr::Constant(num) = expr {
			assert_eq!(num, 12.0);
		} else {
			panic!("should've been simplified");
		}
	}
	#[test]
	fn test_simplify_2() {
		let expr = "4 * 3 / 2 * 10";
		let expr = parse::parse(expr).expect("failed to parse");
		let expr = expr.simplify();
		if let Expr::Constant(num) = expr {
			assert_eq!(num, 60.0);
		} else {
			panic!("should've been simplified");
		}
	}
	#[test]
	fn test_simplify_3() {
		let expr = "4 * 3 / 2 * 10 * x";
		let expr = parse::parse(expr).expect("failed to parse");
		let expr = expr.simplify();
		if let Expr::Constant(num) = expr {
			assert_eq!(num, 60.0);
		} else {
			panic!("should've been simplified {expr:#?}");
		}
	}
}
