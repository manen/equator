use crate::Expr;

#[derive(Clone, PartialEq, Debug)]
pub enum SolveExpr {
	Constant(f64),
	X,
	OneOver(Box<SolveExpr>),

	/// add or subtract
	Add(Box<SolveExpr>, Box<SolveExpr>),
	Multiply(Box<SolveExpr>, Box<SolveExpr>),
}
impl SolveExpr {
	/// return -self
	pub fn flip(&mut self) {
		match self {
			Self::Constant(num) => *num *= -1.0,
			Self::X => *self = Self::Multiply(Box::new(Self::X), Box::new(Self::Constant(-1.0))),
			Self::OneOver(expr) => expr.flip(),
			Self::Add(..) => {
				let s = std::mem::replace(self, Self::X);
				*self = Self::Multiply(Box::new(s), Box::new(Self::Constant(-1.0)))
			}
			Self::Multiply(a, b) => {
				if a.flip_cost() <= b.flip_cost() {
					a.flip();
				} else {
					b.flip();
				}
			}
		}
	}
	pub fn flip_cost(&self) -> i32 {
		match self {
			&Self::Constant(num) if num <= 0.0 => -10,
			&Self::Constant(_) => 10,
			Self::X => 20,
			Self::OneOver(expr) => expr.flip_cost(),
			Self::Add(..) => 30,
			Self::Multiply(a, b) => a.flip_cost().min(b.flip_cost()),
		}
	}

	pub fn simplify(&mut self) {
		let mut swap_with: Option<Self> = None;

		match self {
			Self::Constant(..) | Self::X => {}
			Self::OneOver(expr) => expr.simplify(),
			Self::Add(a, b) => {
				a.simplify();
				b.simplify();
				match (a.as_mut(), b.as_mut()) {
					(Self::Constant(0.0), other) | (other, Self::Constant(0.0)) => {
						swap_with = Some(std::mem::replace(other, Self::X));
					}
					(av, bv) if av == bv => {
						*self = Self::Multiply(a.clone(), Box::new(Self::Constant(2.0)))
					}
					_ => {}
				}
			}
			Self::Multiply(a, b) => {
				a.simplify();
				b.simplify();
				match (a.as_mut(), b.as_mut()) {
					(Self::Constant(1.0), other) | (other, Self::Constant(1.0)) => {
						swap_with = Some(std::mem::replace(other, Self::X));
					}
					(a, b) if a.flip_cost() + b.flip_cost() < 0 => {
						a.flip();
						b.flip();
					}
					_ => {}
				}
			}
		}
		if let Some(other) = swap_with {
			*self = other;
		}
	}
}
impl From<Expr> for SolveExpr {
	fn from(value: Expr) -> Self {
		match value {
			Expr::Constant(num) => Self::Constant(num),
			Expr::X => Self::X,
			Expr::Add(a, b) => Self::Add(Box::new((*a).into()), Box::new((*b).into())),
			Expr::Subtract(a, b) => {
				let a: Self = (*a).into();
				let mut b: Self = (*b).into();
				b.flip();

				Self::Add(Box::new(a), Box::new(b))
			}
			Expr::Multiply(a, b) => {
				let a = (*a).into();
				let b = (*b).into();

				Self::Multiply(Box::new(a), Box::new(b))
			}
			Expr::Divide(a, b) => {
				let a = (*a).into();
				let b = (*b).into();
				let b = Self::OneOver(Box::new(b));

				Self::Multiply(Box::new(a), Box::new(b))
			}
		}
	}
}
