use std::borrow::Cow;

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

		'outer: loop {
			let c = match self.s.chars().nth(self.i) {
				Some(c) => c,
				None => break,
			};

			for tc in chars.clone() {
				if c == tc {
					matched = Some(c);
					end_i = self.i;
					self.i += 1;
					break 'outer;
				}
			}
			self.i += 1;
		}

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
		let read = parser.read_until_any(chars.clone());
		assert_eq!(read, (None, ""));
	}
}
