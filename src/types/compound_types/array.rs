use crate::{
	reference_counter::ReferenceCounter,
	stack_item::{ObjectReferenceEntry, StackItem},
	stack_item_type::StackItemType,
	types::compound_types::{
		compound_type::{CompoundType},
		Struct::Struct,
	},
};
use std::{
	cell::{Ref, RefCell},
	collections::HashMap,
	fmt::Debug,
	hash::Hash,
	ops::Index,
	rc::Rc,
};
use std::any::Any;
use num_bigint::BigInt;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeSeq;
use crate::execution_engine_limits::ExecutionEngineLimits;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default, PartialOrd, Ord)]
pub struct Array {
	pub stack_references: u32,
	pub reference_counter: Option<Rc<RefCell<ReferenceCounter>>>,
	pub object_references: RefCell<Option<HashMap<dyn CompoundType, ObjectReferenceEntry>>>,
	pub dfn: isize,
	pub low_link: usize,
	pub on_stack: bool,
	pub array: Vec<Rc<RefCell<dyn StackItem>>>,
	pub read_only: bool,
}

impl Index<usize> for Array {
	type Output = Rc<RefCell<dyn StackItem>>;

	fn index(&self, index: usize) -> &Self::Output {
		&self.array[index]
	}
}

impl Array {
	pub fn new(
		items: Option<Vec<Rc<RefCell<dyn StackItem>>>>,
		reference_counter: Option<Rc<RefCell<ReferenceCounter>>>,
	) -> Self {
		let items = items.unwrap_or_default();
		Self {
			stack_references: 0,
			reference_counter,
			object_references: RefCell::new(None),
			dfn: 0,
			low_link: 0,
			on_stack: false,
			array: items,
			read_only: false,
		}
	}

	pub fn add(&mut self, item: Rc<RefCell<dyn StackItem>>) {
		self.array.push(item);
	}

	pub fn clear(&mut self) {
		self.array.clear();
	}

	pub fn convert_to(&self, ty: StackItemType) -> Box<dyn StackItem> {
		match ty {
			StackItemType::Array => self.clone().into(),
			StackItemType::Struct => Struct::from(self).into(),
			_ => self.clone().into(),
		}
	}

	pub fn deep_copy(&self, map: &mut HashMap<&dyn StackItem, dyn StackItem>) -> Box<dyn StackItem> {
		if let Some(item) = map.get(self.into()) {
			return item.clone()
		}

		let mut result = Array::new(None, None);
		map.insert(self.into(), result.clone().into());

		for item in &self.array {
			result.add(item.deep_copy(map));
		}

		result.into()
	}

	pub fn iter(&self) -> std::slice::Iter<Rc<RefCell<dyn StackItem>>> {
		self.array.iter()
	}

	pub fn remove_at(&mut self, index: usize) {
		self.array.remove(index);
	}

	pub fn reverse(&mut self) {
		self.array.reverse();
	}
}

impl Clone for Array {
	fn clone(&self) -> Self {
		todo!()
	}
}

impl PartialEq<Self> for Array {
	fn eq(&self, other: &Self) -> bool {
		todo!()
	}
}

impl Serialize for Array {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
		let mut seq = serializer.serialize_seq(Some(self.array.len()))?;
		for item in self.array.iter() {
			seq.serialize_element(item)?;
		}
		seq.end()
	}
}

impl Deserialize for Array {
	fn deserialize<'de, D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
		let items = Vec::<Rc<RefCell<dyn StackItem>>>::deserialize(deserializer)?;
		Ok(Array::new(Some(items), None))
	}
}

impl StackItem for Array {
	const TRUE: Self = Array::new(None, None);

	const FALSE: Self = Array::new(None, None);

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

	fn convert_to(&self, ty: StackItemType) -> Box<dyn StackItem> {
		if self.get_type() == StackItemType::Array && ty == StackItemType::Struct {
			return StackItem::from(Struct::new(
				Some(self.array.clone()),
				self.reference_counter.clone(),
			))
		}

		self.convert_to(ty)?
	}

	fn get_slice(&self) -> &[u8] {
		panic!("Cannot get slice of array")
	}

	fn get_type(&self) -> StackItemType {
		StackItemType::Array
	}
	fn get_boolean(&self) -> bool {
		true
	}
	fn deep_copy(&self, asImmutable: bool) -> Box<dyn StackItem> {
		let result = if let StackItem::VMStruct(_) = self {
			StackItem::VMStruct(Struct::new(None, self.reference_counter.clone()))
		} else {
			StackItem::VMArray(Array::new(None, self.reference_counter.clone()))
		};

		for item in self.array.iter() {
			result.as_array_mut().push(item.deep_copy(asImmutable));
		}

		if asImmutable {
			result.make_read_only();
		}

		result
	}

	fn deep_copy_with_ref_map(
		&self,
		ref_map: &HashMap<&dyn StackItem, &dyn StackItem>,
		asImmutable: bool,
	) -> Box<dyn StackItem> {
		let result = if let StackItemType::Struct = self.get_type() {
			StackItem::VMStruct(Struct::new(None, self.reference_counter.clone()))
		} else {
			StackItem::VMArray(Array::new(None, self.reference_counter.clone()))
		};

		for item in self.array.iter() {
			result.as_array_mut().push(item.deep_copy_with_reference_map(ref_map, asImmutable));
		}

		if asImmutable {
			result.make_read_only();
		}

		result
	}

	fn equals(&self, other: &Option<dyn StackItem>) -> bool {
		if let Some(other) = other {
			if self.array.len() != other.as_array().len() {
				return false;
			}
			for i in 0..self.array.len() {
				if !self.array[i].equals(&other.as_array()[i]) {
					return false;
				}
			}
			true
		} else {
			false
		}
	}

	fn equals_with_limits(&self, other: &dyn StackItem, limits: &ExecutionEngineLimits) -> bool {
		if self.array.len() > limits.max_comparable_size || other.as_array().len() > limits.max_comparable_size {
			panic!("Max comparable size exceeded")
		} else {
			self.equals(other)
		}
	}

	fn get_integer(&self) -> BigInt {
		panic!("Cannot get integer from array");
	}

	fn get_bytes(&self) -> &[u8] {
		panic!("Cannot get bytes from array");
	}
}

impl CompoundType for Array {
	fn count(&self) -> usize {
		self.array.len()
	}

	fn sub_items(&self) -> Vec<Ref<RefCell<dyn StackItem>>> {
		self.array.iter().collect()
	}

	fn sub_items_count(&self) -> usize {
		self.array.len()
	}

	fn read_only(&mut self) {
		self.read_only = true
	}

	fn is_read_only(&self) -> bool {
		self.read_only
	}

	fn clear(&mut self) {
		if self.read_only {
			panic!("Cannot clear read-only array")
		}
		if self.reference_counter.is_some() {
			for item in self.array.iter() {
				self.reference_counter
					.unwrap()
					.borrow_mut()
					.remove_stack_reference(item.clone());
			}
		}
		self.array.clear();
	}
}


impl Clone for Array {
	fn clone(&mut self) -> Self {
		let result = if let StackItem::VMStruct(_) = self {
			StackItem::VMStruct(Struct::new(None, self.reference_counter.clone()))
		} else {
			StackItem::VMArray(Array::new(None, self.reference_counter.clone()))
		};

		self.array.append( result.clone());

		for item in self.array.iter() {
			result.as_array_mut().push(item.clone());
		}

		// if as_immutable {
		// 	result.make_read_only();
		// }

		Self {
			stack_references: self.stack_references,
			reference_counter: self.reference_counter.clone(),
			object_references: self.object_references.clone(),
			dfn: self.dfn,
			low_link: self.low_link,
			on_stack: self.on_stack,
			array: self.array.clone(),
			read_only: false,
		}
	}
}
