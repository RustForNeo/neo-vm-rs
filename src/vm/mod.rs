pub mod instruction;
pub mod op_code;

pub mod script;

pub mod evaluation_stack;

pub mod exception_handling_context;
pub mod exception_handling_state;

pub mod execution_context;

pub mod slot;

pub mod vm_state;
mod script_builder;
mod execution_engine;
mod vm_exception;

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
