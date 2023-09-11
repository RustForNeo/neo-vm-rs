use crate::{
	array::Array,
	compound_type::{CompoundType, CompoundTypeTrait},
	execution_engine_limits::ExecutionEngineLimits,
	reference_counter::ReferenceCounter,
	stack_item::{ObjectReferenceEntry, StackItem, StackItemTrait},
	stack_item_type::StackItemType,
};
use std::{
	cell::RefCell,
	collections::{HashMap, VecDeque},
	fmt::{Debug, Formatter},
	hash::{Hash, Hasher},
	rc::Rc,
};

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct Struct {
	stack_references: u32,
	object_references: RefCell<Option<HashMap<CompoundType, ObjectReferenceEntry>>>,
	dfn: isize,
	low_link: usize,
	on_stack: bool,
	array: Vec<StackItem>,
}

impl Struct {
	/// Create a structure with the specified fields
	pub fn new(fields: Option<Vec<StackItem>>) -> Self {
		Self {
			stack_references: 0,
			object_references: RefCell::new(None),
			dfn: 0,
			low_link: 0,
			on_stack: false,
			array: fields.unwrap_or_default(),
		}
	}

	/// Create a new structure with the same content as this structure.
	/// All nested structures will be copied by value.
	pub fn clone(&self, limits: &ExecutionEngineLimits) -> Self {
		let mut result = Struct::new(None);
		let mut queue = VecDeque::new();
		queue.push_back(&result);
		queue.push_back(self);

		while !queue.is_empty() {
			if limits.stack_size == 0 {
				panic!("Beyond clone limits!");
			}

			let a = queue.pop_front().unwrap();
			let b = queue.pop_front().unwrap();

			for item in &b.fields {
				limits.stack_size -= 1;

				match item {
					StackItem::Struct(s) => {
						let mut sa = Struct::new(None);
						a.fields.push(&sa);
						queue.push_back(&sa);
						queue.push_back(&s);
					},
					_ => {
						a.fields.push(item.clone());
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
			object_references: self.object_references.clone(),
			dfn: self.dfn,
			low_link: self.low_link,
			on_stack: self.on_stack,
			array: self.array.clone(),
		}
	}

	/// Compare this struct to another for equality
	pub fn equals(&self, other: &Struct, limits: &ExecutionEngineLimits) -> bool {
		let mut stack1 = VecDeque::new();
		let mut stack2 = VecDeque::new();

		stack1.push_back(self);
		stack2.push_back(other);

		while !stack1.is_empty() {
			if limits.stack_size == 0 {
				panic!("Too many struct items to compare");
			}
			limits.stack_size -= 1;

			let a = stack1.pop_front().unwrap();
			let b = stack2.pop_front().unwrap();

			match (a, b) {
				(StackItem::ByteString(a), StackItem::ByteString(b)) =>
					if a != b {
						return false
					},
				(StackItem::Struct(sa), StackItem::Struct(sb)) => {
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

			if limits.comparable_size == 0 {
				panic!("The operand exceeds the maximum comparable size");
			}
			limits.comparable_size -= 1;
		}

		true
	}
}

impl StackItemTrait for Struct {
	type ObjectReferences = RefCell<Option<HashMap<CompoundType, ObjectReferenceEntry>>>;

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

	fn deep_copy(&self, ref_map: &HashMap<StackItem, StackItem>, as_immutable: bool) -> StackItem {
		todo!()
	}

	fn get_boolean(&self) -> bool {
		todo!()
	}

	fn get_slice(&self) -> &[u8] {
		todo!()
	}

	fn get_type(&self) -> StackItemType {
		StackItemType::Struct
	}
}

impl CompoundTypeTrait for Struct {
	fn reference_counter(&self) -> Option<&ReferenceCounter> {
		todo!()
	}

	fn count(&self) -> usize {
		todo!()
	}

	fn sub_items(&self) -> Vec<&StackItem> {
		todo!()
	}

	fn sub_items_count(&self) -> usize {
		todo!()
	}

	fn is_read_only(&self) -> bool {
		todo!()
	}

	fn clear(&mut self) {
		todo!()
	}

	fn deep_copy(&self, ref_map: &HashMap<&StackItem, StackItem>) -> StackItem {
		todo!()
	}
}

impl From<Array> for Struct {
	fn from(array: Array) -> Self {
		Self {
			stack_references: array.stack_references,
			object_references: array.object_references,
			dfn: array.dfn,
			low_link: array.low_link,
			on_stack: array.on_stack,
			array: array.array,
		}
	}
}

impl From<&Array> for Struct {
	fn from(array: &Array) -> Self {
		Self {
			stack_references: array.stack_references,
			object_references: array.object_references.clone(),
			dfn: array.dfn,
			low_link: array.low_link,
			on_stack: array.on_stack,
			array: array.array.clone(),
		}
	}
}
