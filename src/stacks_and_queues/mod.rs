// stacks
pub mod linked_stack;
pub mod fixed_capacity_stack;
pub mod resizing_array_stack;

pub trait StackOfStrings {
    fn new() -> Self;
    fn push(&mut self, item: String);
    fn pop(&mut self) ->  String;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize {
        unimplemented!()
    }
}
