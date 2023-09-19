use std::{cell::RefCell, collections::HashMap, convert::TryInto, hash::Hash, io::Cursor};
use std::any::Any;
use std::hash::Hasher;
use std::rc::Rc;

use crate::{
    stack_item::{ObjectReferenceEntry, StackItem},
    stack_item_type::StackItemType,
    types::{
		compound_types::compound_type::CompoundType,
		primitive_types::primitive_type::{PrimitiveType},
	},
};
use murmur3::murmur3_32;
use num_bigint::BigInt;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::execution_engine_limits::ExecutionEngineLimits;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ByteString {
	stack_references: u32,
	object_references: Rc<RefCell<Option<HashMap<CompoundType, ObjectReferenceEntry>>>>,
	dfn: isize,
	low_link: usize,
	on_stack: bool,
	bytes: Vec<u8>,
	hash: u32,
}

impl ByteString {
	pub const EMPTY: Self = Self {
		stack_references: 0,
		object_references: Rc::new(RefCell::new(None)),
		dfn: 0,
		low_link: 0,
		on_stack: false,
		bytes: Vec::new(),
		hash: 0,
	};

	pub fn new(bytes: Vec<u8>) -> Self {
		Self {
			stack_references: 0,
			object_references: Rc::new(RefCell::new(None)),
			dfn: 0,
			low_link: 0,
			on_stack: false,
			bytes,
			hash: 0,
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


impl PrimitiveType for ByteString{
	fn memory(&self) -> &[u8] {
		self.get_slice()
	}
}

impl PartialEq<dyn StackItem> for ByteString {
	fn eq(&self, other: &Self) -> bool {
		self.equals(other)
	}
}

impl Serialize for ByteString {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
		serializer.serialize_bytes(self.bytes.as_slice())
	}
}

impl Deserialize for ByteString {
	fn deserialize<'de, D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
		let bytes = Vec::<u8>::deserialize(deserializer)?;
		Ok(ByteString::new(bytes))
	}
}

impl StackItem for ByteString {
	const TRUE: Self = ByteString::new(vec![1]);
	const FALSE: Self = ByteString::new(vec![0]);
	const NULL: Self = ByteString::EMPTY;

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

	fn convert_to(&self, ty: StackItemType) -> Box<dyn StackItem> {
		todo!()
	}


	fn get_slice(&self) -> &[u8] {
		self.bytes.as_slice()
	}

	fn get_hash_code(&mut self) -> u64 {
		if self.hash == 0 {
			let mut hasher = std::collections::hash_map::DefaultHasher::new();
			hasher.write(&self.bytes);
			self.hash = hasher.finish() as u32;
		}
		self.hash as u64
	}

	fn get_type(&self) -> StackItemType {
		StackItemType::ByteString
	}

	fn get_boolean(&self) -> bool {
		self.bytes.iter().all(|&x| x == 0x00)
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
		if self.bytes.len() > limits.max_comparable_size || other.get_slice().len() > limits.max_comparable_size {
			panic!("Max comparable size exceeded")
		} else {
			self.equals(other)
		}
	}

	fn from_interface(value: &dyn Any) -> Box<dyn StackItem> {
		todo!()
	}

	fn get_integer(&self) -> BigInt {
		todo!()
	}

	fn get_interface<T: Any>(&self) -> Option<&T> {
		todo!()
	}
	fn get_bytes(&self) -> &[u8] {
		&self.bytes
	}
}