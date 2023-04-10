pub mod as_slices;
pub mod wrapping_vectors;
pub mod removes;

pub fn main() {
    println!("run vectors::main()");

    let my_vec1: Vec<i32> = Vec::new();

    let my_vec = vec![1, 2, 3, 4, 5];
    println!("{}", my_vec[2]);
}