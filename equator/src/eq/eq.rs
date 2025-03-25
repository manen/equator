use std::str::FromStr;

use crate::{parse::parse, Error};

use super::Expr;

#[derive(Clone, Debug)]
pub struct Equation {
	#[allow(dead_code)]
	exprs: [Expr; 2],
}
impl FromStr for Equation {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut iter = s.split('=').map(parse);

		let first: Expr = iter.next().ok_or(Error::EOF {
			comment: Some("expected 2 expressions in equation, got 0"),
		})??;
		let second: Expr = iter.next().ok_or(Error::EOF {
			comment: Some("expected 2 expressions in equation, got 1"),
		})??;

		let too_many = iter.next().is_some();
		if too_many {
			return Err(Error::TooManyExpressions { expected: 2 });
		}

		Ok(Self {
			exprs: [first, second],
		})
	}
}
