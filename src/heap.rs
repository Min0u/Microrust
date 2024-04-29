use crate::memory::Address;
use crate::memorycell::MemoryCell;
use crate::parsing::value::Value;

#[derive(Clone, Debug)]
pub struct Heap(Vec<MemoryCell>);

impl Heap {
    pub fn new() -> Self {
        Heap(Vec::new())
    }

    pub fn malloc(&mut self, mutable: bool, value: Value) -> Address {
        for addr in 0..self.0.len() {
            if !self.0[addr].is_allocated() {
                self.0[addr] = MemoryCell::new(mutable, value);
                return Address::HeapAddress(addr);
            }
        }
        self.0.push(MemoryCell::new(mutable, value));
        Address::HeapAddress(self.0.len() - 1)
    }
}