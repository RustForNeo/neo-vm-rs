use crate::{
	array::Array,
	map::Map,
	reference_counter::ReferenceCounter,
	stack_item::{StackItem, StackItemTrait},
	Struct::Struct,
};
use std::{
	cell::RefCell,
	collections::HashMap,
	hash::Hash,
	num::TryFromIntError,
	rc::{Rc, Weak},
};

pub trait CompoundTypeTrait: StackItemTrait {
	fn reference_counter(&self) -> Option<&ReferenceCounter>;

	fn count(&self) -> usize;
	fn sub_items(&self) -> Vec<&StackItem>;
	fn sub_items_count(&self) -> usize;

	fn is_read_only(&self) -> bool;

	fn clear(&mut self);

	fn deep_copy(&self, ref_map: &HashMap<&StackItem, StackItem>) -> StackItem;

	fn get_boolean(&self) -> bool {
		true
	}

	fn get_hash_code(&self) {
		panic!("Not supported");
	}
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum CompoundType {
	Array(Array),
	Struct(Struct),
	Map(Map),
}

impl CompoundType {
	pub fn get_stack_item(item: &CompoundType) -> Box<dyn StackItemTrait<ObjectReferences = ()>> {
		match item {
			CompoundType::Array(array) => Box::new(array),
			CompoundType::Struct(structured) => Box::new(structured),
			CompoundType::Map(map) => Box::new(map),
			// CompoundType::InteropInterface(interop_interface) => Box::new(interop_interface),
			// CompoundType::Null(null) => Box::new(null),
		}
	}
}
impl From<Array> for CompoundType {
	fn from(array: Array) -> Self {
		Self::Array(array)
	}
}

impl From<Map> for CompoundType {
	fn from(map: Map) -> Self {
		Self::Map(map)
	}
}

impl From<Struct> for CompoundType {
	fn from(_struct: Struct) -> Self {
		Self::Boolean(_struct)
	}
}

impl TryInto<Array> for CompoundType {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<Array, Self::Error> {
		match self {
			StackItem::Array(array) => Ok(array),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl TryInto<Struct> for CompoundType {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<Struct, Self::Error> {
		match self {
			StackItem::Struct(_struct) => Ok(_struct),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl TryInto<Map> for CompoundType {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<Map, Self::Error> {
		match self {
			StackItem::Map(_map) => Ok(_map),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl From<StackItem> for CompoundType {
	fn from(stack_item: StackItem) -> Self {
		match stack_item {
			StackItem::Array(array) => Self::Array(array),
			StackItem::Map(map) => Self::Map(map),
			StackItem::Struct(_struct) => Self::Struct(_struct),
			_ => panic!(),
		}
	}
}

impl Into<StackItem> for CompoundType {
	fn into(self) -> StackItem {
		match self {
			Self::Array(array) => array.into(),
			Self::Map(map) => map.into(),
			Self::Struct(_struct) => _struct.into(),
		}
	}
}
