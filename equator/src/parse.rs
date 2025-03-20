use crate::{
	token::{self, Token},
	Expr, Result,
};

pub fn parse(s: &str) -> Result<Expr> {
	let tokens = token::Tokenizer::new(s);
	let tokens = token::ParensParser::new(tokens);
	let tokens = tokens.collect::<Vec<_>>();

	parse_tokens(&tokens)
}

/// input has to be `ParensParse`'d
pub fn parse_tokens(tokens: &[Token]) -> Result<Expr> {
	todo!()

	// we need to go from mindless token list
	// to an order of operationed expression

	// THERE"S ONLY 3 LEVELS
	// 1. +, -  -> we don't need nothing
	// 2. *, /  -> Token::LowOrderParens
	// 3. ()    -> Token::Parens

	// if there's only 1 level in a segment of the expression we don't need anything
	// if there's multiplication and/or division we can put it in a LowOrderParens
}
