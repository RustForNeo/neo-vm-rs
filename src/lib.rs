#![feature(associated_type_defaults)]
#![feature(linked_list_remove)]
#![feature(exclusive_range_pattern)]

extern crate core;

pub use num_bigint::BigInt;
mod exception;
mod script;
mod types;

mod vm;

pub use exception::*;
pub use script::*;
pub use types::*;
pub use vm::*;

pub fn add(left: usize, right: usize) -> usize {
	left + right
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		let result = add(2, 2);
		assert_eq!(result, 4);
	}
}
