#[test]
fn it_works() {
    let amount_to_add = 3;
    let add_n = |y| {
        // a closure capturing `amount_to_add`
        y + amount_to_add
    };
    let z = add_n(5);
    assert_eq!(z, 8);
}

pub fn modify_all<F>(data: &mut [u32], mut mutator: F)
where
    F: FnMut(u32) -> u32,
{
    for value in data {
        *value = mutator(*value);
    }
}
