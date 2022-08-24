fn basic_loop() {
    let mut i = 0;
    loop {
        i += 1;
    }
}

fn breaking() {
    let mut i = 0;
    loop {
        i += 1;

        if i > 10 {
            break;
        }
    }
}

fn continues() {
    let mut i = 0;
    loop {
        i += 1;
        continue;
        println!("this is never printed");
    }
}
// Using Rust’s label syntax labelname: right in front of the loop statement creates this reference point that breaks and continues can use.
fn loop_labels() {
    let mut i = 0;
    'outer: loop {
        'inner: loop {
            i += 1;
            break 'inner;
        }
        break 'outer;
    }
}

fn break_or_return() {
    let a_number = 10;
    let mut i = 0;
    let outcome = loop {
        if i + a_number > 100 {
            break i + a_number;
        } else if i + a_number < 50 {
            i += 10;
        } else {
            i += 1;
        }
    };

    println!("outcome = {}", outcome);
}

fn fors() {
    for i in 0..10 {}
    for i in 0..=10 {}

    for e in &[6, 7, 8, 0, 5, 3, 1, 2, 9] {}
}

fn bubble_sort() {
    let mut haystack = [4, 6, 3, 2, 8];
    println!("unsorted haystack: {:?}", haystack); // bubble sort algorithm
    for _ in 0..haystack.len() {
        let mut swaps = 0;
        for i in 1..haystack.len() {
            if haystack[i - 1] > haystack[i] {
                let tmp = haystack[i];
                haystack[i] = haystack[i - 1];
                haystack[i - 1] = tmp;
                swaps += 1;
            }
        }
    }
    println!(" sorted haystack: {:?}", haystack);
}
