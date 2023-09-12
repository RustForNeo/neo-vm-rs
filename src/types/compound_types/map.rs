use crate::{
	primitive_types::primitive_type::PrimitiveType,
	reference_counter::ReferenceCounter,
	stack_item::{ObjectReferenceEntry, StackItem, StackItemTrait},
	stack_item_type::StackItemType,
	types::compound_types::compound_type::{CompoundType, CompoundTypeTrait},
};
use std::{
	cell::RefCell,
	collections::{
		hash_map::{Entry, Iter, IterMut, Keys, Values},
		HashMap,
	},
	fmt::{Debug, Formatter},
	hash::{Hash, Hasher},
	ops::Deref,
	rc::Rc,
};

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default, PartialOrd, Ord)]
pub struct Map {
	stack_references: u32,
	reference_counter: Rc<RefCell<ReferenceCounter>>,
	object_references: RefCell<Option<HashMap<CompoundType, ObjectReferenceEntry>>>,
	dfn: isize,
	low_link: usize,
	on_stack: bool,
	dictionary: HashMap<Rc<RefCell<PrimitiveType>>, Rc<RefCell<StackItem>>>,
	read_only: bool,
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
			read_only: false,
		}
	}

	pub fn insert(&mut self, key: Rc<RefCell<PrimitiveType>>, value: Rc<RefCell<StackItem>>) {
		if key.size() > Self::MAX_KEY_SIZE {
			panic!("Max key size exceeded: {}", key.size());
		}

		self.dictionary.insert(key.clone(), value);
	}

	pub fn get(&self, key: Rc<RefCell<PrimitiveType>>) -> Option<Rc<RefCell<StackItem>>> {
		if key.size() > Self::MAX_KEY_SIZE {
			panic!("Max key size exceeded: {}", key.size());
		}
		match self.dictionary.get(&key) {
			Some(value) => Some(value.clone()),
			None => None,
		}
	}

	pub fn contains_key(&self, key: Rc<RefCell<PrimitiveType>>) -> bool {
		if key.size() > Self::MAX_KEY_SIZE {
			panic!("Max key size exceeded: {}", key.size());
		}

		self.dictionary.contains_key(&key)
	}

	pub fn remove(&mut self, key: Rc<RefCell<PrimitiveType>>) -> Option<Rc<RefCell<StackItem>>> {
		if key.size() > Self::MAX_KEY_SIZE {
			panic!("Max key size exceeded: {}", key.size());
		}

		self.dictionary.remove(&key)
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

	pub fn keys(&self) -> Vec<Rc<RefCell<StackItem>>> {
		self.dictionary.into_keys().collect()
	}

	pub fn values(&self) -> Vec<Rc<RefCell<StackItem>>> {
		self.dictionary.into_values().collect()
	}

	pub fn iter(&self) -> Iter<'_, Rc<RefCell<PrimitiveType>>, Rc<RefCell<StackItem>>> {
		self.dictionary.iter()
	}

	pub fn iter_mut(&mut self) -> IterMut<'_, Rc<RefCell<PrimitiveType>>, Rc<RefCell<StackItem>>> {
		self.dictionary.iter_mut()
	}

	pub fn entry(
		&mut self,
		key: Rc<RefCell<PrimitiveType>>,
	) -> Entry<'_, Rc<RefCell<PrimitiveType>>, Rc<RefCell<StackItem>>> {
		self.dictionary.entry(key)
	}
}

impl StackItemTrait for Map {
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
		panic!("Cannot get slice of map")
	}

	fn get_type(&self) -> StackItemType {
		StackItemType::Map
	}

	fn equals(&self, other: &Option<StackItem>) -> bool {
		todo!()
	}
}

impl CompoundTypeTrait for Map {
	fn count(&self) -> usize {
		self.dictionary.len()
	}

	fn sub_items(&self) -> Vec<Rc<RefCell<StackItem>>> {
		let mut v = Vec::new();
		self.dictionary.keys().chain(self.dictionary.values()).cloned().collect()
	}

	fn sub_items_count(&self) -> usize {
		self.count() * 2
	}

	fn read_only(&mut self) {
		self.read_only = true;
	}

	fn is_read_only(&self) -> bool {
		self.read_only
	}

	fn clear(&mut self) {
		todo!()
	}

	fn deep_copy(
		&self,
		ref_map: &HashMap<Rc<RefCell<StackItem>>, Rc<RefCell<StackItem>>>,
	) -> StackItem {
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
