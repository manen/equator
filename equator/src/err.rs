#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum Error {
	#[error("failed to parse a number: {0}")]
	ParseFloatError(#[from] std::num::ParseFloatError),
	#[error("there was nothing to parse")]
	NothingToParse,
	#[error("unexpected end of tokens")]
	EOF,
	#[error("unexpected token\nexpected: {expected}\ngot: {got:#?}")]
	UnexpectedToken {
		expected: &'static str,
		got: Box<crate::token::Token>,
	},
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
