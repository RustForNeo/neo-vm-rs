use std::{cell::RefCell, collections::HashMap, hash::Hash};

use crate::{
	stack_item::{ObjectReferenceEntry, StackItem, StackItem},
	stack_item_type::StackItemType,
	vm::script::Script,
};
use crate::compound_types::compound_type::CompoundType;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Pointer {
	stack_references: u32,
	object_references: RefCell<Option<HashMap<CompoundType, ObjectReferenceEntry>>>,
	dfn: isize,
	low_link: usize,
	on_stack: bool,
	script: Script,
	position: usize,
}

impl Pointer {
	pub fn new(script: Script, position: usize) -> Self {
		Self {
			stack_references: 0,
			object_references: RefCell::new(None),
			dfn: 0,
			low_link: 0,
			on_stack: false,
			script,
			position,
		}
	}
}

impl StackItem for Pointer {
	type ObjectReferences = RefCell<Option<HashMap<CompoundType, ObjectReferenceEntry>>>;

	fn dfn(&self) -> isize {
		self.dfn
	}

	fn set_dfn(&mut self, dfn: isize) {
		self.dfn = dfn;
	}

	fn low_link(&self) -> usize {
		self.low_link
	}

	fn set_low_link(&mut self, link: usize) {
		self.low_link = link;
	}

	fn on_stack(&self) -> bool {
		self.on_stack
	}

	fn set_on_stack(&mut self, on_stack: bool) {
		self.on_stack = on_stack;
	}

	fn set_object_references(&mut self, refs: Self::ObjectReferences) {
		self.object_references = refs;
	}

	fn object_references(&self) -> &Self::ObjectReferences {
		&self.object_references
	}

	fn set_stack_references(&mut self, count: usize) {
		self.stack_references = count as u32;
	}

	fn stack_references(&self) -> usize {
		self.stack_references as usize
	}

	fn cleanup(&mut self) {
		todo!()
	}

	fn get_boolean(&self) -> bool {
		true
	}

	fn get_slice(&self) -> &[u8] {
		todo!()
	}

	fn get_type(&self) -> StackItemType {
		StackItemType::Pointer
	}

	fn equals(&self, other: &Option<StackItem>) -> bool {
		todo!()
	}
}

// fn equals(&self, other: &Option<StackItem>) -> bool {
//     match other {
//         Some(o) => self == o.as_ref().downcast_ref::<Pointer>().unwrap(),
//         None => false,
//     }
// }
//
// fn get_boolean(&self) -> bool {
//     true
// }
