use crate::{
	array::Array,
	buffer::Buffer,
	byte_string::ByteString,
	compound_type::CompoundType,
	evaluation_stack::EvaluationStack,
	exception::{
		exception_handling_context::ExceptionHandlingContext,
		exception_handling_state::ExceptionHandlingState,
	},
	execution_context::{ExecutionContext, SharedStates},
	execution_engine_limits::ExecutionEngineLimits,
	instruction::Instruction,
	map::Map,
	null::Null,
	op_code::OpCode,
	pointer::Pointer,
	primitive_type::{PrimitiveType, PrimitiveTypeTrait},
	reference_counter::ReferenceCounter,
	slot::Slot,
	stack_item::{
		StackItem,
		StackItem::{VMArray, VMInteger},
	},
	stack_item_type::StackItemType,
	vm::{script::Script, vm_exception::VMException},
	vm_state::VMState,
	Struct::Struct,
};
use num_bigint::{BigInt, Sign};
use num_traits::{Signed, ToPrimitive, Zero};
use std::{
	cell::{Ref, RefCell},
	convert::TryInto,
	fmt::Error,
	ops::Neg,
	rc::Rc,
};

/// Represents the VM used to execute the script.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ExecutionEngine<'a> {
	/// Restrictions on the VM.
	pub limits: ExecutionEngineLimits,

	/// Used for reference counting of objects in the VM.
	pub reference_counter: Rc<RefCell<ReferenceCounter<'a>>>,

	/// The invocation stack of the VM.
	pub invocation_stack: Vec<Rc<RefCell<ExecutionContext<'a>>>>,

	/// The top frame of the invocation stack.
	pub current_context: Option<Rc<RefCell<ExecutionContext<'a>>>>,

	/// The bottom frame of the invocation stack.
	pub entry_context: Option<Rc<RefCell<ExecutionContext<'a>>>>,

	/// The stack to store the return values.
	pub result_stack: Rc<RefCell<EvaluationStack<'a>>>,

	/// The VM object representing the uncaught exception.
	pub uncaught_exception: Option<StackItem<'a>>,

	/// The current state of the VM.
	pub state: VMState,

	pub is_jumping: bool,
}

/// Interface implemented by objects that can be reference counted.
pub trait ReferenceCounted {
	/// Returns a unique ID for the object.
	fn id(&self) -> usize;

	/// Free any resources used by the object.
	fn free(&mut self);
}

impl<T> ReferenceCounted for T
where
	T: Sized + PartialEq,
{
	fn id(&self) -> usize {
		self as *const T as usize
	}

	fn free(&mut self) {}
}

impl ExecutionEngine {
	/// Constructs a new VM engine with default options.
	pub fn new() -> Self {
		Self::with_options(ExecutionEngineLimits::default())
	}

	/// Constructs a VM engine with the given options.
	pub fn with_options(limits: ExecutionEngineLimits) -> Self {
		Self {
			limits,
			reference_counter: Ref::new(RefCell::new(ReferenceCounter::new())),
			invocation_stack: Vec::new(),
			current_context: None,
			entry_context: None,
			result_stack: Ref::new(RefCell::new(EvaluationStack::new(Rc::new(RefCell::new(
				ReferenceCounter::new(),
			))))),
			uncaught_exception: None,
			state: VMState::Break,
			is_jumping: false,
		}
	}

	/// Starts executing the loaded script.
	pub fn execute(&mut self) -> VMState {
		if self.state == VMState::Break {
			self.state = VMState::None;
		}

		while self.state != VMState::Halt && self.state != VMState::Fault {
			self.execute_next();
		}

		self.state
	}

	/// Steps through executing a single instr.
	///
	fn execute_next(&mut self) {
		if self.invocation_stack.is_empty() {
			self.state = VMState::Halt;
		} else {
			let context = self.current_context.as_ref().unwrap().borrow();

			let instruction = context.current_instruction.unwrap_or(Instruction::RET);

			self.pre_execute_instruction(instruction);

			match self.execute_instruction(instruction) {
				Ok(_) => (),
				Err(e) => Err(VMException::InvalidOpcode("{e}".parse().unwrap())), // self.on_fault(e),
			}

			self.post_execute_instruction(instruction);
			if !self.is_jumping {
				self.current_context.unwrap().move_next();
			}

			self.is_jumping = false;
		}
	}

	fn pop(&mut self) -> StackItem {
		self.current_context.unwrap().shared_states.evaluation_stack.pop().unwrap()
		// panic!("Not implemented")
	}

	fn push(&mut self, item: StackItem) {
		self.current_context.unwrap().shared_states.evaluation_stack.push(item);
		// panic!("Not implemented")
	}

	fn peek(&self, index: usize) -> &StackItem {
		self.current_context
			.unwrap()
			.borrow()
			.shared_states
			.evaluation_stack
			.peek(index as i64)
			.unwrap()
	}

	fn execute_instr(&mut self, instr: Instruction) -> Result<VMState, VMException> {
		match instr.opcode {
			//Push
			OpCode::PushInt8
			| OpCode::PushInt16
			| OpCode::PushInt32
			| OpCode::PushInt64
			| OpCode::PushInt128
			| OpCode::PushInt256 => self.push(StackItem::from(VMInteger::from(instr.operand))),
			OpCode::PushTrue => self.push(StackItem::from(true)),
			OpCode::PushFalse => self.push(StackItem::from(false)),
			OpCode::PushA => {
				let position = (self.current_context.unwrap().instruction_pointer as i32)
					.checked_add(instr.token_i32())
					.unwrap();
				if position < 0
					|| position > self.current_context.unwrap().shared_states.script.len() as i32
				{
					// return Err(VMException::InvalidOpcode("Bad pointer address: {position}");
					return Err(VMException::new(Error::new("Bad pointer address")))
				}

				self.push(StackItem::VMPointer(Pointer::new(
					self.current_context.unwrap().shared_states.script,
					position as usize,
				)))
			},
			OpCode::PushNull => self.push(StackItem::VMNull(Null::default())),
			OpCode::PushData1 | OpCode::PushData2 | OpCode::PushData4 => {
				self.limits.assert_max_item_size(instr.operand.len() as u32);
				self.push(StackItem::from(instr.operand))
			},
			OpCode::PushM1
			| OpCode::Push0
			| OpCode::Push1
			| OpCode::Push2
			| OpCode::Push3
			| OpCode::Push4
			| OpCode::Push5
			| OpCode::Push6
			| OpCode::Push7
			| OpCode::Push8
			| OpCode::Push9
			| OpCode::Push10
			| OpCode::Push11
			| OpCode::Push12
			| OpCode::Push13
			| OpCode::Push14
			| OpCode::Push15
			| OpCode::Push16 => self.push(StackItem::VMInteger(instr.opcode - OpCode::Push0)),

			// Control
			OpCode::Nop => Ok(VMState::None),
			OpCode::Jmp => self.execute_jump_offset(instr.token_i8() as i32),
			OpCode::JmpL => self.execute_jump_offset(instr.token_i32()),
			OpCode::JmpIf =>
				if self.pop().get_bool() {
					self.execute_jump_offset(instr.token_i8() as i32)
				},
			OpCode::JmpIfL =>
				if self.pop().get_bool() {
					self.execute_jump_offset(instr.token_i32())
				},
			OpCode::JmpIfNot =>
				if !self.pop().get_bool() {
					self.execute_jump_offset(instr.token_i8() as i32)
				},
			OpCode::JmpIfNotL =>
				if !self.pop().get_bool() {
					self.execute_jump_offset(instr.token_i32())
				},
			OpCode::JmpEq => {
				let x2 = self.pop().get_integer();
				let x1 = self.pop().get_integer();
				if x1 == x2 {
					self.execute_jump_offset(instr.token_i8() as i32)
				}
			},
			OpCode::JmpEqL => {
				let x2 = self.pop().get_integer();
				let x1 = self.pop().get_integer();
				if x1 == x2 {
					self.execute_jump_offset(instr.token_i32())
				}
			},
			OpCode::JmpNe => {
				let x2 = self.pop().get_integer();
				let x1 = self.pop().get_integer();
				if x1 != x2 {
					self.execute_jump_offset(instr.token_i8() as i32)
				}
			},
			OpCode::JmpNeL => {
				let x2 = self.pop().get_integer();
				let x1 = self.pop().get_integer();
				if x1 != x2 {
					self.execute_jump_offset(instr.token_i32())
				}
			},
			OpCode::JmpGt => {
				let x2 = self.pop().get_integer();
				let x1 = self.pop().get_integer();
				if x1 > x2 {
					self.execute_jump_offset(instr.token_i8() as i32)
				}
			},
			OpCode::JmpGtL => {
				let x2 = self.pop().get_integer();
				let x1 = self.pop().get_integer();
				if x1 > x2 {
					self.execute_jump_offset(instr.token_i32())
				}
			},
			OpCode::JmpGe => {
				let x2 = self.pop().get_integer();
				let x1 = self.pop().get_integer();
				if x1 >= x2 {
					self.execute_jump_offset(instr.token_i8() as i32)
				}
			},
			OpCode::JmpGeL => {
				let x2 = self.pop().get_integer();
				let x1 = self.pop().get_integer();
				if x1 >= x2 {
					self.execute_jump_offset(instr.token_i32())
				}
			},
			OpCode::JmpLt => {
				let x2 = self.pop().get_integer();
				let x1 = self.pop().get_integer();
				if x1 < x2 {
					self.execute_jump_offset(instr.token_i8() as i32)
				}
			},
			OpCode::JmpLtL => {
				let x2 = self.pop().get_integer();
				let x1 = self.pop().get_integer();
				if x1 < x2 {
					self.execute_jump_offset(instr.token_i32())
				}
			},
			OpCode::JmpLe => {
				let x2 = self.pop().get_integer();
				let x1 = self.pop().get_integer();
				if x1 <= x2 {
					self.execute_jump_offset(instr.token_i8() as i32)
				}
			},
			OpCode::JmpLeL => {
				let x2 = self.pop().get_integer();
				let x1 = self.pop().get_integer();
				if x1 <= x2 {
					self.execute_jump_offset(instr.token_i32())
				}
			},
			OpCode::Call => self.execute_call(
				(self.current_context.unwrap().instruction_pointer + instr.token_i8()) as i32,
			),
			OpCode::CallL => self
				.execute_call(self.current_context.unwrap().InstructionPointer + instr.token_i32()),
			OpCode::CallA => {
				let x: Pointer = self.pop().into();
				if x.Script != self.current_context.unwrap().Script {
					return Err(VMException::InvalidOpcode(
						"Pointers can't be shared between scripts".parse().unwrap(),
					))
				}
				self.execute_call(x.Position)
			},
			OpCode::CallT => self.load_token(instr.token_u16()),
			OpCode::Abort =>
				Err(VMException::InvalidOpcode("{OpCode::ABORT} is executed.".parse().unwrap())),
			OpCode::Assert => {
				let x = self.pop().get_bool();
				if !x {
					Err(VMException::InvalidOpcode(
						"{OpCode::ASSERT} is executed with false result.".parse().unwrap(),
					))
				}
				// break;
			},
			OpCode::Throw => self.execute_throw(self.pop()),
			OpCode::Try => {
				let catch_offset = instr.token_i8();
				let finally_offset = instr.token_i8_1();
				self.execute_try(catch_offset as usize, finally_offset as usize)
				// break;
			},
			OpCode::TryL => {
				let catch_offset = instr.token_i32();
				let finally_offset = instr.token_i32_1();
				self.execute_try(catch_offset as usize, finally_offset as usize)
			},
			OpCode::EndTry => {
				let end_offset = instr.token_i8();
				self.execute_end_try(end_offset as usize)
			},
			OpCode::EndTryL => {
				let end_offset = instr.token_i32();
				self.execute_end_try(end_offset as usize)
			},
			OpCode::EndFinally => {
				if self.current_context.unwrap().try_stack.is_none() {
					return Err(VMException::InvalidOpcode(
						"The corresponding TRY block cannot be found.".parse().unwrap(),
					))
				}
				let current_try = match self.current_context.unwrap().try_stack {
					Some(ref mut x) => x,
					None =>
						return Err(VMException::InvalidOpcode(
							"The corresponding TRY block cannot be found.".parse().unwrap(),
						)),
				};

				if self.uncaught_exception.is_none() {
					self.current_context.unwrap().InstructionPointer = current_try.EndPointer;
				} else {
					self.handle_exception();
				}

				self.is_jumping = true
			},
			OpCode::Ret => {
				let context_pop = self.invocation_stack.pop().unwrap();
				let stack_eval = match self.invocation_stack.len() == 0 {
					true => self.result_stack.clone(),
					false => self
						.invocation_stack
						.last()
						.unwrap()
						.borrow()
						.shared_states
						.evaluation_stack
						.clone(),
				};
				// }
				// ? self.result_stack.clone() : self.invocation_stack.self.peek().EvaluationStack;
				if context_pop.borrow().shared_states.evaluation_stack != stack_eval {
					if context_pop.borrow().rv_count >= 0
						&& context_pop.EvaluationStack.Count != context_pop.RVCount
					{
						return Err(VMException::InvalidOpcode(
							"RVCount doesn't match with EvaluationStack".parse().unwrap(),
						))
					}
					context_pop.EvaluationStack.CopyTo(stack_eval);
				}
				if self.invocation_stack.len() == 0 {
					self.state = VMState::Halt;
				}

				self.unload_context(context_pop);
				self.is_jumping = true
				// break;
			},
			OpCode::Syscall => self.on_syscall(instr.token_u32()),

			// Stack ops
			OpCode::Depth => self.push(self.current_context.unwrap().evaluation_stack.Count),
			OpCode::Drop => self.pop(),
			OpCode::Nip => self.current_context.unwrap().shared_states.evaluation_stack.remove(1),
			OpCode::Xdrop => {
				let n = self.pop().get_integer().to_i32().unwrap();
				if n < 0 {
					return Err(VMException::InvalidOpcode(
						"The negative value {n} is invalid for OpCode::{instr.OpCode}."
							.parse()
							.unwrap(),
					))
				}
				self.current_context.unwrap().shared_states.evaluation_stack.remove(n as i64)
			},
			OpCode::Clear => self.current_context.unwrap().shared_states.evaluation_stack.Clear(),
			OpCode::Dup => self.push(self.peek(0).clone()),
			OpCode::Over => self.push(self.peek(1).clone()),
			OpCode::Pick => {
				let n = self.pop().get_integer();
				if n < BigInt::zero() {
					return Err(VMException::InvalidOpcode(
						"The negative value {n} is invalid for OpCode::{instr.OpCode}."
							.parse()
							.unwrap(),
					))
				}
				self.push(self.peek(n.to_i32().unwrap() as usize).clone())
				// break;
			},
			OpCode::Tuck => self
				.current_context
				.unwrap()
				.shared_states
				.evaluation_stack
				.Insert(2, self.peek(0)),
			OpCode::Swap => {
				let x = self.current_context.unwrap().shared_states.evaluation_stack.remove(1);
				self.push(StackItem::from(x))
				// break;
			},
			OpCode::Rot => {
				let x = self.current_context.unwrap().shared_states.evaluation_stack.remove(2);
				self.push(StackItem::from(x))
			},
			OpCode::Roll => {
				let n = self.pop().get_integer().to_i64().unwrap();
				if n < 0 {
					return Err(VMException::InvalidOpcode(
						"The negative value {n} is invalid for OpCode::{instr.OpCode}."
							.parse()
							.unwrap(),
					))
				}
				if n == 0 {
					return Ok(VMState::None)
				}
				let x = self.current_context.unwrap().shared_states.evaluation_stack.remove(n);
				self.push(StackItem::from(x))
			},
			OpCode::Reverse3 => self.current_context.unwrap().evaluation_stack.Reverse(3),
			OpCode::Reverse4 => self.current_context.unwrap().evaluation_stack.Reverse(4),
			OpCode::ReverseN => {
				let n = self.pop().get_integer();
				self.current_context.unwrap().evaluation_stack.Reverse(n)
			},

			//Slot
			OpCode::InitSSLot => {
				if self.current_context.unwrap().shared_states.static_fields.is_some() {
					return Err(VMException::InvalidOpcode(
						"{instr.OpCode} cannot be executed twice.".parse().unwrap(),
					))
				}
				if instr.token_u8() == 0 {
					return Err(VMException::InvalidOpcode(
						"The operand {instr.token_u8()} is invalid for OpCode::{instr.OpCode}."
							.parse()
							.unwrap(),
					))
				}
				self.current_context.unwrap().shared_states.static_fields = Some(
					Slot::new_with_count(instr.token_u8() as i32, self.reference_counter.clone()),
				)
				// break;
			},
			OpCode::InitSlot => {
				if self.current_context.unwrap().local_variables.is_some()
					|| self.current_context.unwrap().arguments.is_some()
				{
					return Err(VMException::InvalidOpcode(
						"{instr.OpCode} cannot be executed twice.".parse().unwrap(),
					))
				}
				if instr.token_u16() == 0 {
					return Err(VMException::InvalidOpcode(
						"The operand {instr.token_u16()} is invalid for OpCode::{instr.OpCode}."
							.parse()
							.unwrap(),
					))
				}
				if instr.token_u8() > 0 {
					self.current_context.unwrap().local_variables = Some(Slot::new_with_count(
						instr.token_u8() as i32,
						self.reference_counter.clone(),
					));
				}
				if instr.token_u8_1() > 0 {
					// generate a vector of instr.token_u8_1() StackItems
					let mut items = Vec::new();
					let size = instr.token_u8_1() as usize;

					// for _ in 0..size{
					//     items.push(StackItem::default());
					// }
					//
					for i in 0..size {
						items[i] = self.pop();
					}

					self.current_context.unwrap().arguments =
						Some(Slot::new(items, self.reference_counter.clone()))
				}
			},
			OpCode::LdSFLd0
			| OpCode::LdSFLd1
			| OpCode::LdSFLd2
			| OpCode::LdSFLd3
			| OpCode::LdSFLd4
			| OpCode::LdSFLd5
			| OpCode::LdSFLd6 => self.execute_load_from_slot(
				&mut self.current_context.unwrap().shared_states.static_fields.unwrap(),
				instr.OpCode - OpCode::LdSFLd0,
			),
			OpCode::LdSFLd => self.execute_load_from_slot(
				&mut self.current_context.unwrap().shared_states.static_fields.unwrap(),
				instr.token_u8() as usize,
			),
			OpCode::StSFLd0
			| OpCode::StSFLd1
			| OpCode::StSFLd2
			| OpCode::StSFLd3
			| OpCode::StSFLd4
			| OpCode::StSFLd5
			| OpCode::StSFLd6 => self.execute_store_to_slot(
				&mut self.current_context.unwrap().shared_states.static_fields,
				instr.OpCode - OpCode::StSFLd0,
			),
			OpCode::StSFLd => self.execute_store_to_slot(
				&mut self.current_context.unwrap().shared_states.static_fields,
				instr.token_u8() as usize,
			),
			OpCode::LdLoc0
			| OpCode::LdLoc1
			| OpCode::LdLoc2
			| OpCode::LdLoc3
			| OpCode::LdLoc4
			| OpCode::LdLoc5
			| OpCode::LdLoc6 => self.execute_load_from_slot(
				self.current_context.unwrap().shared_states.local_variables,
				instr.OpCode - OpCode::LdLoc0,
			),
			OpCode::LdLoc => self.execute_load_from_slot(
				self.current_context.unwrap().shared_states.local_variables,
				instr.token_u8() as usize,
			),
			OpCode::StLoc0
			| OpCode::StLoc1
			| OpCode::StLoc2
			| OpCode::StLoc3
			| OpCode::StLoc4
			| OpCode::StLoc5
			| OpCode::StLoc6 => self.execute_store_to_slot(
				self.current_context.unwrap().shared_states.local_variables,
				instr.OpCode - OpCode::StLoc0,
			),
			OpCode::StLoc => self.execute_store_to_slot(
				self.current_context.unwrap().shared_states.local_variables,
				instr.token_u8() as usize,
			),
			OpCode::LdArg0
			| OpCode::LdArg1
			| OpCode::LdArg2
			| OpCode::LdArg3
			| OpCode::LdArg4
			| OpCode::LdArg5
			| OpCode::LdArg6 => self.execute_load_from_slot(
				&mut self.current_context.unwrap().arguments.unwrap(),
				instr.OpCode - OpCode::LdArg0,
			),
			OpCode::LdArg => self.execute_load_from_slot(
				&mut self.current_context.unwrap().arguments.unwrap(),
				instr.token_u8() as usize,
			),
			OpCode::StArg0
			| OpCode::StArg1
			| OpCode::StArg2
			| OpCode::StArg3
			| OpCode::StArg4
			| OpCode::StArg5
			| OpCode::StArg6 => self.execute_store_to_slot(
				&mut self.current_context.unwrap().arguments,
				instr.OpCode - OpCode::StArg0,
			),
			OpCode::StArg => self.execute_store_to_slot(
				&mut self.current_context.unwrap().arguments,
				instr.token_u8() as usize,
			),

			// Splice
			OpCode::NewBuffer => {
				let length = self.pop().get_integer();
				self.limits.assert_max_item_size(length.to_u32().unwrap());
				self.push(StackItem::VMBuffer(Buffer::new(length.to_usize().unwrap())))
			},
			OpCode::MemCpy => {
				let count = self.pop().get_integer().to_i64().unwrap();
				if count < 0 {
					return Err(VMException::InvalidOpcode(
						"The value {count} is out of range.".parse().unwrap(),
					))
				}
				let si = self.pop().get_integer().to_i64().unwrap();
				if si < 0 {
					return Err(VMException::InvalidOpcode(
						"The value {si} is out of range.".parse().unwrap(),
					))
				}
				let src = self.pop().get_slice();
				if si.checked_add(count).unwrap() > src.len() as i64 {
					return Err(VMException::InvalidOpcode(
						"The value {count} is out of range.".parse().unwrap(),
					))
				}
				let di = self.pop().get_integer().to_i64().unwrap();
				if (di < 0) {
					return Err(VMException::InvalidOpcode(
						"The value {di} is out of range.".parse().unwrap(),
					))
				}
				let dst: Buffer = self.pop().into();
				if di.checked_add(count) > dst.Size {
					return Err(VMException::InvalidOpcode(
						"The value {count} is out of range.".parse().unwrap(),
					))
				}
				src.Slice(si, count).CopyTo(dst.InnerBuffer.Span[di..])
			},
			OpCode::Cat => {
				let x2 = self.pop().GetSpan();
				let x1 = self.pop().GetSpan();
				let length = x1.Length + x2.Length;
				self.limits.assert_max_item_size(length);
				let result = Buffer::new(length); //, false);
				x1.CopyTo(result.InnerBuffer.Span);
				x2.CopyTo(result.InnerBuffer.Span[x1.Length..]);
				self.push(StackItem::from(result))
				// break;
			},
			OpCode::Substr => {
				let count = self.pop().get_integer().to_usize().unwrap();
				if count < 0 {
					return Err(VMException::InvalidOpcode(
						"The value {count} is out of range.".parse().unwrap(),
					))
				}
				let index = self.pop().get_integer().to_usize().unwrap();
				if index < 0 {
					return Err(VMException::InvalidOpcode(
						"The value {index} is out of range.".parse().unwrap(),
					))
				}
				let x = self.pop().GetSpan();
				if index + count > x.Length {
					return Err(VMException::InvalidOpcode(
						"The value {count} is out of range.".parse().unwrap(),
					))
				}
				let result = Buffer::new(count); //, false);
				x.Slice(index, count).CopyTo(result.InnerBuffer.Span);
				self.push(StackItem::from(result))
			},
			OpCode::Left => {
				let count = self.pop().get_integer().to_i32().unwrap();
				if count < 0 {
					return Err(VMException::InvalidOpcode(
						"The value {count} is out of range.".parse().unwrap(),
					))
				}
				let x = self.pop().GetSpan();
				if count > x.Length {
					return Err(VMException::InvalidOpcode(
						"The value {count} is out of range.".parse().unwrap(),
					))
				}
				let result = Buffer::new(count as usize); //, false);
				x[..count].CopyTo(result.InnerBuffer.Span);
				self.push(StackItem::from(result))
			},
			OpCode::Right => {
				let count = self.pop().get_integer().to_i32().unwrap();
				if count < 0 {
					return Err(VMException::InvalidOpcode(
						"The value {count} is out of range.".parse().unwrap(),
					))
				}
				let x = self.pop().get_slice();
				if count > x.Length {
					return Err(VMException::InvalidOpcode(
						"The value {count} is out of range.".parse().unwrap(),
					))
				}
				let result = Buffer::from(x); //, false);
							  // x[^count.. ^ 0].CopyTo(result.InnerBuffer.Span);
				self.push(StackItem::VMBuffer(result))
				// break;
			},

			// Bitwise logic
			OpCode::Invert => {
				let x = self.pop().get_integer();
				self.push(StackItem::from(BigInt::neg(x)))
			},
			OpCode::And => {
				let x2 = self.pop().get_integer();
				let x1 = self.pop().get_integer();
				self.push(StackItem::from(x1 & x2))
			},
			OpCode::Or => {
				let x2 = self.pop().get_integer();
				let x1 = self.pop().get_integer();
				self.push(StackItem::from(x1 | x2))
			},
			OpCode::Xor => {
				let x2 = self.pop().get_integer();
				let x1 = self.pop().get_integer();
				self.push(StackItem::from(x1 ^ x2))
			},
			OpCode::Equal => {
				let x2 = self.pop();
				let x1 = self.pop();
				self.push(x1.Equal(x2, self.limits))
			},
			OpCode::NotEqual => {
				let x2 = self.pop();
				let x1 = self.pop();
				self.push(!x1.Equals(x2, self.limits))
			},

			// Numeric
			OpCode::Sign => {
				let x = self.pop().get_integer();
				self.push(StackItem::from(x.Sign))
			},
			OpCode::Abs => {
				let x = self.pop().get_integer();
				self.push(StackItem::from(BigInt::abs(&x)))
			},
			OpCode::Negate => {
				let x = self.pop().get_integer();
				self.push(StackItem::from(-x))
			},
			OpCode::Inc => {
				let x = self.pop().get_integer();
				self.push(StackItem::from(x + 1))
			},
			OpCode::Dec => {
				let x = self.pop().get_integer();
				self.push(StackItem::from(x - 1))
			},
			OpCode::Add => {
				let x2 = self.pop().get_integer();
				let x1 = self.pop().get_integer();
				self.push(StackItem::from(x1 + x2))
			},
			OpCode::Sub => {
				let x2 = self.pop().get_integer();
				let x1 = self.pop().get_integer();
				self.push(StackItem::from(x1 - x2))
			},
			OpCode::Mul => {
				let x2 = self.pop().get_integer();
				let x1 = self.pop().get_integer();
				self.push(StackItem::from(x1 * x2))
			},
			OpCode::Div => {
				let x2 = self.pop().get_integer();
				let x1 = self.pop().get_integer();
				self.push(StackItem::from(x1 / x2))
			},
			OpCode::Mod => {
				let x2 = self.pop().get_integer();
				let x1 = self.pop().get_integer();
				self.push(StackItem::from(x1 % x2))
			},
			OpCode::Pow => {
				let exponent = self.pop().get_integer().to_i32().unwrap();
				self.limits.assert_shift(exponent);
				let value = self.pop().get_integer();
				self.push(StackItem::from(value.pow(exponent as u32)))
			},
			OpCode::Sqrt => self.push(self.pop().get_integer().Sqrt()),
			OpCode::ModMul => {
				let modulus = self.pop().get_integer();
				let x2 = self.pop().get_integer();
				let x1 = self.pop().get_integer();
				self.push(StackItem::from(x1 * x2 % modulus))
			},
			OpCode::ModPow => {
				let modulus = self.pop().get_integer();
				let exponent = self.pop().get_integer().to_i32().unwrap();
				let value = self.pop().get_integer();
				let result = match exponent == -1 {
					true => value.ModInverse(modulus),
					false => value.ModPow(exponent, modulus),
				};
				// } value.ModInverse(modulus) :  BigInteger.ModPow(value, exponent, modulus);
				self.push(StackItem::from(result))
			},
			OpCode::Shl => {
				let shift = self.pop().get_integer().to_i32().unwrap();
				self.limits.assert_shift(shift);
				if shift == 0 {
					return Ok(VMState::None)
				}
				let x = self.pop().get_integer();
				self.push(StackItem::from(x << shift))
			},
			OpCode::Shr => {
				let shift = self.pop().get_integer().to_i32().unwrap();
				self.limits.assert_shift(shift);
				if shift == 0 {
					return Ok(VMState::None) // break;
				}
				let x = self.pop().get_integer();
				self.push(StackItem::from(x >> shift))
			},
			OpCode::Not => {
				let x = self.pop().get_bool();
				self.push(StackItem::from(!x))
			},
			OpCode::BoolAnd => {
				let x2 = self.pop().get_bool();
				let x1 = self.pop().get_bool();
				self.push(StackItem::from(x1 && x2))
			},
			OpCode::BoolOr => {
				let x2 = self.pop().get_bool();
				let x1 = self.pop().get_bool();
				self.push(StackItem::from(x1 || x2))
			},
			OpCode::Nz => {
				let x = self.pop().get_integer();
				self.push(StackItem::from(!x.is_zero()))
			},
			OpCode::NumEqual => {
				let x2 = self.pop().get_integer();
				let x1 = self.pop().get_integer();
				self.push(StackItem::from(x1 == x2))
			},
			OpCode::NumNotEqual => {
				let x2 = self.pop().get_integer();
				let x1 = self.pop().get_integer();
				self.push(StackItem::from(x1 != x2))
			},
			OpCode::Lt => {
				let x2 = self.pop();
				let x1 = self.pop();
				if x1.get_item_type() == StackItemType::Any
					|| x2.get_item_type() == StackItemType::Any
				{
					self.push(StackItem::from(false))
				} else {
					self.push(StackItem::from(x1.get_integer() < x2.get_integer()))
				}
			},
			OpCode::Le => {
				let x2 = self.pop();
				let x1 = self.pop();
				if x1.get_item_type() == StackItemType::Any
					|| x2.get_item_type() == StackItemType::Any
				{
					self.push(StackItem::from(false))
				} else {
					self.push(StackItem::from(x1.get_integer() <= x2.get_integer()))
				}
				// break;
			},
			OpCode::Gt => {
				let x2 = self.pop();
				let x1 = self.pop();
				if x1.get_item_type() == StackItemType::Any
					|| x2.get_item_type() == StackItemType::Any
				{
					self.push(StackItem::from(false))
				} else {
					self.push(StackItem::from(x1.get_integer() > x2.get_integer()))
				}
				// break;
			},
			OpCode::Ge => {
				let x2 = self.pop();
				let x1 = self.pop();
				if x1.get_item_type() == StackItemType::Any
					|| x2.get_item_type() == StackItemType::Any
				{
					self.push(StackItem::from(false))
				} else {
					self.push(StackItem::from(x1.get_integer() >= x2.get_integer()))
				}
				// break;
			},
			OpCode::Min => {
				let x2 = self.pop().get_integer();
				let x1 = self.pop().get_integer();
				self.push(StackItem::from(BigInt::min(x1, x2)))
			},
			OpCode::Max => {
				let x2 = self.pop().get_integer();
				let x1 = self.pop().get_integer();
				self.push(StackItem::from(BigInt::max(x1, x2)))
				// break;
			},
			OpCode::Within => {
				let b = self.pop().get_integer();
				let a = self.pop().get_integer();
				let x = self.pop().get_integer();
				self.push(StackItem::from(a <= x && x < b))
			},

			// Compound-type
			OpCode::PackMap => {
				let size = self.pop().get_integer().to_usize().unwrap();
				if size < 0 || size * 2 > self.current_context.unwrap().evaluation_stack.Count {
					return Err(VMException::InvalidOpcode(
						"The value {size} is out of range.".parse().unwrap(),
					))
				}
				let map = Map::new(Some(self.reference_counter.clone()));
				for i in 0..size {
					let key: PrimitiveType = self.pop().into();
					let value = self.pop();
					map[key] = value;
				}
				self.push(StackItem::VMMap(map))
			},
			OpCode::PackStruct => {
				let size = self.pop().get_integer().to_i64().unwrap();
				if size < 0 || size > self.current_context.unwrap().evaluation_stack.Count {
					return Err(VMException::InvalidOpcode(
						"The value {size} is out of range.".parse().unwrap(),
					))
				}
				let _struct = Struct::new(None, Some(self.reference_counter.clone()));
				for i in 0..size {
					let item = self.pop();
					_struct.Add(item);
				}
				self.push(StackItem::VMStruct(_struct))
				// break;
			},
			OpCode::Pack => {
				let size = self.pop().get_integer().to_usize().unwrap();
				if size < 0
					|| size > self.current_context.unwrap().shared_states.evaluation_stack.len()
				{
					return Err(VMException::InvalidOpcode(
						"The value {size} is out of range.".parse().unwrap(),
					))
				}
				let array = Array::new(None, Some(self.reference_counter.clone()));
				for i in 0..size {
					let item = self.pop();
					array.Add(item);
				}
				self.push(StackItem::VMArray(array))
			},
			OpCode::Unpack => {
				let compound: CompoundType = self.pop().into();
				match compound {
					CompoundType::VMMap(map) =>
						for (key, value) in map.values().rev() {
							self.push((value as PrimitiveType).into());
							self.push(key as StackItem);
						},

					// break;
					CompoundType::VMArray(array) =>
						for i in (0..=array.array.len()).rev() {
							self.push(array[i]);
						},
					// break;
					_ =>
						return Err(VMException::InvalidOpcode(
							"Invalid type for {instr.OpCode}: {compound.Type}".parse().unwrap(),
						)),
				}
				self.push(StackItem::from(compound.Count))
			},
			OpCode::NewArray0 => self
				.push(StackItem::VMArray(Array::new(None, Some(self.reference_counter.clone())))),
			OpCode::NewArray | OpCode::NewArrayT => {
				let n = self.pop().get_integer().to_i64().unwrap();
				if n < 0 || n > self.limits.MaxStackSize {
					return Err(VMException::InvalidOpcode(
						"MaxStackSize exceed: {n}".parse().unwrap(),
					))
				}
				let item: StackItem;
				if instr.OpCode == OpCode::NewArrayT {
					let _type = instr.token_u8();
					if !StackItemType::is_valid(_type) {
						return Err(VMException::InvalidOpcode(
							"Invalid type for {instr.OpCode}: {instr.token_u8()}".parse().unwrap(),
						))
					}
					item = match _type as StackItemType {
						StackItemType::Boolean => StackItem::from(false),
						StackItemType::Integer => StackItem::from(BigInt::zero()),
						StackItemType::ByteString =>
							StackItem::VMByteString(ByteString::new(Vec::new())),
						_ => StackItem::VMNull(Null::default()),
					};
				} else {
					item = StackItem::VMNull(Null::default());
				}
				self.push(StackItem::from(Array::new(
					std::iter::repeat(item).take(n as usize).collect(),
					Some(self.reference_counter.clone()),
				)))
				// break;
			},
			OpCode::NewStruct0 =>
				self.push(StackItem::from(Struct::new(None, Some(self.reference_counter.clone())))),
			OpCode::NewStruct => {
				let n = self.pop().get_integer() as usize;
				if n < 0 || n > self.limits.max_stack_size {
					return Err(VMException::InvalidOpcode(
						"MaxStackSize exceed: {n}".parse().unwrap(),
					))
				}
				let result = Struct::new(None, Some(self.reference_counter.clone()));
				for i in 0..n {
					result.Add(StackItem::from(Null::default()));
				}
				self.push(StackItem::from(result))
				// break;
			},
			OpCode::NewMap =>
				self.push(StackItem::from(Map::new(Some(self.reference_counter.clone())))),
			OpCode::Size => {
				let x = self.pop();
				match x {
					StackItem::VMArray(array) => self.push(StackItem::from(array.Count)),
					StackItem::VMMap(map) => self.push(StackItem::from(map.Count)),
					StackItem::VMStruct(_struct) => self.push(StackItem::from(_struct.Count)),
					StackItem::VMByteString(array) => self.push(StackItem::from(array.Size)),
					StackItem::VMBuffer(buffer) => self.push(StackItem::from(buffer.Size)),
					StackItem::VMInteger(integer) => self.push(StackItem::from(integer.size())),
					_ =>
						return Err(VMException::InvalidOpcode(
							"Invalid type for {instr.OpCode}: {x.Type}".parse().unwrap(),
						)),
				}
			},
			OpCode::HasKey => {
				let key: PrimitiveType = self.pop().into();
				let x = self.pop();
				match x {
					StackItem::VMMap(map) => self.push(StackItem::from(map.contains_key(&key))),
					StackItem::VMByteString(array) => {
						let index = key.get_integer().to_u32().unwrap();
						if index < 0 {
							return Err(VMException::InvalidOpcode(
								"The negative value {index} is invalid for OpCode::{instr.OpCode}."
									.parse()
									.unwrap(),
							))
						}
						self.push(StackItem::from(index < array.Size))
					},
					StackItem::VMBuffer(buffer) => {
						let index = key.get_integer().to_u32().unwrap();
						if index < 0 {
							return Err(VMException::InvalidOpcode(
								"The negative value {index} is invalid for OpCode::{instr.OpCode}."
									.parse()
									.unwrap(),
							))
						}
						self.push(StackItem::from(index < buffer.Size))
					},
					StackItem::VMArray(array) => {
						let index = key.get_integer().to_u32().unwrap();
						if index < 0 {
							return Err(VMException::InvalidOpcode(
								"The negative value {index} is invalid for OpCode::{instr.OpCode}."
									.parse()
									.unwrap(),
							))
						}

						self.push(StackItem::from(index < array.Count))
					},
					_ =>
						return Err(VMException::InvalidOpcode(
							"Invalid type for {instr.OpCode}: {x.Type}".parse().unwrap(),
						)),
				}
				// break;
			},
			OpCode::Keys => {
				let map: Map = self.pop().into();
				self.push(VMArray(VMArray::new(&self.reference_counter, map.Keys)))
			},
			OpCode::Values => {
				let x = self.pop();
				let values = match x {
					StackItem::VMArray(array) => array,
					StackItem::VMMap(map) => map.values(),
					_ => panic!(), //return Err(VMException::InvalidOpcode("Invalid type for {instr.OpCode}: {x.Type}".parse().unwrap())),
				};
				let mut new_array = Array::new(None, Some(self.reference_counter.clone()));
				for item in values.array {
					if item.get_item_type() == StackItemType::Struct {
						let s: Struct = item.into();
						new_array.add(s.clone(&self.limits).try_into().unwrap());

					// new_array.Add(s.Clone(self.limits));
					} else {
						new_array.Add(item);
					}
				}

				self.push(StackItem::VMArray(new_array))
			},
			OpCode::PickItem => {
				let key: PrimitiveType = self.pop().into();
				let x = self.pop();
				match x {
					StackItem::VMArray(array) => {
						let index = key.get_integer().to_i64().unwrap();
						if index < 0 || index >= array.Count {
							return Err(VMException::InvalidOpcode(
								"The value {index} is out of range.".parse().unwrap(),
							))
						}
						self.push(array[index])
					},
					StackItem::VMMap(map) => {
						let value = match map.get(&key) {
							Some(v) => v,
							None =>
								return Err(VMException::InvalidOpcode(
									"Key not found in {nameof(Map)}".parse().unwrap(),
								)),
						};
						self.push(StackItem::from(value))
					},
					StackItem::VMByteString(byte_string) => {},
					StackItem::VMBoolean(boolean) => {},
					StackItem::VMInteger(integer) => {
						let byte_array = integer.get_slice();
						let index = key.get_integer().to_i64().unwrap();
						if index < 0 || index >= byte_array.Length {
							return Err(VMException::InvalidOpcode(
								"The value {index} is out of range.".parse().unwrap(),
							))
						}
						self.push(StackItem::from(BigInt::from_bytes_le(
							Sign::NoSign,
							byte_array.get(index).unwrap(),
						)))
					},
					StackItem::VMBuffer(buffer) => {
						let index = key.get_integer().to_i64().unwrap();
						if index < 0 || index >= buffer.Size {
							return Err(VMException::InvalidOpcode(
								"The value {index} is out of range.".parse().unwrap(),
							))
						}
						self.push(StackItem::from(BigInt::from_bytes_le(
							Sign::NoSign,
							buffer.get_slice().get(index).unwrap(),
						)))
					},
					_ =>
						return Err(VMException::InvalidOpcode(
							"Invalid type for {instr.OpCode}: {x.Type}".parse().unwrap(),
						)),
				}
				// break;
			},
			OpCode::Append => {
				let mut new_item = self.pop();
				let array: Array = self.pop().into();
				if new_item.get_item_type() == StackItemType::Struct {
					let s: Struct = new_item.into();
					new_item = s.clone(&self.limits).try_into().unwrap();
					// new_item = s.Clone(self.limits);
				}
				array.Add(new_item)
			},
			OpCode::SetItem => {
				let mut value = self.pop();
				if value.get_item_type() == StackItemType::Struct {
					let s: Struct = value.into();
					value = s.clone(&self.limits).try_into().unwrap();
				}
				let key: PrimitiveType = self.pop().into();
				let x = self.pop();
				match x {
					VMArray(array) => {
						let index = key.get_integer().to_i32().unwrap();
						if index < 0 || index >= array.Count {
							return Err(VMException::InvalidOpcode(
								"The value {index} is out of range.".parse().unwrap(),
							))
						}
						array[index] = value
					},
					StackItem::VMMap(map) => map[key] = value,
					StackItem::VMBuffer(buffer) => {
						let index = key.get_integer().to_i32().unwrap();
						if index < 0 || index >= buffer.Size {
							return Err(VMException::InvalidOpcode(
								"The value {index} is out of range.".parse().unwrap(),
							))
						}
						if !StackItemType::is_primitive(value.get_item_type() as u8) {
							return Err(VMException::InvalidOpcode(
								"Value must be a primitive type in {instr.OpCode}".parse().unwrap(),
							))
						}
						let b = value.get_integer().to_i64().unwrap();
						if b < i8::min as i64 || b > i8::max as i64 {
							return Err(VMException::InvalidOpcode(
								"Overflow in {instr.OpCode}, {b} is not a byte type."
									.parse()
									.unwrap(),
							))
						}
						buffer.InnerBuffer.Span[index] = b
					},
					_ => Err(VMException::InvalidOpcode(
						"Invalid type for {instr.OpCode}: {x.Type}".parse().unwrap(),
					)),
				}
				// break;
			},
			OpCode::ReverseItems => {
				let x = self.pop();
				match x {
					StackItem::VMArray(array) => array.Reverse(),
					StackItem::VMBuffer(buffer) => buffer.InnerBuffer.Span.Reverse(),
					_ => Err(VMException::InvalidOpcode(
						"Invalid type for {instr.OpCode}: {x.Type}".parse().unwrap(),
					)),
				}
			},
			OpCode::Remove => {
				let key: PrimitiveType = self.pop().into();
				let x = self.pop();
				match (x) {
					StackItem::VMArray(mut array) => {
						let index = key.get_integer().to_i32().unwrap();
						if index < 0 || index >= array.Count {
							return Err(VMException::InvalidOpcode(
								"The value {index} is out of range.".parse().unwrap(),
							))
						}
						array.remove_at(index as usize)
					},
					StackItem::VMMap(mut map) => map.remove(key),
					_ =>
						return Err(VMException::InvalidOpcode(
							"Invalid type for {instr.OpCode}: {x.Type}".parse().unwrap(),
						)),
				}
			},
			OpCode::ClearItems => {
				let x: CompoundType = self.pop().into();
				x.Clear()
			},
			OpCode::PopItem => {
				let mut x: Array = self.pop().into();
				let index = x.Count - 1;
				self.push(x[index]);
				x.remove_at(index)
			},

			//Types
			OpCode::IsNull => {
				let x = self.pop();
				self.push(StackItem::from(x.get_item_type() == StackItemType::Any))
			},
			OpCode::IsType => {
				let x = self.pop();
				let _type: StackItemType = instr.token_u8() as StackItemType;
				if _type == StackItemType::Any || !StackItemType::is_valid(instr.token_u8()) {
					return Err(VMException::InvalidOpcode("Invalid type: {type}".parse().unwrap()))
				}
				self.push(StackItem::from(x.get_item_type() == _type))
			},
			OpCode::Convert => {
				let x = self.pop();
				self.push(x.ConvertTo(instr.token_u8()))
			},
			OpCode::AbortMsg => {
				let msg = self.pop().GetString();
				Err(VMException::InvalidOpcode(
					"{OpCode::ABORTMSG} is executed. Reason: {msg}".parse().unwrap(),
				))
			},
			OpCode::AssertMsg => {
				let msg = self.pop().GetString();
				let x = self.pop().get_bool();
				if !x {
					return Err(VMException::InvalidOpcode(
						"{OpCode::ASSERTMSG} is executed with false result. Reason: {msg}"
							.parse()
							.unwrap(),
					))
				}
				// break;
			},
			_ => panic!("Opcode {instr} is undefined."),
		}

		Ok(VMState::Halt)
	}

	fn execute_call(&mut self, offset: i32) {
		let new_context = self.current_context.unwrap().clone_at_offset(offset);
		self.load_context(new_context);
	}

	fn execute_jump_offset(&mut self, offset: i32) {
		self.execute_jump(
			(self.current_context.unwrap().instr_pointer as i32)
				.checked_add(offset)
				.unwrap(),
		)
	}
	fn execute_jump(&mut self, offset: i32) {
		let new_ip = (self.current_context.unwrap().instr_pointer as i32 + offset) as usize;
		if new_ip >= self.current_context.unwrap().script.0.len() {
			return self.handle_error(Error::InvalidJump)
		}
		self.current_context.unwrap().instr_pointer = new_ip;
	}

	fn handle_error(&mut self, err: Error) {
		self.state = VMState::Fault;
		self.uncaught_exception = Some(StackItem::VMNull(Null::default()));
	}

	fn load_context(&mut self, context: &Rc<RefCell<ExecutionContext>>) {
		self.invocation_stack.push(context.clone());
		self.current_context = Some(self.invocation_stack.last().unwrap().clone());
		if self.entry_context.is_none() {
			self.entry_context = self.current_context.clone();
		}
	}

	fn unload_context(&mut self, mut context: Rc<RefCell<ExecutionContext>>) {
		if let Some(current) = &mut self.current_context {
			if current.evaluation_stack != context.evaluation_stack {
				context.evaluation_stack.clear();
				current.evaluation_stack.append(&mut context.evaluation_stack);
			}
		}

		if self.invocation_stack.is_empty() {
			self.current_context = None;
			self.entry_context = None;
		} else {
			self.current_context = Some(self.invocation_stack.last().unwrap().clone());
		}

		context.clear();
	}

	fn create_context(
		&self,
		script: Script,
		rvcount: i32,
		initial_position: usize,
	) -> ExecutionContext {
		let share = SharedStates {
			script,
			evaluation_stack: Default::default(),
			static_fields: None,
			states: Default::default(),
		};

		ExecutionContext {
			shared_states,
			instruction_pointer: initial_position,
			rv_count: rvcount,
			local_variables: None,
			try_stack: None,
			arguments: None,
		}
	}

	fn load_script(
		&mut self,
		script: Script,
		rvcount: i32,
		initial_position: usize,
	) -> Rc<RefCell<ExecutionContext>> {
		let context = Rc::new(RefCell::new(self.create_context(script, rvcount, initial_position)));

		self.load_context(&context);

		context
	}

	fn pre_execute_instruction(&mut self, instruction: Instruction) {
		if self.reference_counter.borrow().count > self.limits.max_stack_size {
			panic!("Max stack size exceeded");
		}

		match instruction {
			Instruction::JMP(offset) => {
				self.is_jumping = true;
			},
			Instruction::CALL(context) => {
				self.load_context(context);
			},
			_ => (),
		}
	}

	fn post_execute_instruction(&mut self, instruction: Instruction) {
		let count = self.reference_counter.borrow().count;
		if count > self.limits.max_stack_size {
			panic!("Max stack size exceeded: {}", count);
		}

		match instruction {
			Instruction::RET => {
				let context = self.invocation_stack.pop().unwrap();
				// do something with returned context
			},
			Instruction::THROW => {
				self.handle_exception();
			},
			_ => (),
		}
	}
	fn handle_exception(&mut self) {
		// loop through contexts
		// set instruction pointer to catch or finally
		// pop contexts
		if let Some(exception) = self.uncaught_exception.take() {
			panic!("Unhandled exception: {:?}", exception);
		}
	}

	fn execute_try(&mut self, catch_offset: usize, finally_offset: usize) {
		let context = self.current_context.as_mut().unwrap().borrow_mut();

		if catch_offset == 0 && finally_offset == 0 {
			panic!("Invalid try block offsets");
		}

		if context.try_stack.is_none() {
			context.try_stack = Some(Vec::new());
		}

		if context.try_stack.as_ref().unwrap().len() >= self.limits.max_try_nesting_depth {
			panic!("Max try nesting depth exceeded");
		}

		let catch_pointer =
			if catch_offset > 0 { Some(context.instruction_pointer + catch_offset) } else { None };

		let finally_pointer = if finally_offset > 0 {
			Some(context.instruction_pointer + finally_offset)
		} else {
			None
		};

		context.try_stack.as_mut().unwrap().push(ExceptionHandlingContext {
			state: ExceptionHandlingState::Try,
			catch_pointer: catch_pointer.unwrap() as i32,
			finally_pointer: finally_pointer.unwrap() as i32,
			end_pointer: 0,
		});

		self.is_jumping = true;
	}

	fn execute_throw(&mut self, exception: StackItem) {
		self.uncaught_exception = Some(exception);
		self.handle_exception();
	}

	fn execute_end_try(&mut self, end_offset: usize) {
		let context = self.current_context.as_mut().unwrap().borrow_mut();

		let mut current_try = match context.try_stack.as_mut().unwrap().pop() {
			Some(try_context) => try_context,
			None => panic!("No matching try block found"),
		};

		if let ExceptionHandlingState::Finally = current_try.state() {
			panic!("EndTry cannot be called in finally block");
		}

		let end_pointer = context.instruction_pointer + end_offset;

		if let Some(handler) = current_try.finally_pointer() {
			current_try.set_state(ExceptionHandlingState::Finally);
			current_try.set_end_pointer(end_pointer as i32);
			context.instruction_pointer = handler;
		} else {
			context.instruction_pointer = end_pointer;
		}

		self.is_jumping = true;
	}

	fn execute_load_from_slot(&mut self, slot: &mut Slot, index: usize) {
		if let Some(values) = slot {
			if index < values.len() {
				let value = values[index].clone();
				self.push(value);
			} else {
				panic!("Invalid slot index: {}", index);
			}
		} else {
			panic!("Slot not initialized");
		}
	}

	fn execute_store_to_slot(&mut self, slot: &mut Option<Slot>, index: usize) {
		if let Some(slot) = slot {
			if index >= slot.len() {
				panic!("Index out of range when storing to slot: {}", index);
			}

			let value = self.result_stack.pop();
			slot[index] = value;
		} else {
			panic!("Slot has not been initialized.");
		}
	}

	fn load_token(&mut self, token: u16) -> Result<ExecutionContext, &'static str> {
		panic!("Not implemented");
	}

	fn on_syscall(&mut self, method: u32) {
		panic!("Not implemented")
		// let syscall = match method {
		//     0 => Syscall::Syscall0,
		//     1 => Syscall::Syscall1,
		//     _ => panic!("Invalid syscall: {}", method),
		// };
		//
		// syscall.invoke(self);
	}
}
