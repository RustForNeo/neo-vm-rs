pub mod compound_type;
pub mod execution_engine_limits;
pub mod interop_interface;
pub mod reference_counter;
pub mod stack_item;
pub mod stack_item_type;
pub mod tarjan;

pub mod primitive_type;

pub mod boolean;
pub mod buffer;
pub mod byte_string;
pub mod integer;

pub mod array;

pub mod null;

pub mod map;

pub mod pointer;

pub mod Struct;

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
