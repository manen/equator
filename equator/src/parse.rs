use crate::{
	token::{self, Token},
	Error, Expr, Result,
};

pub fn parse(s: &str) -> Result<Expr> {
	let tokens = token::Tokenizer::new(s);
	let tokens = token::ParensParser::new(tokens);

	parse_tokens(tokens)
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
		// let parsed = match token {
		// 	Token::Add => {}
		// };
	}

	Ok(expr)
}
