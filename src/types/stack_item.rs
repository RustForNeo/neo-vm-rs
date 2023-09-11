use crate::{
	array::Array, boolean::Boolean, buffer::Buffer, byte_string::ByteString,
	compound_type::CompoundType, integer::Integer, interop_interface::InteropInterface, map::Map,
	null::Null, pointer::Pointer, primitive_type::PrimitiveType, stack_item_type::StackItemType,
	Struct::Struct,
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

pub trait StackItemTrait<'a>: Debug + Hash + Eq {
	type ObjectReferences = RefCell<Option<HashMap<CompoundType<'a>, ObjectReferenceEntry<'a>>>>;

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
	fn equals(&self, other: &Option<StackItem>) -> bool;
}

pub struct ObjectReferenceEntry<'a> {
	item: StackItem<'a>,
	references: i32,
}

impl<'a> ObjectReferenceEntry {
	pub fn new(item: StackItem) -> Self {
		Self { item, references: 0 }
	}
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum StackItem<'a> {
	VMBuffer(Buffer<'a>),
	VMByteString(ByteString<'a>),
	VMBoolean(Boolean<'a>),
	VMInteger(Integer<'a>),
	VMPointer(Pointer<'a>),
	VMArray(Array<'a>),
	VMStruct(Struct<'a>),
	VMMap(Map<'a>),
	InteropInterface(InteropInterface<'a>),
	VMNull(Null),
}

impl StackItem {
	pub fn get_stack_item(item: &StackItem) -> Box<dyn StackItemTrait<ObjectReferences = ()>> {
		match item {
			StackItem::VMBuffer(buffer) => Box::new(buffer),
			StackItem::VMByteString(byte_string) => Box::new(byte_string),
			StackItem::VMBoolean(boolean) => Box::new(boolean),
			StackItem::VMInteger(integer) => Box::new(integer),
			StackItem::VMPointer(pointer) => Box::new(pointer),
			StackItem::VMArray(array) => Box::new(array),
			StackItem::VMStruct(structured) => Box::new(structured),
			StackItem::VMMap(map) => Box::new(map),
			StackItem::InteropInterface(interop_interface) => Box::new(interop_interface),
			StackItem::VMNull(null) => Box::new(null),
			_ => {},
		}
	}

	pub fn downcast<T: StackItemTrait>(self) -> Result<T, Self> {
		if let Some(item) = self.as_any().downcast::<T>() {
			Ok(item)
		} else {
			Err(self)
		}
	}

	pub fn equals(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::VMNull(a), Self::VMNull(b)) => true,
			(Self::VMBoolean(a), Self::VMBoolean(b)) => a == b,
			(Self::VMInteger(a), Self::VMInteger(b)) => a == b,
			(Self::VMPointer(a), Self::VMPointer(b)) => a == b,
			(Self::VMBuffer(a), Self::VMBuffer(b)) => a == b,
			(Self::VMArray(a), Self::VMArray(b)) => a == b,
			(Self::VMMap(a), Self::VMMap(b)) => a == b,
			(Self::VMStruct(a), Self::VMStruct(b)) => a == b,
			_ => false,
		}
	}

	pub fn get_integer(&self) -> BigInt {
		match self {
			StackItem::VMInteger(integer) => integer.get_integer().unwrap(),
			StackItem::VMBoolean(boolean) => boolean.get_integer().unwrap(),
			StackItem::VMBuffer(buffer) => buffer.get_integer().unwrap(),
			StackItem::VMPointer(pointer) => pointer.get_integer().unwrap(),
			StackItem::VMArray(array) => array.get_integer().unwrap(),
			StackItem::VMMap(map) => map.get_integer().unwrap(),
			StackItem::VMStruct(structured) => structured.get_integer().unwrap(),
			StackItem::InteropInterface(interop_interface) =>
				interop_interface.get_integer().unwrap(),
			StackItem::VMNull(null) => null.get_integer().unwrap(),
			_ => panic!("Not implemented"),
		}
	}

	pub fn get_bool(&self) -> bool {
		match self {
			StackItem::VMInteger(integer) => integer.get_boolean(),
			StackItem::VMBoolean(boolean) => boolean.get_boolean(),
			StackItem::VMBuffer(buffer) => buffer.get_boolean(),
			StackItem::VMPointer(pointer) => pointer.get_boolean(),
			StackItem::VMArray(array) => array.get_boolean(),
			StackItem::VMMap(map) => map.get_boolean(),
			StackItem::VMStruct(structured) => structured.get_boolean(),
			StackItem::InteropInterface(interop_interface) => interop_interface.get_boolean(),
			StackItem::VMNull(null) => null.get_boolean(),
			_ => panic!("Not implemented"),
		}
	}

	pub fn get_slice(&self) -> &[u8] {
		match self {
			StackItem::VMInteger(integer) => integer.get_slice(),
			StackItem::VMBoolean(boolean) => boolean.get_slice(),
			StackItem::VMBuffer(buffer) => buffer.get_slice(),
			StackItem::VMPointer(pointer) => pointer.get_slice(),
			StackItem::VMArray(array) => array.get_slice(),
			StackItem::VMMap(map) => map.get_slice(),
			StackItem::VMStruct(structured) => structured.get_slice(),
			StackItem::InteropInterface(interop_interface) => interop_interface.get_slice(),
			StackItem::VMNull(null) => null.get_slice(),
			_ => panic!("Not implemented"),
		}
	}
	pub fn get_item_type(&self) -> StackItemType {
		match self {
			StackItem::VMInteger(integer) => integer.get_type(),
			StackItem::VMBoolean(boolean) => boolean.get_type(),
			StackItem::VMBuffer(buffer) => buffer.get_type(),
			StackItem::VMPointer(pointer) => pointer.get_type(),
			StackItem::VMArray(array) => array.get_type(),
			StackItem::VMMap(map) => map.get_type(),
			StackItem::VMStruct(structured) => structured.get_type(),
			StackItem::InteropInterface(interop_interface) => interop_interface.get_type(),
			StackItem::VMNull(null) => null.get_type(),
			_ => panic!("Not implemented"),
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
			StackItem::Pointer(pointer) => pointer,
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
		Self::VMBuffer(buffer)
	}
}

impl From<ByteString> for StackItem {
	fn from(byte_string: ByteString) -> Self {
		Self::VMByteString(byte_string)
	}
}

impl From<Boolean> for StackItem {
	fn from(boolean: Boolean) -> Self {
		Self::VMBoolean(boolean)
	}
}

impl From<Integer> for StackItem {
	fn from(integer: Integer) -> Self {
		Self::VMInteger(integer)
	}
}

impl From<Pointer> for StackItem {
	fn from(pointer: Pointer) -> Self {
		Self::VMPointer(pointer)
	}
}

impl From<Array> for StackItem {
	fn from(array: Array) -> Self {
		Self::VMArray(array)
	}
}

impl From<Struct> for StackItem {
	fn from(structured: Struct) -> Self {
		Self::VMStruct(structured)
	}
}

impl From<Map> for StackItem {
	fn from(map: Map) -> Self {
		Self::VMMap(map)
	}
}

impl From<InteropInterface> for StackItem {
	fn from(interop_interface: InteropInterface) -> Self {
		Self::InteropInterface(interop_interface)
	}
}

impl From<Null> for StackItem {
	fn from(null: Null) -> Self {
		Self::VMNull(null)
	}
}

impl TryInto<Buffer> for StackItem {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<Buffer, Self::Error> {
		match self {
			StackItem::VMBuffer(buffer) => Ok(buffer),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl TryInto<ByteString> for StackItem {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<ByteString, Self::Error> {
		match self {
			StackItem::VMByteString(byte_string) => Ok(byte_string),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl TryInto<Boolean> for StackItem {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<Boolean, Self::Error> {
		match self {
			StackItem::VMBoolean(boolean) => Ok(boolean),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl TryInto<Integer> for StackItem {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<Integer, Self::Error> {
		match self {
			StackItem::VMInteger(integer) => Ok(integer),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl TryInto<Pointer> for StackItem {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<Pointer, Self::Error> {
		match self {
			StackItem::VMPointer(pointer) => Ok(pointer),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl TryInto<Array> for StackItem {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<Array, Self::Error> {
		match self {
			StackItem::VMArray(array) => Ok(array),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl TryInto<Struct> for StackItem {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<Struct, Self::Error> {
		match self {
			StackItem::VMStruct(structured) => Ok(structured),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl TryInto<Map> for StackItem {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<Map, Self::Error> {
		match self {
			StackItem::VMMap(map) => Ok(map),
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
			StackItem::VMNull(null) => Ok(null),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl TryInto<PrimitiveType> for StackItem {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<PrimitiveType, Self::Error> {
		match self {
			StackItem::VMBuffer(buffer) => Ok(buffer.into()),
			StackItem::VMByteString(byte_string) => Ok(byte_string.into()),
			StackItem::VMBoolean(boolean) => Ok(boolean.into()),
			StackItem::VMInteger(integer) => Ok(integer.into()),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl Into<StackItem> for bool {
	fn into(self) -> StackItem {
		StackItem::VMBoolean(Boolean::new(self))
	}
}

impl From<bool> for StackItem {
	fn from(boolean: bool) -> Self {
		StackItem::VMBoolean(Boolean::new(boolean))
	}
}

impl From<Vec<u8>> for StackItem {
	fn from(bytes: Vec<u8>) -> Self {
		StackItem::VMByteString(ByteString::new(bytes))
	}
}

impl From<&[u8]> for StackItem {
	fn from(bytes: &[u8]) -> Self {
		StackItem::VMByteString(ByteString::new(bytes.to_vec()))
	}
}

impl From<BigInt> for StackItem {
	fn from(big_int: BigInt) -> Self {
		StackItem::VMInteger(Integer::new(&big_int))
	}
}

impl From<&BigInt> for StackItem {
	fn from(big_int: &BigInt) -> Self {
		StackItem::VMInteger(Integer::new(big_int))
	}
}
