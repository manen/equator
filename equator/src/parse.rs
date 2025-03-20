use std::{borrow::Cow, collections::HashMap};

#[derive(Clone, Debug)]
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
}
impl<'a> Tokenizer<'a> {
	pub fn new(s: impl Into<Cow<'a, str>>) -> Self {
		Self {
			parser: baseparser::Parser::new(s),
			cmap: gen_charmap(),
		}
	}
}
impl<'a> Iterator for Tokenizer<'a> {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		None
	}
}
