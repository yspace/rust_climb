pub fn main() {
    let mut i = 0;
    while i <= 10 {
        i += 1;
    }
}

fn insert_to_sorted() {
    let needle: i32 = 3;
    let haystack: [i32; 5] = [1, 2, 4, 5, 6];
    let mut i = 1;
    let len = haystack.len();
    // while i < &haystack.len() && &haystack[i] <= needle {
    while i < len /*&& &haystack[i] <= &needle */ {
        i += 1;
    }
}
