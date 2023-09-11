use std::{
	cell::RefCell,
	collections::HashMap,
	convert::TryInto,
	hash::{Hash, Hasher},
	io::Cursor,
	mem::size_of,
};

use crate::{
	compound_type::{CompoundType, CompoundTypeTrait},
	get_stack_item_data,
	primitive_type::{PrimitiveType, PrimitiveTypeTrait},
	stack_item::{ObjectReferenceEntry, StackItem, StackItemTrait},
	stack_item_type::StackItemType,
};
use murmur3::murmur3_32;
use num_bigint::BigInt;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ByteString<'a> {
	stack_references: u32,
	object_references: RefCell<Option<HashMap<CompoundType<'a>, ObjectReferenceEntry<'a>>>>,
	dfn: isize,
	low_link: usize,
	on_stack: bool,
	bytes: Vec<u8>,
	hash: Option<u32>,
}

impl ByteString {
	pub const EMPTY: Self = Self {
		stack_references: 0,
		object_references: RefCell::new(None),
		dfn: 0,
		low_link: 0,
		on_stack: false,
		bytes: Vec::new(),
		hash: None,
	};

	pub fn new(bytes: Vec<u8>) -> Self {
		Self {
			stack_references: 0,
			object_references: RefCell::new(None),
			dfn: 0,
			low_link: 0,
			on_stack: false,
			bytes,
			hash: None,
		}
	}

	fn equals(&self, other: &Self) -> bool {
		self.bytes == other.bytes
	}

	fn hash(&mut self) -> u32 {
		self.hash
			.unwrap_or_else(|| murmur3_32(&mut Cursor::new(&self.bytes), 0).unwrap())
	}
}

impl<'a> StackItemTrait for ByteString {
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
		todo!()
	}

	fn convert_to(&self, ty: StackItemType) -> StackItem {
		todo!()
	}

	fn deep_copy(&self, ref_map: &HashMap<&StackItem, StackItem>, as_immutable: bool) -> StackItem {
		todo!()
	}

	fn get_boolean(&self) -> bool {
		if self.bytes.len() > 32 {
			panic!("Value overflow")
		}
		// check whether self.bytes only contains the value 0x00
		!(self.bytes.iter().all(|b| *b == 0x00) || self.bytes.is_empty())
	}

	fn get_slice(&self) -> &[u8] {
		self.bytes.as_slice()
	}

	fn get_type(&self) -> StackItemType {
		StackItemType::ByteString
	}

	fn equals(&self, other: &Option<StackItem>) -> bool {
		todo!()
	}
}

impl PrimitiveTypeTrait for ByteString {
	fn memory(&self) -> &[u8] {
		self.get_slice()
	}
}
impl TryInto<Vec<u8>> for ByteString {
	type Error = <Vec<u8> as TryInto<Vec<u8>>>::Error;

	fn try_into(self) -> Result<Vec<u8>, Self::Error> {
		Ok(self.bytes)
	}
}

impl From<Vec<u8>> for ByteString {
	fn from(bytes: Vec<u8>) -> Self {
		Self::new(bytes)
	}
}

impl From<&str> for ByteString {
	fn from(text: &str) -> Self {
		text.as_bytes().to_vec().into()
	}
}

impl Into<StackItem> for ByteString {
	fn into(self) -> StackItem {
		StackItem::VMByteString(self)
	}
}

impl Into<PrimitiveType> for ByteString {
	fn into(self) -> PrimitiveType {
		PrimitiveType::VMByteString(self)
	}
}

impl From<PrimitiveType> for ByteString {
	fn from(ty: PrimitiveType) -> Self {
		match ty {
			PrimitiveType::VMByteString(b) => b,
			_ => panic!(),
		}
	}
}

impl PartialEq<StackItem> for ByteString {
	fn eq(&self, other: &StackItem) -> bool {
		if other.get_type() != StackItemType::ByteString {
			return false
		}
		self.equals(other.try_into().unwrap())
	}
}
