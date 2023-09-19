use crate::{
	stack_item::{ObjectReferenceEntry, StackItem},
	stack_item_type::StackItemType,
};
use std::{cell::RefCell, collections::HashMap, hash::Hash, num::TryFromIntError};
use std::any::Any;
use std::rc::Rc;

use crate::types::{
	compound_types::compound_type::CompoundType,
	primitive_types::primitive_type::{PrimitiveType},
};
use num_bigint::BigInt;
use num_traits::{One, Zero};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::execution_engine_limits::ExecutionEngineLimits;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Boolean {
	stack_references: u32,
	object_references: Rc<RefCell<Option<HashMap<CompoundType, ObjectReferenceEntry>>>>,
	dfn: isize,
	low_link: usize,
	on_stack: bool,
	value: bool,
}

impl Boolean {
	pub fn new(value: bool) -> Self {
		Self {
			stack_references: 0,
			object_references: Rc::new(RefCell::new(None)),
			dfn: 0,
			low_link: 0,
			on_stack: false,
			value,
		}
	}

}

impl Serialize for Boolean {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
		serializer.serialize_bool(self.value)
	}
}

impl Deserialize for Boolean {
	fn deserialize<'de, D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
		let value = bool::deserialize(deserializer)?;
		Ok(Boolean::new(value))
	}
}


impl PrimitiveType for Boolean{
	fn memory(&self) -> &[u8] {
		if self.value {
			Self::TRUE.clone().as_slice()
		} else {
			Self::FALSE.clone().as_slice()
		}
	}
}

impl StackItem for Boolean {
	const TRUE: Self = Boolean::new(true);

	const FALSE: Self = Boolean::new(false);

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

	fn cleanup(&mut self) {
		panic!("Boolean cleanup")
	}

	fn get_slice(&self) -> &[u8] {
		todo!()
	}

	fn get_type(&self) -> StackItemType {
		StackItemType::Boolean
	}

	fn get_boolean(&self) -> bool {
		self.value
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

	fn from_interface(value: &dyn Any) -> Box<dyn StackItem> {
		todo!()
	}

	fn get_integer(&self) -> Result<BigInt, TryFromIntError> {
		return Ok(if self.value { BigInt::one() } else { BigInt::zero() })
	}

	fn get_interface<T: Any>(&self) -> Option<&T> {
		todo!()
	}

	fn get_bytes(&self) -> &[u8] {
		self.get_slice()
	}
}