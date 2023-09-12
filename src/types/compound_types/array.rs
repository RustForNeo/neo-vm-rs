use crate::{
	reference_counter::ReferenceCounter,
	stack_item::{ObjectReferenceEntry, StackItem, StackItemTrait},
	stack_item_type::StackItemType,
	types::compound_types::{
		compound_type::{CompoundType, CompoundTypeTrait},
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

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default, PartialOrd, Ord)]
pub struct Array {
	pub stack_references: u32,
	pub reference_counter: Option<Rc<RefCell<ReferenceCounter>>>,
	pub object_references: RefCell<Option<HashMap<CompoundType, ObjectReferenceEntry>>>,
	pub dfn: isize,
	pub low_link: usize,
	pub on_stack: bool,
	pub array: Vec<Rc<RefCell<StackItem>>>,
	pub read_only: bool,
}

impl Index<usize> for Array {
	type Output = Rc<RefCell<StackItem>>;

	fn index(&self, index: usize) -> &Self::Output {
		&self.array[index]
	}
}

impl Array {
	pub fn new(
		items: Option<Vec<Rc<RefCell<StackItem>>>>,
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

	pub fn add(&mut self, item: Rc<RefCell<StackItem>>) {
		self.array.push(item);
	}

	pub fn clear(&mut self) {
		self.array.clear();
	}

	pub fn convert_to(&self, ty: StackItemType) -> StackItem {
		match ty {
			StackItemType::Array => self.clone().into(),
			StackItemType::Struct => Struct::from(self).into(),
			_ => self.clone().into(),
		}
	}

	pub fn deep_copy(&self, map: &mut HashMap<&StackItem, StackItem>) -> StackItem {
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

	pub fn iter(&self) -> std::slice::Iter<Rc<RefCell<StackItem>>> {
		self.array.iter()
	}

	pub fn remove_at(&mut self, index: usize) {
		self.array.remove(index);
	}

	pub fn reverse(&mut self) {
		self.array.reverse();
	}
}

impl StackItemTrait for Array {
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
		if self.get_type() == StackItemType::Array && ty == StackItemType::Struct {
			return StackItem::from(Struct::new(
				Some(self.array.clone()),
				self.reference_counter.clone(),
			))
		}

		StackItemTrait::convert_to(&self, ty)
	}

	fn get_boolean(&self) -> bool {
		true
	}

	fn get_slice(&self) -> &[u8] {
		panic!("Cannot get slice of array")
	}

	fn get_type(&self) -> StackItemType {
		StackItemType::Array
	}

	fn equals(&self, other: &Option<StackItem>) -> bool {
		todo!()
	}
}

impl CompoundTypeTrait for Array {
	fn count(&self) -> usize {
		self.array.len()
	}

	fn sub_items(&self) -> Vec<Ref<RefCell<StackItem>>> {
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

impl Into<StackItem> for Array {
	fn into(self) -> StackItem {
		StackItem::VMArray(self)
	}
}

impl Clone for Array {
	fn clone(&self) -> Self {
		let result = if let StackItem::VMStruct(_) = self {
			StackItem::VMStruct(Struct::new(None, self.reference_counter.clone()))
		} else {
			StackItem::VMArray(Array::new(None, self.reference_counter.clone()))
		};

		self.ref_map.insert(self, result.clone());

		for item in self.as_array().iter() {
			result.as_array_mut().push(item.clone(ref_map, as_immutable));
		}

		if as_immutable {
			result.make_read_only();
		}

		Self {
			stack_references: self.stack_references,
			reference_counter: self.reference_counter.clone(),
			object_references: self.object_references.clone(),
			dfn: self.dfn,
			low_link: self.low_link,
			on_stack: self.on_stack,
			array: self.array.clone(),
		}
	}
}
