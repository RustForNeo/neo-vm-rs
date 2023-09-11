use std::{collections::VecDeque, convert::TryInto};
use crate::reference_counter::ReferenceCounter;
use crate::stack_item::StackItem;

/// Represents the evaluation stack in the VM.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct EvaluationStack<'a> {
    stack: VecDeque<StackItem<'a>>,
}

impl EvaluationStack {
    pub fn new(reference_counter: &ReferenceCounter) -> Self {
        Self {
            stack: VecDeque::new(),
        }
    }

    /// Gets the number of items on the stack.
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn clear(&mut self) {
        self.stack.clear();
    }

    pub fn copy_to(&self, other: &mut EvaluationStack, count: Option<usize>) {
        let count = count.unwrap_or(self.len());
        if count == 0 {
            return;
        }
        other.stack.extend(self.stack.iter().take(count));
    }

    /// Returns the item at the specified index from the top of the stack without removing it.
    pub fn peek(&self, index: i64) -> Option<&StackItem> {
        let index = index.try_into().ok()?;
        self.stack.get(self.len().checked_sub(index + 1)?)
    }

    /// Pushes an item onto the top of the stack.
    pub fn push(&mut self, item: StackItem) {
        self.stack.push_back(item);
    }

    pub fn reverse(&mut self, n: usize) {
        if n > self.len() {
            panic!("n out of bounds");
        }
        if n <= 1 {
            return;
        }
        self.stack.make_contiguous().rotate_right(n);
    }

    /// Removes and returns the item at the top of the stack.
    pub fn pop(&mut self) -> Option<StackItem> {
        self.stack.pop_back()
    }

    /// Removes and returns the item at the top of the stack and convert it to the specified type.
    pub fn pop_as<T: TryInto<StackItem> + Copy>(&mut self) -> Option<T> {
        self.stack.pop_back().map(|x| x.try_into().ok())
    }

    pub fn remove<T: TryInto<StackItem> + Copy>(&mut self, index: i64) -> Option<T> {
        let index = index.try_into().ok()?;
        let index = self.len().checked_sub(index + 1)?;
        self.stack.get_mut(index).map(|x| (*x).try_into().ok()).map(|x| x.map(|y| {
            self.stack.remove(index);
            y
        }))?.flatten()
    }
}