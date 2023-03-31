mod basics {
    #[test]
    fn simple() {
        // 有io边缘影响 不是一个纯的闭包
        let eat = || println!("eat!");

        eat();

        let increment = |val| val + 1;
        let result = increment(1);
        println!("result: {:?}", result);

        let print_and_increment = |val| {
            println!("{val} will be incremented and returned");
            val + 1
        };

        let result = print_and_increment(2);
        println!("result: {result}");
    }

    #[test]
    fn test_highorder() {
        let left_value = || 1;
        let right_value = || 2;
        let adder = |left: fn() -> i32, right: fn() -> i32| left() + right();
        let result = adder(left_value, right_value); // returns 3

        println!("result: {result}"); 
    }
}
