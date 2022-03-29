use human_panic::setup_panic;

pub fn main() {
   setup_panic!();

   panic!("Hello world")
}