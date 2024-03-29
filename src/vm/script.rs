use crate::{instruction::Instruction, op_code::OpCode, stack_item_type::StackItemType};
use std::{collections::HashMap, convert::TryFrom, ops::Index};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Script {
	value: Vec<u8>,
	strict_mode: bool,
	instructions: HashMap<usize, Instruction>,
}

impl Script {
	pub fn len(&self) -> usize {
		self.value.len()
	}

	pub fn get(&self, index: usize) -> OpCode {
		OpCode::try_from(self.value[index]).unwrap()
	}

	pub fn new(bytes: Vec<u8>, strict_mode: bool) -> Result<Self, ScriptError> {
		let mut script = Self { value: bytes, strict_mode, instructions: HashMap::new() };

		if strict_mode {
			script.validate()?;
		}

		Ok(script)
	}

	pub fn validate(&mut self) -> Result<(), ScriptError> {
		let mut ip = 0;
		while ip < self.len() {
			let instruction = self.get_instruction(ip).unwrap();
			ip += instruction.size();
		}

		for (ip, instruction) in &self.instructions {
			match instruction.opcode {
				OpCode::Jmp
				| OpCode::JmpIf
				| OpCode::JmpIfNot
				| OpCode::JmpEq
				| OpCode::JmpNe
				| OpCode::JmpGt
				| OpCode::JmpGe
				| OpCode::JmpLt
				| OpCode::JmpLe
				| OpCode::Call
				| OpCode::EndTry =>
					if !self.instructions.contains_key(&(ip + instruction.token_i8())) {
						panic!("ip: {}, opcode: {:?}", ip, instruction.opcode);
					},
				OpCode::PushA
				| OpCode::JmpL
				| OpCode::JmpIfL
				| OpCode::JmpIfNotL
				| OpCode::JmpEqL
				| OpCode::JmpNeL
				| OpCode::JmpGtL
				| OpCode::JmpGeL
				| OpCode::JmpLtL
				| OpCode::JmpLeL
				| OpCode::CallL
				| OpCode::EndTryL =>
					if !self.instructions.contains_key(&(ip + instruction.token_i32())) {
						panic!("ip: {}, opcode: {:?}", ip, instruction.opcode);
					},
				OpCode::Try => {
					if !self.instructions.contains_key(&(ip + instruction.token_i8())) {
						panic!("ip: {}, opcode: {:?}", ip, instruction.opcode);
					}
					if !self.instructions.contains_key(&(ip + instruction.token_i8_1())) {
						panic!("ip: {}, opcode: {:?}", ip, instruction.opcode);
					}
				},
				OpCode::TryL => {
					if !self.instructions.contains_key(&(ip + instruction.token_i32())) {
						panic!("ip: {}, opcode: {:?}", ip, instruction.opcode);
					}
					if !self.instructions.contains_key(&(ip + instruction.token_i32_1())) {
						panic!("ip: {}, opcode: {:?}", ip, instruction.opcode);
					}
				},
				OpCode::NewArrayT | OpCode::IsType | OpCode::Convert => {
					let type_code = instruction.token_u8();
					if !StackItemType::is_valid(type_code) {
						panic!("Invalid type code: {}", type_code);
					}
					if instruction.opcode != OpCode::NewArrayT
						&& type_code == StackItemType::Any as u8
					{
						panic!("ip: {}, opcode: {:?} with Any type", ip, instruction.opcode);
					}
				},
				_ => {},
			}
		}

		Ok(())
	}

	pub fn get_instruction(&mut self, ip: usize) -> Result<&Instruction, ScriptError> {
		if !self.instructions.contains_key(&ip) {
			if self.strict_mode {
				return Err(ScriptError::InvalidInstrPointer(ip))
			}

			let instr = Instruction::parse(&self.value, ip)?;
			self.instructions.insert(ip, instr);
		}

		Ok(self.instructions.get(&ip).unwrap())
	}
}

impl Index<usize> for Script {
	type Output = OpCode;

	fn index(&self, index: usize) -> &Self::Output {
		&self.get(index)
	}
}

impl TryFrom<Vec<u8>> for Script {
	type Error = ScriptError;

	fn try_from(script: Vec<u8>) -> Result<Self, Self::Error> {
		Self::new(script, false)
	}
}

enum ScriptError {
	InvalidInstrPointer(usize),
	// other errors
}
