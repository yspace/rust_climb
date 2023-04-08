/*
trait Clone {
    fn clone(&self) -> Self;

    // provided default impls
    fn clone_from(&mut self, source: &Self);
}

People also commonly use cloning as an escape hatch to avoid dealing with the borrow checker.
 Managing structs with references can be challenging, but we can turn the references into owned values by cloning them.
 */
fn guarantee_length<T: Clone>(mut vec: Vec<T>, min_len: usize, fill_with: &T) -> Vec<T> {
    for _ in 0..min_len.saturating_sub(vec.len()) {
        vec.push(fill_with.clone());
    }
    vec
}