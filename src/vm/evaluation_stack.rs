use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use crate::reference_counter::ReferenceCounter;
use crate::stack_item::StackItem;

pub struct EvaluationStack {
	inner_list: VecDeque<Rc<RefCell<dyn StackItem>>>,
	reference_counter: Rc<RefCell<ReferenceCounter>>,
}

impl EvaluationStack {

	pub fn new(reference_counter: Rc<RefCell<ReferenceCounter>>) -> Self {
		Self {
			inner_list: VecDeque::new(),
			reference_counter,
		}
	}

	pub fn clear(&mut self) {
		for item in &self.inner_list {
			self.reference_counter.remove_stack_reference(item);
		}
		self.inner_list.clear();
	}

	pub fn copy_to(&self, stack: &mut EvaluationStack, count: i32) {
		if count < -1 || count as usize > self.inner_list.len() {
			panic!("Argument out of range");
		}
		if count == 0 {
			return;
		}
		if count == -1 || count as usize == self.inner_list.len() {
			stack.inner_list.extend(&self.inner_list);
		} else {
			let start = self.inner_list.len() - count as usize;
			stack.inner_list.extend(&self.inner_list[start..]);
		}
	}

	pub fn insert(&mut self, index: usize, item: Rc<RefCell<dyn StackItem>>) {
		if index > self.inner_list.len() {
			panic!("Insert out of bounds");
		}
		self.inner_list.insert(self.inner_list.len() - index, item);
		self.reference_counter.add_stack_reference(&item);
	}

	pub fn move_to(&mut self, stack: &mut EvaluationStack, count: i32) {
		if count == 0 {
			return;
		}
		self.copy_to(stack, count);
		if count == -1 || count as usize == self.inner_list.len() {
			self.inner_list.clear();
		} else {
			let end = self.inner_list.len() - count as usize;
			self.inner_list.drain(end..);
		}
	}

	pub fn peek(&self, index: i32) -> Rc<RefCell<dyn StackItem>> {
		let index = index as isize;
		if index >= self.inner_list.len() as isize {
			panic!("Peek out of bounds");
		}
		if index < 0 {
			let index = self.inner_list.len() as isize + index;
			if index < 0 {
				panic!("Peek out of bounds");
			}
		}
		self.inner_list.get((self.inner_list.len() as isize - index - 1) as usize).unwrap().clone()
	}

	pub fn push(&mut self, item: Rc<RefCell<dyn StackItem>>) {
		self.inner_list.push_back(item);
		self.reference_counter.add_stack_reference(&item);
	}

	pub fn reverse(&mut self, n: i32) {
		let n = n as usize;
		if n < 0 || n > self.inner_list.len() {
			panic!("Argument out of range");
		}
		if n <= 1 {
			return;
		}
		let end = self.inner_list.len() - n;
		self.inner_list.make_contiguous().reverse();
	}

	pub fn pop(&mut self) -> Rc<RefCell<dyn StackItem>> {
		self.remove(0)
	}

	pub fn pop_typed<T: StackItem>(&mut self) -> T {
		self.remove::<T>(0)
	}

	pub fn remove<T: StackItem>(&mut self, index: i32) -> T {
		let index = index as isize;
		if index >= self.inner_list.len() as isize {
			panic!("Argument out of range");
		}
		if index < 0 {
			let index = self.inner_list.len() as isize + index;
			if index < 0 {
				panic!("Argument out of range");
			}
		}
		let index = self.inner_list.len() as isize - index - 1;
		let item = self.inner_list.remove(index as usize).unwrap();
		if !item.is::<T>() {
			panic!("Invalid cast");
		}
		self.reference_counter.remove_stack_reference(&item);
		item.try_into().unwrap()
	}

	pub fn size(&self) -> usize {
		self.inner_list.len()
	}
}