// 最有用的就是作为函数返回值 多值返回 虽然有用但比较局限
// 元祖元素尽量限制在12个以内 标准库也只是对十二个以内元素的元祖提供了一些trait的实现

// 可以用作模式匹配 不可以被迭代 切片 反射（运行期决定其组件类型）

fn swap<A, B>(a: A, b: B) -> (B, A) {
     (b, a)
}

#[test]
fn test_tuples() {
    let tuple = (1, 2, 3);

    println!("tuple = ({}, {}, {})", tuple.0, tuple.1, tuple.2);

    match tuple {
        (one, two, three) => println!("{}, {}, {}", one, two, three),
    }

    let (one, two, three) = tuple;
    println!("{}, {}, {}", one, two, three);
}
