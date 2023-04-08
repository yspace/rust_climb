
trait Trait{
    type AssociatedType;

    fn func(arg: Self::AssociatedType);
}

struct SomeType;
struct OtherType;

impl Trait for SomeType{
    type AssociatedType = i8 ;
    fn func(arg: Self::AssociatedType){

    }
}

impl Trait for OtherType {
    type AssociatedType = u8; // chooses u8
    fn func(arg: Self::AssociatedType) {}
}

#[test]
fn test_associated_type(){
    SomeType::func(-1_i8);
    OtherType::func(1_u8);
}