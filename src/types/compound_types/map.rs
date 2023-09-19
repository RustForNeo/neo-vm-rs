use crate::{
	primitive_types::primitive_type::PrimitiveType,
	reference_counter::ReferenceCounter,
	stack_item::{ObjectReferenceEntry, StackItem,},
	stack_item_type::StackItemType,
	types::compound_types::compound_type::{CompoundType},
};
use std::{
	cell::RefCell,
	collections::{
		hash_map::{Entry, Iter, IterMut},
		HashMap,
	},
	fmt::Debug,
	hash::Hash,
	rc::Rc,
};
use std::any::Any;
use num_bigint::BigInt;
use crate::execution_engine_limits::ExecutionEngineLimits;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default, PartialOrd, Ord)]
pub struct Map {
	stack_references: u32,
	reference_counter: Option<Rc<RefCell<ReferenceCounter>>>,
	object_references: RefCell<Option<HashMap<dyn CompoundType, ObjectReferenceEntry>>>,
	dfn: isize,
	low_link: usize,
	on_stack: bool,
	dictionary: HashMap<Rc<RefCell<dyn PrimitiveType>>, Rc<RefCell<dyn StackItem>>>,
	read_only: bool,
}

impl Map {
	pub const MAX_KEY_SIZE: usize = 64;

	pub fn new(reference_counter: Option<Rc<RefCell<ReferenceCounter>>>) -> Self {
		Self {
			stack_references: 0,
			reference_counter,
			object_references: RefCell::new(None),
			dfn: 0,
			low_link: 0,
			on_stack: false,
			dictionary: HashMap::new(),
			read_only: false,
		}
	}

	pub fn insert(&mut self, key: Rc<RefCell<dyn PrimitiveType>>, value: Rc<RefCell<dyn StackItem>>) {
		if key.size() > Self::MAX_KEY_SIZE {
			panic!("Max key size exceeded: {}", key.size());
		}

		self.dictionary.insert(key.clone(), value);
	}

	pub fn get(&self, key: Rc<RefCell<dyn PrimitiveType>>) -> Option<Rc<RefCell<dyn StackItem>>> {
		if key.size() > Self::MAX_KEY_SIZE {
			panic!("Max key size exceeded: {}", key.size());
		}
		match self.dictionary.get(&key) {
			Some(value) => Some(value.clone()),
			None => None,
		}
	}

	pub fn contains_key(&self, key: Rc<RefCell<dyn PrimitiveType>>) -> bool {
		if key.size() > Self::MAX_KEY_SIZE {
			panic!("Max key size exceeded: {}", key.size());
		}

		self.dictionary.contains_key(&key)
	}

	pub fn remove(&mut self, key: Rc<RefCell<dyn PrimitiveType>>) -> Option<Rc<RefCell<dyn StackItem>>> {
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

	pub fn keys(&self) -> Vec<Rc<RefCell<dyn StackItem>>> {
		self.dictionary.into_keys().collect()
	}

	pub fn values(&self) -> Vec<Rc<RefCell<dyn StackItem>>> {
		self.dictionary.into_values().collect()
	}

	pub fn iter(&self) -> Iter<'_, Rc<RefCell<dyn PrimitiveType>>, Rc<RefCell<dyn StackItem>>> {
		self.dictionary.iter()
	}

	pub fn iter_mut(&mut self) -> IterMut<'_, Rc<RefCell<dyn PrimitiveType>>, Rc<RefCell<dyn StackItem>>> {
		self.dictionary.iter_mut()
	}

	pub fn entry(
		&mut self,
		key: Rc<RefCell<dyn PrimitiveType>>,
	) -> Entry<'_, Rc<RefCell<dyn PrimitiveType>>, Rc<RefCell<dyn StackItem>>> {
		self.dictionary.entry(key)
	}
}

impl StackItem for Map {
	const TRUE: Self =  Default::default();

	const FALSE: Self =  Default::default();

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
		panic!("Cannot get slice of map")
	}

	fn get_type(&self) -> StackItemType {
		StackItemType::Map
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

	fn get_interface<T: Any>(&self) -> Option<&T> {
		todo!()
	}

	fn get_bytes(&self) -> &[u8] {
		todo!()
	}
}

impl CompoundType for Map {
	fn count(&self) -> usize {
		self.dictionary.len()
	}

	fn sub_items(&self) -> Vec<Rc<RefCell<dyn StackItem>>> {
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
		if self.read_only {
			panic!("Cannot clear read-only map")
		}
		if self.reference_counter.is_some() {
			for (key, value) in self.dictionary.iter() {
				self.reference_counter
					.unwrap()
					.get_mut()
					.remove_stack_reference(key.clone().into());
				self.reference_counter.unwrap().get_mut().remove_stack_reference(value.clone());
			}
		}
		self.dictionary.clear();
	}
}

impl PartialEq for Map {
	fn eq(&self, other: &Self) -> bool {
		self.dictionary == other.dictionary
	}
}
impl Clone for Map {
	fn clone(&self) -> Self {
		let mut result = Self::new(self.reference_counter.clone());
		// ref_map.insert(self, result.clone());
		for (key, value) in self.dictionary.iter() {
			result.dictionary.insert(key.clone(), value.clone());
		}

		result
	}
}
