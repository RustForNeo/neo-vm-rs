pub mod instruction;
pub mod op_code;

pub mod script;

pub mod evaluation_stack;

pub mod execution_context;

pub mod slot;

mod execution_engine;
mod vm_exception;
pub mod vm_state;

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
