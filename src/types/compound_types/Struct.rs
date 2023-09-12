use crate::{
	execution_engine_limits::ExecutionEngineLimits,
	reference_counter::ReferenceCounter,
	stack_item::{ObjectReferenceEntry, StackItem, StackItemTrait},
	stack_item_type::StackItemType,
	types::compound_types::{
		array::Array,
		compound_type::{CompoundType, CompoundTypeTrait},
	},
};
use std::{
	cell::{Ref, RefCell},
	collections::{HashMap, VecDeque},
	fmt::Debug,
	hash::Hash,
	rc::Rc,
};

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct Struct {
	reference_counter: Option<Rc<RefCell<ReferenceCounter>>>,
	stack_references: u32,
	object_references: RefCell<Option<HashMap<CompoundType, ObjectReferenceEntry>>>,
	dfn: isize,
	low_link: usize,
	on_stack: bool,
	array: Vec<Rc<RefCell<StackItem>>>,
	read_only: bool,
}

impl Struct {
	/// Create a structure with the specified fields
	pub fn new(
		fields: Option<Vec<Rc<RefCell<StackItem>>>>,
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
		queue.push_back(&result);
		queue.push_back(self);

		let mut count = limits.max_stack_size - 1;
		while !queue.is_empty() {
			let a = queue.pop_front().unwrap();
			let b = queue.pop_front().unwrap();
			for item in &b.array {
				count -= 1;

				if count == 0 {
					panic!("Beyond clone limits!");
				}
				match item {
					StackItem::VMStruct(s) => {
						let mut sa = Struct::new(None, None);
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

		while !stack1.is_empty() {
			if limits.stack_size == 0 {
				panic!("Too many struct items to compare");
			}
			limits.stack_size -= 1;

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

	fn get_boolean(&self) -> bool {
		true
	}

	fn get_slice(&self) -> &[u8] {
		todo!()
	}

	fn get_type(&self) -> StackItemType {
		StackItemType::Struct
	}

	fn equals(&self, other: &Option<StackItem>) -> bool {
		todo!()
	}
}

impl CompoundTypeTrait for Struct {
	fn count(&self) -> usize {
		self.array.len()
	}

	fn sub_items(&self) -> Vec<Ref<RefCell<StackItem>>> {
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
