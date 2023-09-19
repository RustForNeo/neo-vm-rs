use crate::{
	execution_engine_limits::ExecutionEngineLimits,
	reference_counter::ReferenceCounter,
	stack_item::{ObjectReferenceEntry, StackItem},
	stack_item_type::StackItemType,
	types::compound_types::{
		array::Array,
		compound_type::{CompoundType},
	},
};
use std::{
	cell::{Ref, RefCell},
	collections::{HashMap, VecDeque},
	fmt::Debug,
	hash::Hash,
	rc::Rc,
};
use num_bigint::BigInt;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct Struct {
	reference_counter: Option<Rc<RefCell<ReferenceCounter>>>,
	stack_references: u32,
	object_references: RefCell<Option<HashMap<dyn CompoundType, ObjectReferenceEntry>>>,
	dfn: isize,
	low_link: usize,
	on_stack: bool,
	array: Vec<Rc<RefCell<dyn StackItem>>>,
	read_only: bool,
}

impl Struct {
	/// Create a structure with the specified fields
	pub fn new(
		fields: Option<Vec<Rc<RefCell<dyn StackItem>>>>,
		reference_counter: Option<Rc<RefCell<ReferenceCounter>>>,
	) -> Self {
		Self {
			reference_counter,
			stack_references: 0,
			object_references: RefCell::new(None),
			dfn: 0,
			low_link: 0,
			on_stack: false,
			array: fields.unwrap_or_default(),
			read_only: false,
		}
	}

	/// Create a new structure with the same content as this structure.
	/// All nested structures will be copied by value.
	pub fn clone(&self, limits: &ExecutionEngineLimits) -> Self {
		let mut result = Struct::new(None, self.reference_counter.clone());
		let mut queue = VecDeque::new();
		queue.push_back(&mut result);
		queue.push_back(&mut self.clone(limits));

		let mut count = limits.max_stack_size - 1;
		while !queue.is_empty() {
			let mut a = queue.pop_front().unwrap();
			let b = queue.pop_front().unwrap();
			for item in &b.array {
				count -= 1;

				if count == 0 {
					panic!("Beyond clone limits!");
				}
				match item.borrow().get_type() {
					StackItemType::Struct => {
						let mut sa = Struct::new(None, None);
						a.array.push(Rc::new(RefCell::new(sa)));
						queue.push_back(&mut sa);
						queue.push_back(&mut item.borrow());
					},
					_ => {
						a.array.push(item.clone());
					},
				}
			}
		}

		result
	}

	/// Convert this struct to an array
	pub fn to_array(&self) -> Array {
		Array {
			stack_references: self.stack_references,
			reference_counter: self.reference_counter.clone(),
			object_references: self.object_references.clone(),
			dfn: self.dfn,
			low_link: self.low_link,
			on_stack: self.on_stack,
			array: self.array.clone(),
			read_only: self.read_only,
		}
	}

	/// Compare this struct to another for equality
	pub fn equals(&self, other: &Struct, limits: &ExecutionEngineLimits) -> bool {
		let mut stack1 = VecDeque::new();
		let mut stack2 = VecDeque::new();

		stack1.push_back(self);
		stack2.push_back(other);

		let mut count = limits.max_stack_size;
		let mut maxComparableSize = limits.max_comparable_size;

		while !stack1.is_empty() {
			if count == 0 {
				panic!("Too many struct items to compare");
			}
			count -= 1;

			let a = stack1.pop_front().unwrap();
			let b = stack2.pop_front().unwrap();

			match (a, b) {
				(StackItem::VMByteString(a), StackItem::VMByteString(b)) =>
					if a != b {
						return false
					},
				(StackItem::VMStruct(sa), StackItem::VMStruct(sb)) => {
					if Rc::ptr_eq(&sa, &sb) {
						continue
					}

					if sa.fields.len() != sb.fields.len() {
						return false
					}

					for item in &sa.fields {
						stack1.push_back(item.clone());
					}

					for item in &sb.fields {
						stack2.push_back(item.clone());
					}
				},
				_ =>
					if a != b {
						return false
					},
			}

			if maxComparableSize == 0 {
				panic!("The operand exceeds the maximum comparable size");
			}
			maxComparableSize -= 1;
		}

		true
	}
}

impl Clone for Struct {
	fn clone(&self) -> Self {
		todo!()
	}
}

impl PartialEq<Self> for Struct {
	fn eq(&self, other: &Self) -> bool {

	}
}

impl Serialize for Struct {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
		serializer.serialize_bytes(self.array.as_slice())
	}
}

impl Deserialize for Struct {
	fn deserialize<'de, D>(, deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
		let bytes = Vec::<dyn StackItem>::deserialize(deserializer)?;
		Ok(Struct::new(Some(Rc::new(RefCell::new(bytes))), None);
	}
}

impl StackItem for Struct {
	const TRUE: Self = Default::default();

	const FALSE: Self = Default::default();

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
		todo!()
	}

	fn get_slice(&self) -> &[u8] {
		todo!()
	}

	fn get_type(&self) -> StackItemType {
		StackItemType::Struct
	}
	fn get_boolean(&self) -> bool {
		true
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
		todo!()
	}

	fn get_integer(&self) -> BigInt {
		todo!()
	}

	fn get_bytes(&self) -> &[u8] {
		todo!()
	}
}

impl CompoundType for Struct {
	fn count(&self) -> usize {
		self.array.len()
	}

	fn sub_items(&self) -> Vec<Ref<RefCell<dyn StackItem>>> {
		self.array.iter().collect()
	}

	fn sub_items_count(&self) -> usize {
		self.count()
	}

	fn read_only(&mut self) {
		self.read_only = true
	}

	fn is_read_only(&self) -> bool {
		self.read_only
	}

	fn clear(&mut self) {
		if self.read_only {
			panic!("Cannot clear read-only struct")
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

impl From<Array> for Struct {
	fn from(array: Array) -> Self {
		Self {
			reference_counter: array.reference_counter,
			stack_references: array.stack_references,
			object_references: array.object_references,
			dfn: array.dfn,
			low_link: array.low_link,
			on_stack: array.on_stack,
			array: array.array,
			read_only: array.read_only,
		}
	}
}

impl From<&Array> for Struct {
	fn from(array: &Array) -> Self {
		Self {
			reference_counter: array.reference_counter.clone(),
			stack_references: array.stack_references,
			object_references: array.object_references.clone(),
			dfn: array.dfn,
			low_link: array.low_link,
			on_stack: array.on_stack,
			array: array.array.clone(),
			read_only: array.read_only,
		}
	}
}

impl Clone for Struct {
	fn clone(&self) -> Self {
		let mut result = Self::new(None, self.reference_counter.clone());

		for item in &self.array {
			result.array.push(item.clone());
		}

		result
	}
}
