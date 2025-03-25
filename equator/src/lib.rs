mod err;
pub use err::*;

pub mod parse;
pub mod solve;
pub mod token;

pub mod eq;
pub use eq::{Equation, Expr};
