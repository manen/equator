use std::{borrow::Cow, collections::HashMap};

use crate::Error;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
	Constant(f64),
	X,

	Add,
	Subtract,
	Multiply,
	Divide,

	ParensOpen,
	ParensClose,
	Parens(Vec<Token>),
	LowOrderParens(Vec<Token>),

	Error(Error),
}

type Charmap = HashMap<char, Token>;
fn gen_charmap() -> Charmap {
	[
		('+', Token::Add),
		('-', Token::Subtract),
		('*', Token::Multiply),
		('/', Token::Divide),
		('x', Token::X),
		('(', Token::ParensOpen),
		(')', Token::ParensClose),
	]
	.into_iter()
	.collect()
}

#[derive(Clone, Debug)]
pub struct Tokenizer<'a> {
	parser: baseparser::Parser<'a>,
	cmap: Charmap, // <- this is supposed to be gen_charmap()

	remaining: Option<Token>,
}
impl<'a> Tokenizer<'a> {
	pub fn new(s: impl Into<Cow<'a, str>>) -> Self {
		Self {
			parser: baseparser::Parser::new(s),
			cmap: gen_charmap(),
			remaining: None,
		}
	}
}
impl<'a> Iterator for Tokenizer<'a> {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		self.remaining.take().map(|a| Some(a)).unwrap_or_else(|| {
			let (splitter, read) = self.parser.read_until_any(self.cmap.keys().cloned());
			let splitter = splitter.map(|c| self.cmap.get(&c).cloned()).flatten();

			let read = read.trim();

			if read.len() > 0 {
				self.remaining = splitter;

				let f: Result<f64, _> = read.parse();

				match f {
					Ok(a) => Some(Token::Constant(a)),
					Err(err) => Some(Token::Error(err.into())),
				}
			} else {
				splitter
			}
		})
	}
}

#[derive(Clone, Debug)]
pub struct ParensParser<I: Iterator<Item = Token>> {
	iter: I,
}
impl<I: Iterator<Item = Token>> ParensParser<I> {
	pub fn new(iter: I) -> Self {
		Self { iter }
	}
}
impl<I: Iterator<Item = Token>> Iterator for ParensParser<I> {
	type Item = Token;

	fn next(&mut self) -> Option<Token> {
		match self.iter.next() {
			None => None,
			Some(Token::ParensOpen) => {
				let inside = {
					let iter = (&mut self.iter).take_while(|a| a != &Token::ParensClose);
					iter.collect::<Vec<_>>()
				};
				Some(Token::Parens(inside))
			}
			Some(token) => Some(token),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_tokenizer() {
		let mut tokenizer = Tokenizer::new("6 * 4 * x / 6 ( 4 + 6 )");

		assert_eq!(tokenizer.next(), Some(Token::Constant(6.0)));
		assert_eq!(tokenizer.next(), Some(Token::Multiply));
		assert_eq!(tokenizer.next(), Some(Token::Constant(4.0)));
		assert_eq!(tokenizer.next(), Some(Token::Multiply));
		assert_eq!(tokenizer.next(), Some(Token::X));
		assert_eq!(tokenizer.next(), Some(Token::Divide));
		assert_eq!(tokenizer.next(), Some(Token::Constant(6.0)));
		assert_eq!(tokenizer.next(), Some(Token::ParensOpen));
		assert_eq!(tokenizer.next(), Some(Token::Constant(4.0)));
		assert_eq!(tokenizer.next(), Some(Token::Add));
		assert_eq!(tokenizer.next(), Some(Token::Constant(6.0)));
		assert_eq!(tokenizer.next(), Some(Token::ParensClose));
		assert_eq!(tokenizer.next(), None);
	}

	#[test]
	fn test_parens_parser() {
		let tokenizer = Tokenizer::new("6 * 4 * x / 6 ( 4 + 6 )");
		let mut tokenizer = ParensParser::new(tokenizer);

		assert_eq!(tokenizer.next(), Some(Token::Constant(6.0)));
		assert_eq!(tokenizer.next(), Some(Token::Multiply));
		assert_eq!(tokenizer.next(), Some(Token::Constant(4.0)));
		assert_eq!(tokenizer.next(), Some(Token::Multiply));
		assert_eq!(tokenizer.next(), Some(Token::X));
		assert_eq!(tokenizer.next(), Some(Token::Divide));
		assert_eq!(tokenizer.next(), Some(Token::Constant(6.0)));
		assert_eq!(
			tokenizer.next(),
			Some(Token::Parens(vec![
				Token::Constant(4.0),
				Token::Add,
				Token::Constant(6.0),
			]))
		);
		assert_eq!(tokenizer.next(), None);
	}
}
