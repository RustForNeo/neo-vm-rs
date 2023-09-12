use crate::{
	buffer::Buffer,
	stack_item::{StackItem, StackItemTrait},
	stack_item_type::StackItemType,
	types::primitive_types::{boolean::Boolean, byte_string::ByteString, integer::Integer},
};
use num_bigint::BigInt;
use std::{collections::HashMap, convert::TryInto, num::TryFromIntError, vec::Vec};

pub trait PrimitiveTypeTrait: StackItemTrait + Clone {
	fn memory(&self) -> &[u8];

	/// The size of the vm object in bytes.
	fn size(&self) -> usize {
		self.memory().len()
	}

	fn convert_to(&self, ty: StackItemType) -> StackItem {
		match ty {
			StackItemType::Integer => Integer::from(self.get_integer()).into(),
			StackItemType::ByteString => ByteString::from(self.memory()).into(),
			StackItemType::Buffer => Buffer::from(self.memory()).into(),
			StackItemType::Boolean => Boolean::from(self.get_boolean()).into(),
			_ => panic!(), //self.base_convert_to(ty),
		}
	}

	fn deep_copy(
		&self,
		_ref_map: &HashMap<&StackItem, StackItem>,
		_as_immutable: bool,
	) -> StackItem {
		self.clone().into()
	}

	fn get_slice(&self) -> &[u8] {
		self.memory()
	}
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum PrimitiveType {
	VMBuffer(Buffer),
	VMByteString(ByteString),
	VMBoolean(Boolean),
	VMInteger(Integer),
}

impl PrimitiveType {
	pub fn get_stack_item(item: &PrimitiveType) -> Box<dyn StackItemTrait<ObjectReferences = ()>> {
		match item {
			PrimitiveType::VMBuffer(buffer) => Box::new(buffer),
			PrimitiveType::VMByteString(byte_string) => Box::new(byte_string),
			PrimitiveType::VMBoolean(boolean) => Box::new(boolean),
			PrimitiveType::VMInteger(integer) => Box::new(integer),
		}
	}

	pub fn get_integer(&self) -> BigInt {
		match self {
			StackItem::VMInteger(integer) => integer.get_integer().unwrap(),
			StackItem::VMBoolean(boolean) => boolean.get_integer().unwrap(),
			StackItem::VMByteString(byte_string) => byte_string.get_integer().unwrap(),
			StackItem::VMBuffer(buffer) => buffer.get_integer().unwrap(),
			StackItem::VMPointer(pointer) => pointer.get_integer().unwrap(),
			_ => panic!("Not implemented"),
		}
	}
}

impl From<Buffer> for PrimitiveType {
	fn from(buffer: Buffer) -> Self {
		Self::VMBuffer(buffer)
	}
}

impl From<ByteString> for PrimitiveType {
	fn from(byte_string: ByteString) -> Self {
		Self::VMByteString(byte_string)
	}
}

impl From<Boolean> for PrimitiveType {
	fn from(boolean: Boolean) -> Self {
		Self::VMBoolean(boolean)
	}
}

impl From<Integer> for PrimitiveType {
	fn from(integer: Integer) -> Self {
		Self::VMInteger(integer)
	}
}

impl TryInto<Buffer> for PrimitiveType {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<Buffer, Self::Error> {
		match self {
			StackItem::VMBuffer(buffer) => Ok(buffer),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl TryInto<ByteString> for PrimitiveType {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<ByteString, Self::Error> {
		match self {
			StackItem::VMByteString(byte_string) => Ok(byte_string),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl TryInto<Boolean> for PrimitiveType {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<Boolean, Self::Error> {
		match self {
			StackItem::VMBoolean(boolean) => Ok(boolean),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl TryInto<Integer> for PrimitiveType {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<Integer, Self::Error> {
		match self {
			StackItem::VMInteger(integer) => Ok(integer),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl From<StackItem> for PrimitiveType {
	fn from(stack_item: StackItem) -> Self {
		match stack_item {
			StackItem::VMBuffer(buffer) => Self::VMBuffer(buffer),
			StackItem::VMByteString(byte_string) => Self::VMByteString(byte_string),
			StackItem::VMBoolean(boolean) => Self::VMBoolean(boolean),
			StackItem::VMInteger(integer) => Self::VMInteger(integer),
			_ => panic!(),
		}
	}
}

impl Into<StackItem> for PrimitiveType {
	fn into(self) -> StackItem {
		match self {
			Self::VMBuffer(buffer) => buffer.into(),
			Self::VMByteString(byte_string) => byte_string.into(),
			Self::VMBoolean(boolean) => boolean.into(),
			Self::VMInteger(integer) => integer.into(),
		}
	}
}
