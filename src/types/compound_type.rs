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
pub enum CompoundType<'a> {
	VMArray(Array<'a>),
	VMStruct(Struct<'a>),
	VMMap(Map<'a>),
}

impl CompoundType {
	pub fn get_stack_item(item: &CompoundType) -> Box<dyn StackItemTrait<ObjectReferences = ()>> {
		match item {
			CompoundType::VMArray(array) => Box::new(array),
			CompoundType::VMStruct(structured) => Box::new(structured),
			CompoundType::VMMap(map) => Box::new(map),
			// CompoundType::InteropInterface(interop_interface) => Box::new(interop_interface),
			// CompoundType::Null(null) => Box::new(null),
		}
	}
}
impl From<Array> for CompoundType {
	fn from(array: Array) -> Self {
		Self::VMArray(array)
	}
}

impl From<Map> for CompoundType {
	fn from(map: Map) -> Self {
		Self::VMMap(map)
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
			StackItem::VMArray(array) => Ok(array),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl TryInto<Struct> for CompoundType {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<Struct, Self::Error> {
		match self {
			StackItem::VMStruct(_struct) => Ok(_struct),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl TryInto<Map> for CompoundType {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<Map, Self::Error> {
		match self {
			StackItem::VMMap(_map) => Ok(_map),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl From<StackItem> for CompoundType {
	fn from(stack_item: StackItem) -> Self {
		match stack_item {
			StackItem::VMArray(array) => Self::VMArray(array),
			StackItem::VMMap(map) => Self::VMMap(map),
			StackItem::VMStruct(_struct) => Self::VMStruct(_struct),
			_ => panic!(),
		}
	}
}

impl Into<StackItem> for CompoundType {
	fn into(self) -> StackItem {
		match self {
			Self::VMArray(array) => array.into(),
			Self::VMMap(map) => map.into(),
			Self::VMStruct(_struct) => _struct.into(),
		}
	}
}
