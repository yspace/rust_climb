
#[test]
fn test_compose(){
    fn compose<T>(f: impl Fn(T) -> T, g: impl Fn(T) -> T) -> impl Fn(T) -> T {
        move |i: T| f(g(i))
      }

      fn make_adder(left: i32) -> impl Fn(i32) -> i32 {
        move |right: i32| {
          println!("{} + {} is {}", left, right, left + right);
          left + right
        }
      }
      
      let plus_two = make_adder(2);  // ← make_adder from above
      let times_two = |i: i32| i * 2;
      let double_plus_two = compose(plus_two, times_two);
      println!("{} * 2 + 2 = {}", 10, double_plus_two(10));
}