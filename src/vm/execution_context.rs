use crate::{
	evaluation_stack::EvaluationStack,
	exception::exception_handling_context::ExceptionHandlingContext,
	reference_counter::ReferenceCounter, slot::Slot, stack_item::StackItem, vm::script::Script,
};
use std::{
	any::{Any, TypeId},
	cell::{Ref, RefCell},
	collections::HashMap,
	rc::Rc,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ExecutionContext {
	pub shared_states: SharedStates,
	pub instruction_pointer: usize,

	/// The number of return values when this context returns.
	pub rv_count: i32,

	/// The local variables of this context.
	pub local_variables: Option<Slot>,

	/// The arguments passed to this context.
	pub arguments: Option<Slot>,

	/// The try stack to handle exceptions.
	pub try_stack: Option<Vec<ExceptionHandlingContext>>,
}

pub struct SharedStates {
	pub(crate) script: Script,
	pub(crate) evaluation_stack: Rc<RefCell<EvaluationStack>>,
	pub(crate) static_fields: Option<Slot>,
	pub(crate) states: HashMap<TypeId, Box<dyn Any>>,
}

impl ExecutionContext {
	pub fn new(script: Script, reference_counter: Rc<RefCell<ReferenceCounter>>) -> Self {
		let shared_states = SharedStates {
			script,
			evaluation_stack: Ref::new(RefCell::new(EvaluationStack::new(reference_counter))),
			static_fields: None,
			states: HashMap::new(),
		};
		Self {
			shared_states,
			instruction_pointer: 0,
			rv_count: 0,
			local_variables: None,
			arguments: None,
			try_stack: None,
		}
	}

	// Other fields and methods

	pub fn get_state<T: 'static>(&mut self) -> &mut T
	where
		T: Default + Any,
	{
		self.shared_states
			.states
			.entry(TypeId::of::<T>())
			.or_insert_with(|| Box::new(Default::default()))
			.downcast_mut::<T>()
			.unwrap()
	}

	pub fn peek(&self, index: usize) -> Rc<RefCell<dyn StackItem>> {
		let idx = self.items.len() - index - 1;
		&self.items[idx]
	}

	pub fn push(&mut self, item:Rc<RefCell<dyn StackItem>>) {
		self.items.push(item);
		self.reference_counter.add_stack_reference(&item);
	}

	pub fn pop(&mut self) -> Rc<RefCell<dyn StackItem>> {
		let item = self.items.pop().expect("stack empty");
		self.reference_counter.remove_stack_reference(&item);
		item
	}

	pub fn remove(&mut self, index: usize) -> Rc<RefCell<dyn StackItem>> {
		let idx = self.items.len() - index - 1;
		let item = self.items.remove(idx).expect("index out of bounds");
		self.reference_counter.remove_stack_reference(&item);
		item.try_into().unwrap()
	}

	pub fn move_next(&mut self) {
		self.instruction_pointer += 1;

		if self.instruction_pointer >= self.script.len() {
			self.instruction_pointer = 0;
		}
	}
}
