#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
	#[error("failed to parse a number: {0}")]
	ParseFloatError(#[from] std::num::ParseFloatError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
