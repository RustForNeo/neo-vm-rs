use crate::{
	stack_item::{StackItem},
};
use std::{
	cell::{Ref, RefCell},
	hash::Hash,
};

pub trait CompoundType: StackItem {
	fn count(&self) -> usize;
	fn sub_items(&self) -> Vec<Ref<RefCell<dyn StackItem>>>;
	fn sub_items_count(&self) -> usize{
		self.sub_items().len()
	}
	fn read_only(&self);
	fn is_read_only(&self) -> bool {
		false
	}

	fn clear(&mut self);

	fn as_bool(&self) -> bool {
		true
	}
}