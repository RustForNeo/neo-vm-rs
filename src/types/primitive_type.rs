use crate::{
	boolean::Boolean,
	buffer::Buffer,
	byte_string::ByteString,
	integer::Integer,
	stack_item::{StackItem, StackItemTrait},
	stack_item_type::StackItemType,
};
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
	Buffer(Buffer),
	ByteString(ByteString),
	Boolean(Boolean),
	Integer(Integer),
}

impl PrimitiveType {
	pub fn get_stack_item(item: &PrimitiveType) -> Box<dyn StackItemTrait<ObjectReferences = ()>> {
		match item {
			PrimitiveType::Buffer(buffer) => Box::new(buffer),
			PrimitiveType::ByteString(byte_string) => Box::new(byte_string),
			PrimitiveType::Boolean(boolean) => Box::new(boolean),
			PrimitiveType::Integer(integer) => Box::new(integer),
		}
	}
}

impl From<Buffer> for PrimitiveType {
	fn from(buffer: Buffer) -> Self {
		Self::Buffer(buffer)
	}
}

impl From<ByteString> for PrimitiveType {
	fn from(byte_string: ByteString) -> Self {
		Self::ByteString(byte_string)
	}
}

impl From<Boolean> for PrimitiveType {
	fn from(boolean: Boolean) -> Self {
		Self::Boolean(boolean)
	}
}

impl From<Integer> for PrimitiveType {
	fn from(integer: Integer) -> Self {
		Self::Integer(integer)
	}
}

impl TryInto<Buffer> for PrimitiveType {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<Buffer, Self::Error> {
		match self {
			StackItem::Buffer(buffer) => Ok(buffer),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl TryInto<ByteString> for PrimitiveType {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<ByteString, Self::Error> {
		match self {
			StackItem::ByteString(byte_string) => Ok(byte_string),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl TryInto<Boolean> for PrimitiveType {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<Boolean, Self::Error> {
		match self {
			StackItem::Boolean(boolean) => Ok(boolean),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl TryInto<Integer> for PrimitiveType {
	type Error = TryFromIntError;

	fn try_into(self) -> Result<Integer, Self::Error> {
		match self {
			StackItem::Integer(integer) => Ok(integer),
			_ => Err(TryFromIntError::from(TryFromIntError)),
		}
	}
}

impl From<StackItem> for PrimitiveType {
	fn from(stack_item: StackItem) -> Self {
		match stack_item {
			StackItem::Buffer(buffer) => Self::Buffer(buffer),
			StackItem::ByteString(byte_string) => Self::ByteString(byte_string),
			StackItem::Boolean(boolean) => Self::Boolean(boolean),
			StackItem::Integer(integer) => Self::Integer(integer),
			_ => panic!(),
		}
	}
}

impl Into<StackItem> for PrimitiveType {
	fn into(self) -> StackItem {
		match self {
			Self::Buffer(buffer) => buffer.into(),
			Self::ByteString(byte_string) => byte_string.into(),
			Self::Boolean(boolean) => boolean.into(),
			Self::Integer(integer) => integer.into(),
		}
	}
}
