use crate::{
	stack_item_type::StackItemType,
};
use std::{
	cell::RefCell,
	fmt::{Debug},
	hash::{Hash, Hasher},
	rc::Rc,
	string::FromUtf8Error,
};
use std::any::Any;
use std::collections::HashMap;
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use crate::execution_engine_limits::ExecutionEngineLimits;
use crate::interop_interface::InteropInterface;
use crate::null::Null;

pub trait StackItem: Clone + Hash + Eq+PartialEq+Serialize+Deserialize {
	const TRUE: Self;
	const FALSE: Self;
	const NULL: Self;

	fn dfn(&self) -> isize;

	fn set_dfn(&mut self, dfn: isize);

	fn low_link(&self) -> usize;
	fn set_low_link(&mut self, link: usize);

	fn on_stack(&self) -> bool;
	fn set_on_stack(&mut self, on_stack: bool);

	fn set_object_references(&mut self, refs: Self::ObjectReferences);
	fn object_references(&self) -> &Self::ObjectReferences;

	fn set_stack_references(&mut self, count: usize);

	fn stack_references(&self) -> usize;

	fn successors(&self) -> Vec<dyn StackItem> {
		self.object_references()
			.borrow()
			.as_ref()
			.unwrap()
			.values()
			.map(|v| v.item())
			.collect()
	}

	fn reset(&mut self) {
		self.set_dfn(-1);
		self.set_low_link(0);
		self.set_on_stack(false);
	}

	fn is_null(&self) -> bool {
		false
	}

	fn cleanup(&mut self);

	fn convert_to(&self, type_: StackItemType) -> Result<Self, Err> {
		if type_ == self.get_type() {
			Ok(self.to_owned())
		} else if type_ == StackItemType::Boolean {
			Ok(self.get_boolean())
		} else {
			Err(())
		}
	}

	fn get_slice(&self) -> &[u8];

	fn get_string(&self) -> Result<String, FromUtf8Error> {
		String::from_utf8(self.get_slice().to_vec())
	}

	fn get_hash_code(&self) -> u64 {
		use std::hash::Hasher;
		let mut hasher = std::collections::hash_map::DefaultHasher::new();
		self.hash(&mut hasher);
		hasher.finish()
	}

	fn get_type(&self) -> StackItemType;

	fn get_boolean(&self) -> bool;


	fn deep_copy(&self, asImmutable:bool) -> Box<dyn StackItem>;

	fn deep_copy_with_ref_map(&self, ref_map: &HashMap<&dyn StackItem, &dyn StackItem>, asImmutable:bool) -> Box<dyn StackItem>;

	fn equals(&self, other: &dyn StackItem) -> bool;

	fn equals_with_limits(&self, other: &dyn StackItem, limits: &ExecutionEngineLimits) -> bool;

	fn from_interface(value: Some(dyn Any)) -> Box<dyn StackItem>{

		match value {
			Some(value)=>InteropInterface::new(value),
			None => Null::new(),
		}
	}
	fn get_integer(&self) -> BigInt;

	fn get_interface<T: Any>(&self) -> Option<&T>{
		panic!("Not implemented")
	}


	fn get_bytes(&self) -> &[u8];

	fn to_ref(&self) -> Rc<RefCell<dyn StackItem>> {
		Rc::new(RefCell::new(self.clone()))
	}

}

pub struct ObjectReferenceEntry {
	pub(crate) item: Rc<RefCell<dyn StackItem>>,
	pub(crate) references: i32,
}

impl ObjectReferenceEntry {
	pub fn new(item: Rc<RefCell<dyn StackItem>>) -> Self {
		Self { item, references: 0 }
	}
}
