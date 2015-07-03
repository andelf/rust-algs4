pub mod linked_stack;




pub trait StackOfStrings {
    fn new() -> Self;
    fn push(&mut self, item: String);
    fn pop(&mut self) ->  String;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize {
        unimplemented!()
    }
}
