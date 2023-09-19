use crate::{
	stack_item::{ObjectReferenceEntry, StackItem},
	stack_item_type::StackItemType,
};
use std::{
	cell::RefCell,
	collections::HashMap,
	fmt::{Debug, Formatter},
	hash::{Hash, Hasher},
};
use num_bigint::BigInt;
use crate::compound_types::compound_type::CompoundType;
use crate::execution_engine_limits::ExecutionEngineLimits;

/// Represents `null` in the vm.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct Null {
	dfn:isize,
	low_link: usize,
	on_stack: bool,
	object_references: RefCell<Option<HashMap<dyn CompoundType, ObjectReferenceEntry>>>,
	stack_references: u32,
}

impl StackItem for Null {
	const TRUE: Self = Default::default();

	const FALSE: Self = Default::default();

	const NULL: Self = Default::default();

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

	fn is_null(&self) -> bool {
		true
	}

	fn cleanup(&mut self) {
		todo!()
	}

	fn convert_to(&self, ty: StackItemType) -> Result<StackItem, Err()> {
		if ty == StackItemType::Any {
			Ok(StackItem::VMNull(Self))
		} else {
			Err(())
		}
	}

	fn get_slice(&self) -> &[u8] {
		todo!()
	}

	fn get_string(&self) -> Option<String> {
		None
	}

	fn get_hash_code(&self) -> u64 {
		0
	}

	fn get_type(&self) -> StackItemType {
		StackItemType::Any
	}

	fn get_boolean(&self) -> bool {
		false
	}
	fn deep_copy(&self, asImmutable: bool) -> Box<dyn StackItem> {
		todo!()
	}
	fn deep_copy_with_ref_map(&self, ref_map: &HashMap<&dyn StackItem, &dyn StackItem>, asImmutable: bool) -> Box<dyn StackItem> {
		todo!()
	}

	fn equals(&self, other: &Option<dyn StackItem>) -> bool {
		todo!()
	}

	fn equals_with_limits(&self, other: &dyn StackItem, limits: &ExecutionEngineLimits) -> bool {
		todo!()
	}

	fn get_integer(&self) -> BigInt {
		todo!()
	}

	fn get_interface<T: 'static>(&self) -> Result<&T, ()> {
		Err(())
	}

	fn get_bytes(&self) -> &[u8] {
		todo!()
	}
}

impl Into<dyn StackItem> for Null {
	fn into(self) -> Box<dyn StackItem> {
		StackItem::VMNull(self)
	}
}

impl From<dyn StackItem> for Null {
	fn from(item: Box<dyn StackItem>) -> Self {
		match item {
			StackItem::VMNull(n) => n,
			_ => panic!("Cannot convert {:?} to Null", item),
		}
	}
}
