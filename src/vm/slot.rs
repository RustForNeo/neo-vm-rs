use crate::null::Null;
use crate::reference_counter::ReferenceCounter;
use crate::stack_item::StackItem;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Slot<'a> {
    items: Vec<StackItem>,
    reference_counter: &'a ReferenceCounter,
}

impl Slot {
    pub fn new(items: Vec<StackItem>, reference_counter: &ReferenceCounter) -> Self {
        let mut slot = Self {
            items,
            reference_counter,
        };
        for item in &slot.items {
            slot.reference_counter.add_stack_reference(item, 1);
        }
        slot
    }

    pub fn new_with_count(count: i32, reference_counter: &ReferenceCounter) -> Self {

        let mut items = Vec::new();
        for _ in 0..count{
            items.push(StackItem::from(Null::default()));
        }

        Self{
            items,
            reference_counter: reference_counter,
        }
    }

    pub fn with_capacity(capacity: usize, reference_counter: &ReferenceCounter) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
            reference_counter,
        }
    }

    pub fn get(&self, index: usize) -> &StackItem {
        &self.items[index]
    }

    pub fn set(&mut self, index: usize, value: StackItem) {
        let old_value = std::mem::replace(&mut self.items[index], value);
        self.reference_counter.remove_stack_reference(&old_value);
        self.reference_counter.add_stack_reference(&value, 1);
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn clear_references(&mut self) {
        for item in &self.items {
            self.reference_counter.remove_stack_reference(item);
        }
    }
}

impl IntoIterator for Slot {
    type Item = StackItem;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}