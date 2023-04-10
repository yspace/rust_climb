#[test]
fn test_remove() {
    let mut v = vec![1, 2, 3];
    // iterate through the vector and return the position for the
    // first element == 2. If something is found bind it to the name
    // index
    if let Some(index) = v.iter().position(|&i| i == 2) {
        v.remove(index); // remove the element at the position index (2)
    }

    println!("{:?}", v); // prints [1, 3]
    v.pop();
    println!("{:?}", v); // prints [1]
}

#[test]
fn test_retain() {
    let mut vec = vec![1, 2, 3, 4];
    vec.retain(|&x| x % 2 == 0);
    assert_eq!(vec, [2, 4]);
}

#[test]
fn test_swap_remove() {
    let mut some_vec = vec![1, 2, 3, 4, 7, 8, 9, 10];
    if let Some(index) = some_vec.iter().position(|value| *value == 10) {
        some_vec.swap_remove(index);
    }
    println!("vec {:?}", some_vec);
}

// @see https://stackoverflow.com/questions/26243025/how-to-remove-an-element-from-a-vector-given-the-element
#[test]
fn main() {
    let mut vec = vec![1, 2, 3, 4];

    println!("Before: {:?}", vec);

    let removed = vec
        .iter()
        .position(|&n| n > 2)
        .map(|e| vec.remove(e))
        .is_some();

    println!("Did we remove anything? {}", removed);

    println!("After: {:?}", vec);
}

#[test]
fn test_position_remove() {
    let mut vec = vec![1, 2, 3, 4];

    let needle = 3 ;
    // Panic if no such element is found
    vec.remove(
        vec.iter()
            .position(|x| *x == needle)
            .expect("needle not found"),
    );

    let needle = 2 ;
    // Ignore if no such element is found
    if let Some(pos) = vec.iter().position(|x| *x == needle) {
        vec.remove(pos);
    }

    println!("After: {:?}", vec);
}
