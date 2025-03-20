use crate::{
	token::{self, Token},
	Error, Expr, Result,
};

pub fn parse(s: &str) -> Result<Expr> {
	let tokens = token::Tokenizer::new(s);
	let tokens = token::ParensParser::new(tokens);

	parse_tokens(tokens.map(|a| {
		println!("{a:#?}");
		a
	}))
}

/// input has to be `ParensParse`'d
pub fn parse_tokens<I: Iterator<Item = Token>>(mut iter: I) -> Result<Expr> {
	let to_expr = |token: Option<Token>| match token {
		Some(Token::Constant(num)) => Ok(Expr::Constant(num)),
		Some(Token::X) => Ok(Expr::X),
		Some(Token::Parens(tokens)) => parse_tokens(tokens.into_iter()),
		Some(token) => Err(Error::UnexpectedToken {
			expected: "number, operations in parenthesis, or x",
			got: Box::new(token),
		}),
		None => Err(Error::EOF),
	};

	let mut expr: Expr = to_expr(iter.next())?;

	loop {
		let token = iter.next();
		let token = match token {
			Some(a) => a,
			None => break,
		};
		match token {
			Token::Add => {
				let next = to_expr(iter.next())?;
				expr = Expr::add(expr, next);
			}
			Token::Subtract => {
				let next = to_expr(iter.next())?;
				expr = Expr::subtract(expr, next);
			}
			Token::Multiply => {
				let next = to_expr(iter.next())?;
				expr = Expr::multiply(expr, next);
			}
			Token::Divide => {
				let next = to_expr(iter.next())?;
				expr = Expr::divide(expr, next);
			}
			Token::X => expr = Expr::multiply(expr, Expr::X),
			_ => {
				return Err(Error::UnexpectedToken {
					expected: "+, -, *, /, or x",
					got: Box::new(token),
				})
			}
		};
	}

	Ok(expr)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parser_1() {
		let s = "4*5";
		let expr = parse(s).expect("failed to parse");

		assert_eq!(
			expr,
			Expr::multiply(Expr::Constant(4.0), Expr::Constant(5.0))
		);
	}
	#[test]
	fn test_parser_3() {
		let s = "4*5x";
		let expr = parse(s).expect("failed to parse");
		let expr = expr.simplify();

		assert_eq!(expr, Expr::multiply(Expr::Constant(20.0), Expr::X));
	}
	#[test]
	fn test_parser_2() {
		let s = "6 + 5 + 3";
		let expr = parse(s).expect("failed to parse");

		assert_eq!(
			expr,
			Expr::add(
				Expr::add(Expr::Constant(6.0), Expr::Constant(5.0)),
				Expr::Constant(3.0)
			)
		)
	}
}
