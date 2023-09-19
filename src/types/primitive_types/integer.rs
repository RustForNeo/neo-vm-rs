use crate::{
	stack_item::{ObjectReferenceEntry, StackItem,},
	stack_item_type::StackItemType,
	types::{
		compound_types::compound_type::CompoundType,
		primitive_types::primitive_type::{PrimitiveType,},
	},
};
use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::{
	cell::RefCell,
	collections::HashMap,
	convert::TryFrom,
	fmt::Debug,
	hash::Hash,
	ops::{Add, Div, Mul, Rem, Sub},
};
use std::any::Any;
use std::rc::Rc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::execution_engine_limits::ExecutionEngineLimits;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default, Copy)]
pub struct Integer {
	stack_references: u32,
	object_references: Rc<RefCell<Option<HashMap<CompoundType, ObjectReferenceEntry>>>>,
	dfn: isize,
	low_link: usize,
	on_stack: bool,
	value: BigInt,
}

impl Integer {
	const MAX_SIZE: u32 = 32;

	pub(crate) fn new(value: &BigInt) -> Self {
		let size = value.to_bytes().len() as u32;
		if size > Self::MAX_SIZE {
			panic!("Max size exceeded: {}", size);
		}

		Self {
			stack_references: 0,
			object_references: Rc::new(RefCell::new(None)),
			dfn: 0,
			low_link: 0,
			on_stack: false,
			value: value.clone(),
		}
	}
}

// Conversions

impl TryFrom<&[u8]> for Integer {
	type Error = ();

	fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
		BigInt::from_bytes(value).map(|v| Integer::new(&v)).map_err(|_| ())
	}
}

impl From<bool> for Integer {
	fn from(value: bool) -> Self {
		let int_val = if value { BigInt::one() } else { BigInt::zero() };

		Integer::new(&int_val)
	}
}

macro_rules! from_primitive {
	($t:ty) => {
		impl From<$t> for Integer {
			fn from(value: $t) -> Self {
				Integer::new(&BigInt::from(value))
			}
		}
	};
}

from_primitive!(i8);
from_primitive!(u8);
from_primitive!(i16);
from_primitive!(u16);
from_primitive!(i32);
from_primitive!(u32);
from_primitive!(i64);
from_primitive!(u64);
from_primitive!(isize);
from_primitive!(usize);

impl Add for Integer {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		let result = self.value + other.value;
		Integer::new(&result)
	}
}

impl Sub for Integer {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		let result = self.value - other.value;
		Integer::new(&result)
	}
}

impl Mul for Integer {
	type Output = Self;

	fn mul(self, other: Self) -> Self {
		let result = self.value * other.value;
		Integer::new(&result)
	}
}

impl Div for Integer {
	type Output = Self;

	fn div(self, other: Self) -> Self {
		let result = self.value / other.value;
		Integer::new(&result)
	}
}

impl Rem for Integer {
	type Output = Self;

	fn rem(self, other: Self) -> Self {
		let result = self.value % other.value;
		Integer::new(&result)
	}
}

impl PartialEq<dyn StackItem> for Integer {
	fn eq(&self, other: &Self) -> bool {
		self.equals(other)
	}
}

impl Serialize for Integer {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
		serializer.serialize_bytes(self.memory())
	}
}

impl Deserialize for Integer {
	fn deserialize<'de, D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
		let bytes = Vec::<u8>::deserialize(deserializer)?;
		Integer::try_from(bytes.as_slice()).map_err(|_| serde::de::Error::custom("Invalid integer"))
	}
}


// fn memory(&self) -> Vec<u8> {
// 	if self.value.is_zero() {
// 		Vec::new()
// 	} else {
// 		self.value.to_bytes()
// 	}
// }
//
// fn get_boolean(&self) -> bool {
// 	!self.value.is_zero()
// }
//
// fn equals(&self, other: &Integer) -> bool {
// 	self.value == other.value
// }
//
// fn get_integer(&self) -> &BigInt {
// 	&self.value
// }
impl StackItem for Integer {
	const TRUE: Self = Integer::new(&BigInt::one());

	const FALSE: Self = Integer::new(&BigInt::zero());

	const NULL: Self = Default::default();

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

	fn get_slice(&self) -> &[u8] {

		if self.value.is_zero() {
		Vec::new().as_slice()
	} else {
		self.value.to_signed_bytes_be().as_slice()
	}

		// self.value.to_signed_bytes_be().as_slice()
	}

	fn get_type(&self) -> StackItemType {
		StackItemType::Integer
	}
	fn get_boolean(&self) -> bool {
		!self.value.is_zero()
	}
	fn deep_copy(&self, asImmutable: bool) -> Box<dyn StackItem> {
		todo!()
	}

	fn deep_copy_with_ref_map(&self, ref_map: &HashMap<&dyn StackItem, &dyn StackItem>, asImmutable: bool) -> Box<dyn StackItem> {
		todo!()
	}

	fn equals(&self, other: &Option<dyn StackItem>) -> bool {
		if other.get_type() != StackItemType::Integer {
			return false;
		}
		self ==other || other.get_integer() == self.value
	}

	fn equals_with_limits(&self, other: &dyn StackItem, limits: &ExecutionEngineLimits) -> bool {
		todo!()
	}

	fn get_integer(&self) -> BigInt {
		self.value.clone()
	}

	fn get_interface<T: Any>(&self) -> Option<&T> {
		todo!()
	}

	fn get_bytes(&self) -> &[u8] {
		todo!()
	}
}

impl PrimitiveType for Integer {
	fn memory(&self) -> &[u8] {
		self.get_slice()
	}
}

impl Into<dyn StackItem> for Integer {
	fn into(self) -> Box<dyn StackItem> {
		StackItem::VMInteger(self)
	}
}

impl Into<dyn PrimitiveType> for Integer {
	fn into(self) -> Box<dyn PrimitiveType> {
		PrimitiveType::VMInteger(self)
	}
}

impl From<dyn PrimitiveType> for Integer {
	fn from(value: &dyn PrimitiveType) -> Self {
		match value {
			PrimitiveType::VMInteger(i) => i,
			_ => panic!("Invalid cast"),
		}
	}
}
