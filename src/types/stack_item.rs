use crate::{
	array::Array, boolean::Boolean, buffer::Buffer, byte_string::ByteString,
	compound_type::CompoundType, integer::Integer, interop_interface::InteropInterface, map::Map,
	null::Null, primitive_type::PrimitiveType, stack_item_type::StackItemType, Struct::Struct,
};
use num_bigint::BigInt;
use std::{
	cell::RefCell,
	collections::{HashMap, HashSet},
	fmt::{Debug, Formatter},
	hash::{Hash, Hasher},
	num::TryFromIntError,
	string::FromUtf8Error,
};

pub trait StackItemTrait: Debug + Hash + Eq {
	type ObjectReferences = RefCell<Option<HashMap<CompoundType, ObjectReferenceEntry>>>;

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

	fn successors(&self) -> Vec<StackItem> {
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

	fn convert_to(&self, ty: StackItemType) -> StackItem {
		if ty == self.get_type() {
			return self.into()
		}
		if ty == StackItemType::Boolean {
			return self.get_boolean().into()
		}
		panic!("Not implemented")
	}
	fn deep_copy(&self, ref_map: &HashMap<StackItem, StackItem>, as_immutable: bool) -> StackItem;

	fn get_boolean(&self) -> bool;

	fn get_integer(&self) -> Result<BigInt, TryFromIntError> {
		Err(TryFromIntError::from(TryFromIntError))
	}

	fn get_interface<T: 'static>(&self) -> Result<&T, TryFromIntError> {
		Err(TryFromIntError::from(TryFromIntError))
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
}

pub struct ObjectReferenceEntry {
	item: StackItem,
	references: i32,
}

impl ObjectReferenceEntry {
	pub fn new(item: StackItem) -> Self {
		Self { item, references: 0 }
	}
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum StackItem {
	Buffer(Buffer),
	ByteString(ByteString),
	Boolean(Boolean),
	Integer(Integer),
	Array(Array),
	Struct(Struct),
	Map(Map),
	InteropInterface(InteropInterface),
	Null(Null),
}

impl StackItem {
	pub fn get_stack_item(item: &StackItem) -> Box<dyn StackItemTrait<ObjectReferences = ()>> {
		match item {
			StackItem::Buffer(buffer) => Box::new(buffer),
			StackItem::ByteString(byte_string) => Box::new(byte_string),
			StackItem::Boolean(boolean) => Box::new(boolean),
			StackItem::Integer(integer) => Box::new(integer),
			StackItem::Array(array) => Box::new(array),
			StackItem::Struct(structured) => Box::new(structured),
			StackItem::Map(map) => Box::new(map),
			StackItem::InteropInterface(interop_interface) => Box::new(interop_interface),
			StackItem::Null(null) => Box::new(null),
		}
	}
}

#[macro_export]
macro_rules! get_stack_item_data {
	($item:expr) => {
		match $item {
			StackItem::Buffer(buffer) => buffer,
			StackItem::ByteString(byte_string) => byte_string,
			StackItem::Boolean(boolean) => boolean,
			StackItem::Integer(integer) => integer,
			StackItem::Array(array) => array,
			StackItem::Struct(structured) => structured,
			StackItem::Map(map) => map,
			StackItem::InteropInterface(interop_interface) => interop_interface,
			StackItem::Null(null) => null,
		}
	};
}

impl From<Buffer> for StackItem {
	fn from(buffer: Buffer) -> Self {
		Self::Buffer(buffer)
	}
}

impl From<ByteString> for StackItem {
	fn from(byte_string: ByteString) -> Self {
		Self::ByteString(byte_string)
	}
}

impl From<Boolean> for StackItem {
	fn from(boolean: Boolean) -> Self {
		Self::Boolean(boolean)
	}
}

impl From<Integer> for StackItem {
	fn from(integer: Integer) -> Self {
		Self::Integer(integer)
	}
}

impl From<Array> for StackItem {
	fn from(array: Array) -> Self {
		Self::Array(array)
	}
}

impl From<Struct> for StackItem {
	fn from(structured: Struct) -> Self {
		Self::Struct(structured)
	}
}

impl From<Map> for StackItem {
	fn from(map: Map) -> Self {
		Self::Map(map)
	}
}

impl From<InteropInterface> for StackItem {
	fn from(interop_interface: InteropInterface) -> Self {
		Self::InteropInterface(interop_interface)
	}
}

impl From<Null> for StackItem {
	fn from(null: Null) -> Self {
		Self::Null(null)
	}
}

impl TryInto<Buffer> for StackItem {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<Buffer, Self::Error> {
		match self {
			StackItem::Buffer(buffer) => Ok(buffer),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl TryInto<ByteString> for StackItem {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<ByteString, Self::Error> {
		match self {
			StackItem::ByteString(byte_string) => Ok(byte_string),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl TryInto<Boolean> for StackItem {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<Boolean, Self::Error> {
		match self {
			StackItem::Boolean(boolean) => Ok(boolean),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl TryInto<Integer> for StackItem {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<Integer, Self::Error> {
		match self {
			StackItem::Integer(integer) => Ok(integer),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl TryInto<Array> for StackItem {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<Array, Self::Error> {
		match self {
			StackItem::Array(array) => Ok(array),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl TryInto<Struct> for StackItem {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<Struct, Self::Error> {
		match self {
			StackItem::Struct(structured) => Ok(structured),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl TryInto<Map> for StackItem {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<Map, Self::Error> {
		match self {
			StackItem::Map(map) => Ok(map),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl TryInto<InteropInterface> for StackItem {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<InteropInterface, Self::Error> {
		match self {
			StackItem::InteropInterface(interop_interface) => Ok(interop_interface),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl TryInto<Null> for StackItem {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<Null, Self::Error> {
		match self {
			StackItem::Null(null) => Ok(null),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl TryInto<PrimitiveType> for StackItem {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<PrimitiveType, Self::Error> {
		match self {
			StackItem::Buffer(buffer) => Ok(buffer.into()),
			StackItem::ByteString(byte_string) => Ok(byte_string.into()),
			StackItem::Boolean(boolean) => Ok(boolean.into()),
			StackItem::Integer(integer) => Ok(integer.into()),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl Into<StackItem> for bool {
	fn into(self) -> StackItem {
		StackItem::Boolean(Boolean::new(self))
	}
}
