use crate::{
	compound_type::{CompoundType, CompoundTypeTrait},
	reference_counter::ReferenceCounter,
	stack_item::{ObjectReferenceEntry, StackItem, StackItemTrait},
	stack_item_type::StackItemType,
	Struct::Struct,
};
use std::{
	cell::RefCell,
	collections::HashMap,
	fmt::{Debug, Formatter},
	hash::{Hash, Hasher},
	ops::Index,
};

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default, PartialOrd, Ord)]
pub struct Array {
	pub(crate) stack_references: u32,
	pub(crate) object_references: RefCell<Option<HashMap<CompoundType, ObjectReferenceEntry>>>,
	pub(crate) dfn: isize,
	pub(crate) low_link: usize,
	pub(crate) on_stack: bool,
	pub(crate) array: Vec<StackItem>,
}

impl Index<usize> for Array {
	type Output = StackItem;

	fn index(&self, index: usize) -> &Self::Output {
		&self.array[index]
	}
}

impl Array {
	pub fn new(items: Option<Vec<StackItem>>) -> Self {
		let items = items.unwrap_or_default();
		Self {
			stack_references: 0,
			object_references: RefCell::new(None),
			dfn: 0,
			low_link: 0,
			on_stack: false,
			array: items,
		}
	}

	pub fn add(&mut self, item: StackItem) {
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

		let mut result = Array::new(None);
		map.insert(self.into(), result.clone().into());

		for item in &self.array {
			result.add(item.deep_copy(map));
		}

		result.into()
	}

	pub fn iter(&self) -> std::slice::Iter<StackItem> {
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
		todo!()
	}

	fn deep_copy(&self, ref_map: &HashMap<&StackItem, StackItem>, as_immutable: bool) -> StackItem {
		todo!()
	}

	fn get_boolean(&self) -> bool {
		todo!()
	}

	fn get_slice(&self) -> &[u8] {
		todo!()
	}

	fn get_type(&self) -> StackItemType {
		StackItemType::Array
	}
}

impl CompoundTypeTrait for Array {
	fn reference_counter(&self) -> Option<&ReferenceCounter> {
		todo!()
	}

	fn count(&self) -> usize {
		self.array.len()
	}

	fn sub_items(&self) -> Vec<&StackItem> {
		self.array.iter().collect()
	}

	fn sub_items_count(&self) -> usize {
		self.array.len()
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

impl Into<StackItem> for Array {
	fn into(self) -> StackItem {
		StackItem::Array(self)
	}
}
