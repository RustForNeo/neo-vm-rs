use crate::vm_state::VMState;
use std::{
	error::Error,
	fmt,
	fmt::{Display, Formatter},
};

/// Represents errors during VM execution.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VMException {
	/// Trying to exceed invocation stack size limit.
	InvocationStackOverflow(String),

	/// Trying to exceed try nesting limit.
	TryNestingOverflow(String),

	/// Trying to exceed maximum stack size.
	StackOverflow(String),

	/// Trying to create a single item that exceeds size limit.
	ItemTooLarge(String),

	/// Invalid opcode encountered.
	InvalidOpcode(String),

	/// Trying to divide by zero.
	DivisionByZero(String),

	/// Invalid jump offset or pointer.
	InvalidJump(String),

	/// Unsupported token encountered.
	InvalidToken(String),

	/// Invalid parameter for operation.
	InvalidParameter(String),

	/// Item not found in collection.
	ItemNotFound(String),

	/// Type mismatch for operation.
	InvalidType(String),

	/// Custom error with message.
	Custom(String),
}

impl Display for VMException {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		todo!()
	}
}

impl Error for VMException {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		None
	}
}

// impl fmt::Display for VMException {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Self::InvocationStackOverflow => {
//                 write!(f, "invocation stack size limit exceeded")
//             }
//             Self::TryNestingOverflow => {
//                 write!(f, "try nesting depth limit exceeded")
//             }
//             Self::StackOverflow => {
//                 write!(f, "stack size limit exceeded")
//             }
//             Self::ItemTooLarge => {
//                 write!(f, "item size exceeds limit")
//             }
//             Self::InvalidOpcode => {
//                 write!(f, "encountered invalid opcode")
//             }
//             Self::DivisionByZero => {
//                 write!(f, "tried to divide by zero")
//             }
//             Self::InvalidJump => {
//                 write!(f, "invalid jump offset or pointer")
//             }
//             Self::InvalidToken => {
//                 write!(f, "invalid token encountered")
//             }
//             Self::InvalidParameter => {
//                 write!(f, "invalid parameter for operation")
//             }
//             Self::ItemNotFound => {
//                 write!(f, "item not found in collection")
//             }
//             Self::InvalidType => {
//                 write!(f, "type mismatch for operation")
//             }
//             Self::Custom(msg) => {
//                 write!(f, "custom VM error: {}", msg)
//             }
//         }
//     }
// }
