use crate::{
	compound_type::{CompoundType, CompoundTypeTrait},
	primitive_type::{PrimitiveType, PrimitiveTypeTrait},
	stack_item::{ObjectReferenceEntry, StackItem, StackItemTrait},
	stack_item_type::StackItemType,
};
use std::{cell::RefCell, collections::HashMap, hash::Hash, num::TryFromIntError};

use num_bigint::BigInt;
use num_traits::{One, Zero};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Boolean<'a> {
	stack_references: u32,
	object_references: RefCell<Option<HashMap<CompoundType<'a>, ObjectReferenceEntry<'a>>>>,
	dfn: isize,
	low_link: usize,
	on_stack: bool,
	value: bool,
}

impl Boolean {
	const TRUE: Vec<u8> = vec![1];
	const FALSE: Vec<u8> = vec![0];

	pub fn new(value: bool) -> Self {
		Self {
			stack_references: 0,
			object_references: RefCell::new(None),
			dfn: 0,
			low_link: 0,
			on_stack: false,
			value,
		}
	}

	fn memory(&self) -> Vec<u8> {
		if self.value {
			Self::TRUE.clone()
		} else {
			Self::FALSE.clone()
		}
	}

	fn get_boolean(&self) -> bool {
		self.value
	}

	fn equals(&self, other: &Boolean) -> bool {
		self.value == other.value
	}

	fn get_integer(&self) -> BigInt {
		if self.value {
			BigInt::one()
		} else {
			BigInt::zero()
		}
	}
}

impl<'a> StackItemTrait for Boolean {
	type ObjectReferences = RefCell<Option<HashMap<CompoundType<'a>, ObjectReferenceEntry<'a>>>>;

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
		panic!("Boolean cleanup")
	}

	fn deep_copy(&self, ref_map: &HashMap<&StackItem, StackItem>, as_immutable: bool) -> StackItem {
		todo!()
	}

	fn get_boolean(&self) -> bool {
		self.value
	}

	fn get_integer(&self) -> Result<BigInt, TryFromIntError> {
		return Ok(if self.value { BigInt::one() } else { BigInt::zero() })
	}

	fn get_slice(&self) -> &[u8] {
		todo!()
	}

	fn get_type(&self) -> StackItemType {
		StackItemType::Boolean
	}

	fn equals(&self, other: &Option<StackItem>) -> bool {
		todo!()
	}
}

impl PrimitiveTypeTrait for Boolean {
	fn memory(&self) -> &[u8] {
		todo!()
	}
}

impl Into<StackItem> for Boolean {
	fn into(self) -> StackItem {
		StackItem::VMBoolean(self)
	}
}

impl Into<PrimitiveType> for Boolean {
	fn into(self) -> PrimitiveType {
		PrimitiveType::VMBoolean(self)
	}
}

impl From<PrimitiveType> for Boolean {
	fn from(ty: PrimitiveType) -> Self {
		match ty {
			PrimitiveType::VMBoolean(b) => b,
			_ => panic!("Invalid cast"),
		}
	}
}

impl From<StackItem> for Boolean {
	fn from(item: StackItem) -> Self {
		match item {
			StackItem::VMBoolean(b) => b,
			_ => panic!("Invalid cast"),
		}
	}
}

impl From<bool> for Boolean {
	fn from(value: bool) -> Self {
		Self::new(value)
	}
}

impl Into<StackItem> for bool {
	fn into(self) -> StackItem {
		Boolean::new(self).into()
	}
}

impl Into<PrimitiveType> for bool {
	fn into(self) -> PrimitiveType {
		Boolean::new(self).into()
	}
}
