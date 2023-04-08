/*

trait Hash {
    fn hash<H: Hasher>(&self, state: &mut H);

    // provided default impls
    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H);
}

If a type impls both Hash and Eq those impls must agree with each other such that for all a and b if a == b then a.hash() == b.hash().
 So we should always use the derive macro to impl both or manually impl both,
 but not mix the two, otherwise we risk breaking the above invariant.

 */

use std::hash::Hash;
use std::hash::Hasher;

struct Point {
    x: i32,
    y: i32,
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_i32(self.x);
        hasher.write_i32(self.y);
    }
}
/*
与👆上面同效果

 #[derive(Hash)]
struct Point {
    x: i32,
    y: i32,
}
  */

mod benefit_of_hash_eq {
    use std::collections::{HashSet, HashMap};

    // now our type can be stored
    // in HashSets and HashMaps!
    #[derive(PartialEq, Eq, Hash)]
    struct Point {
        x: i32,
        y: i32,
    }

    fn example_hashset() {
        let mut points = HashSet::new();
        points.insert(Point { x: 0, y: 0 }); // ✅
    }

    #[test]
    fn example_hashmap() {
        let mut points = HashMap::new();
        points.insert(Point { x: 0, y: 0 },"some hotel address"); // ✅
    }
}
