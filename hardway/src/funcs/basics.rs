fn gcd(mut n: u64, mut m: u64) -> u64
{
    // There is also a debug_assert! macro, whose assertions are skipped when the program is compiled for speed.
    assert!(n != 0 && m != 0);
    while m != 0
    {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

fn return_expression() -> i32 {
    let var1 = {
        println!("do something! ");
        "some returned var"
    };

    {
        2
    }
}

