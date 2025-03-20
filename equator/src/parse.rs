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
pub struct ExprParser<'a> {
	s: Cow<'a, str>,
	cmap: Charmap, // <- this is supposed to be gen_charmap()
	i: usize,
}
impl<'a> ExprParser<'a> {
	pub fn new(s: impl Into<Cow<'a, str>>) -> Self {
		Self {
			s: s.into(),
			cmap: gen_charmap(),
			i: 0,
		}
	}
}
impl<'a> Iterator for ExprParser<'a> {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		None
	}
}

#[derive(Clone, Debug)]
pub struct Parser<'a> {
	s: Cow<'a, str>,
	i: usize,
}
impl<'a> Parser<'a> {
	pub fn new(s: impl Into<Cow<'a, str>>) -> Self {
		Self { s: s.into(), i: 0 }
	}

	pub fn read_until_any<I: Iterator<Item = char> + Clone>(
		&mut self,
		chars: I,
	) -> (Option<char>, &str) {
		let start_i = self.i;
		let mut end_i = self.s.len();

		let mut matched = None;

		'outer: for i in self.i..self.s.len() {
			self.i = i;
			let chars = chars.clone();

			let c = match self.s.chars().nth(i) {
				Some(c) => c,
				None => break,
			};

			for tc in chars {
				if c == tc {
					matched = Some(c);
					end_i = self.i;
					self.i += 1;
					break 'outer;
				}
			}
		}

		println!("slice between ({start_i}-{end_i})");
		(matched, &self.s.as_ref()[start_i..end_i])
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parser() {
		let mut parser = Parser::new("hi*weather manxi'm whatever man*hello");
		let chars = ['x', '*'].iter().copied();

		let read = parser.read_until_any(chars.clone());
		assert_eq!(read, (Some('*'), "hi"));
		let read = parser.read_until_any(chars.clone());
		assert_eq!(read, (Some('x'), "weather man"));
		let read = parser.read_until_any(chars.clone());
		assert_eq!(read, (Some('*'), "i'm whatever man"));
		let read = parser.read_until_any(chars.clone());
		assert_eq!(read, (None, "hello"));
	}
}
