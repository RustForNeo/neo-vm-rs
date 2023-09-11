use crate::{
	compound_type::{CompoundType, CompoundTypeTrait},
	primitive_type::{PrimitiveType, PrimitiveTypeTrait},
	reference_counter::ReferenceCounter,
	stack_item::{ObjectReferenceEntry, StackItem, StackItemTrait},
	stack_item_type::StackItemType,
};
use std::{
	cell::RefCell,
	collections::{
		hash_map::{Entry, Iter, IterMut, Keys, Values},
		HashMap,
	},
	fmt::{Debug, Formatter},
	hash::{Hash, Hasher},
	rc::Rc,
};

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default, PartialOrd, Ord)]
pub struct Map<'a> {
	stack_references: u32,
	reference_counter: Rc<RefCell<ReferenceCounter<'a>>>,
	object_references: RefCell<Option<HashMap<CompoundType<'a>, ObjectReferenceEntry<'a>>>>,
	dfn: isize,
	low_link: usize,
	on_stack: bool,
	dictionary: HashMap<PrimitiveType<'a>, StackItem<'a>>,
}

impl Map {
	pub const MAX_KEY_SIZE: usize = 64;

	pub fn new(reference_counter: Option<Rc<RefCell<ReferenceCounter>>>) -> Self {
		Self {
			stack_references: 0,
			reference_counter: match reference_counter {
				Some(rc) => rc,
				None => &Default::default(),
			},
			object_references: RefCell::new(None),
			dfn: 0,
			low_link: 0,
			on_stack: false,
			dictionary: HashMap::new(),
		}
	}

	pub fn insert(&mut self, key: &PrimitiveType, value: StackItem) {
		if key.size() > Self::MAX_KEY_SIZE {
			panic!("Max key size exceeded: {}", key.size());
		}

		self.dictionary.insert(key.clone(), value);
	}

	pub fn get(&self, key: &PrimitiveType) -> Option<&StackItem> {
		if key.size() > Self::MAX_KEY_SIZE {
			panic!("Max key size exceeded: {}", key.size());
		}

		self.dictionary.get(key)
	}

	pub fn contains_key(&self, key: &PrimitiveType) -> bool {
		if key.size() > Self::MAX_KEY_SIZE {
			panic!("Max key size exceeded: {}", key.size());
		}

		self.dictionary.contains_key(key)
	}

	pub fn remove(&mut self, key: PrimitiveType) -> Option<StackItem> {
		if key.size() > Self::MAX_KEY_SIZE {
			panic!("Max key size exceeded: {}", key.size());
		}

		self.dictionary.remove(key.borrow())
	}

	// Other map methods...
	pub fn len(&self) -> usize {
		self.dictionary.len()
	}

	pub fn is_empty(&self) -> bool {
		self.dictionary.is_empty()
	}

	pub fn clear(&mut self) {
		self.dictionary.clear();
	}

	pub fn keys(&self) -> Keys<'_, PrimitiveType, StackItem> {
		self.dictionary.keys()
	}

	pub fn values(&self) -> Values<PrimitiveType, StackItem> {
		self.dictionary.values()
	}

	pub fn iter(&self) -> Iter<'_, PrimitiveType, StackItem> {
		self.dictionary.iter()
	}

	pub fn iter_mut(&mut self) -> IterMut<'_, PrimitiveType, StackItem> {
		self.dictionary.iter_mut()
	}

	pub fn entry(&mut self, key: PrimitiveType) -> Entry<'_, PrimitiveType, StackItem> {
		self.dictionary.entry(key)
	}
}

impl<'a> StackItemTrait for Map {
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
		StackItemType::Map
	}

	fn equals(&self, other: &Option<StackItem>) -> bool {
		todo!()
	}
}

impl CompoundTypeTrait for Map {
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

impl Into<StackItem> for Map {
	fn into(self) -> StackItem {
		StackItem::VMMap(self)
	}
}

impl From<Map> for StackItem {
	fn from(map: Map) -> Self {
		StackItem::VMMap(map)
	}
}

impl From<Map> for CompoundType {
	fn from(map: Map) -> Self {
		Self::VMMap(map)
	}
}

impl Into<CompoundType> for Map {
	fn into(self) -> CompoundType {
		CompoundType::VMMap(self)
	}
}
