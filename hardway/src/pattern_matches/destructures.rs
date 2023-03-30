fn destructure_tuple(tuple: &(i32, i32,i32)) {
    match tuple {
        (first, ..) => println!("first ele of tuple is {first}"),
    }
    match tuple {
        (.., last) => println!("last ele of tuple is {last}"),
    }
    match tuple {
        (_, middle, _) => println!("The middle tuple element is {middle}"),
    }
    match tuple {
        (first, middle, last) => {
            println!("The whole tuple is ({first}, {middle}, {last}")
        }
    }
}
