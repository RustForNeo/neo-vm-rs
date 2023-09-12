use crate::{
	compound_types::{array::Array, map::Map, Struct::Struct},
	stack_item::{StackItem, StackItemTrait},
};
use std::{
	cell::{Ref, RefCell},
	hash::Hash,
	num::TryFromIntError,
};

pub trait CompoundTypeTrait: StackItemTrait {
	fn count(&self) -> usize;
	fn sub_items(&self) -> Vec<Ref<RefCell<StackItem>>>;
	fn sub_items_count(&self) -> usize;
	fn read_only(&self);
	fn is_read_only(&self) -> bool {
		false
	}

	fn clear(&mut self);
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum CompoundType {
	VMArray(Array),
	VMStruct(Struct),
	VMMap(Map),
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
