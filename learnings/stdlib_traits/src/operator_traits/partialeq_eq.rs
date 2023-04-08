/*
trait PartialEq<Rhs = Self>
where
    Rhs: ?Sized,
{
    fn eq(&self, other: &Rhs) -> bool;

    // provided default impls
    fn ne(&self, other: &Rhs) -> bool;
}

All PartialEq<Rhs> impls must ensure that equality is symmetric and transitive. That means for all a, b, and c:

    a == b implies b == a (symmetry)
    a == b && b == c implies a == c (transitivity)

必须保证 对称性 跟传递性
 */

struct Point {
    x: i32,
    y: i32,
}

// Rhs == Self == Point
impl PartialEq for Point {
    // impl automatically symmetric & transitive
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

// If all the members of a type impl PartialEq then it can be derived:

#[derive(PartialEq)]
struct MyPoint(Point);

/*
generic blanket impls:

// this impl only gives us: Point == Point
#[derive(PartialEq)]
struct Point {
    x: i32,
    y: i32
}

// all of the generic blanket impls below
// are provided by the standard library

// this impl gives us: &Point == &Point
impl<A, B> PartialEq<&'_ B> for &'_ A
where A: PartialEq<B> + ?Sized, B: ?Sized;

// this impl gives us: &mut Point == &Point
impl<A, B> PartialEq<&'_ B> for &'_ mut A
where A: PartialEq<B> + ?Sized, B: ?Sized;

// this impl gives us: &Point == &mut Point
impl<A, B> PartialEq<&'_ mut B> for &'_ A
where A: PartialEq<B> + ?Sized, B: ?Sized;

// this impl gives us: &mut Point == &mut Point
impl<A, B> PartialEq<&'_ mut B> for &'_ mut A
where A: PartialEq<B> + ?Sized, B: ?Sized;
 */

mod cute_but_bad_example {
    // cute but bad example of how someone might be tempted to impl PartialEq to check equality between different types that don't meet the above criteria:

    #[derive(PartialEq)]
    enum Suit {
        Spade,
        Club,
        Heart,
        Diamond,
    }

    #[derive(PartialEq)]
    enum Rank {
        Ace,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Jack,
        Queen,
        King,
    }

    #[derive(PartialEq)]
    struct Card {
        suit: Suit,
        rank: Rank,
    }

    // check equality of Card's suit
    // impl PartialEq<Suit> for Card {
    //     fn eq(&self, other: &Suit) -> bool {
    //         self.suit == *other
    //     }
    // }

    // check equality of Card's rank
    // impl PartialEq<Rank> for Card {
    //     fn eq(&self, other: &Rank) -> bool {
    //         self.rank == *other
    //     }
    // }

    #[test]
    fn main() {
        let AceOfSpades = Card {
            suit: Suit::Spade,
            rank: Rank::Ace,
        };
        assert!(AceOfSpades == Suit::Spade); // ✅
        assert!(AceOfSpades == Rank::Ace); // ✅
    }

    // 添加对称性实现
    // check equality of Card's suit
    impl PartialEq<Suit> for Card {
        fn eq(&self, other: &Suit) -> bool {
            self.suit == *other
        }
    }

    // added for symmetry
    impl PartialEq<Card> for Suit {
        fn eq(&self, other: &Card) -> bool {
            *self == other.suit
        }
    }

    // check equality of Card's rank
    impl PartialEq<Rank> for Card {
        fn eq(&self, other: &Rank) -> bool {
            self.rank == *other
        }
    }

    // added for symmetry
    impl PartialEq<Card> for Rank {
        fn eq(&self, other: &Card) -> bool {
            *self == other.rank
        }
    }

    #[test]
    fn main4symmetry() {
        // Ace of Spades
        let a = Card {
            suit: Suit::Spade,
            rank: Rank::Ace,
        };
        let b = Suit::Spade;
        // King of Spades
        let c = Card {
            suit: Suit::Spade,
            rank: Rank::King,
        };
        assert!(a == b && b == c); // ✅
                                   // assert!(a == c); // ❌
    }
}

mod good_example {
    #[derive(PartialEq)]
    struct Foot(u32);

    #[derive(PartialEq)]
    struct Yard(u32);

    #[derive(PartialEq)]
    struct Mile(u32);

    impl PartialEq<Mile> for Foot {
        fn eq(&self, other: &Mile) -> bool {
            self.0 == other.0 * 5280
        }
    }

    impl PartialEq<Foot> for Mile {
        fn eq(&self, other: &Foot) -> bool {
            self.0 * 5280 == other.0
        }
    }

    impl PartialEq<Mile> for Yard {
        fn eq(&self, other: &Mile) -> bool {
            self.0 == other.0 * 1760
        }
    }

    impl PartialEq<Yard> for Mile {
        fn eq(&self, other: &Yard) -> bool {
            self.0 * 1760 == other.0
        }
    }

    impl PartialEq<Foot> for Yard {
        fn eq(&self, other: &Foot) -> bool {
            self.0 * 3 == other.0
        }
    }

    impl PartialEq<Yard> for Foot {
        fn eq(&self, other: &Yard) -> bool {
            self.0 == other.0 * 3
        }
    }

    #[test]
    fn main() {
        let a = Foot(5280);
        let b = Yard(1760);
        let c = Mile(1);

        // symmetry
        assert!(a == b && b == a); // ✅
        assert!(b == c && c == b); // ✅
        assert!(a == c && c == a); // ✅

        // transitivity
        assert!(a == b && b == c && a == c); // ✅
        assert!(c == b && b == a && c == a); // ✅
    }
}

/*

Eq is a marker trait and a subtrait of PartialEq<Self>.

trait Eq: PartialEq<Self> {}

Floats are PartialEq but not Eq because NaN != NaN.
 Almost all other PartialEq types are trivially Eq,
 unless of course if they contain floats.
 */

mod partialeq_usage {
    #[derive(PartialEq, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    fn example_assert(p1: Point, p2: Point) {
        assert_eq!(p1, p2);
    }

    fn example_compare_collections<T: PartialEq>(vec1: Vec<T>, vec2: Vec<T>) {
        // if T: PartialEq this now works!
        if vec1 == vec2 {
            // some code
        } else {
            // other code
        }
    }
}
