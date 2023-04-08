
pub mod function;
pub mod methods;
pub mod associated_types;
pub mod generic_parameters;
pub mod scope;
pub mod derive_macros;
pub mod default_impls;
pub mod generic_blanket_impl;
pub mod subtraits_vs_supertraits;

trait Trait{
    fn returns_num()->i32 ;

    fn returns_self()-> Self ;
}

struct SomeType{}
struct OtherType{}

impl Trait for SomeType{
    fn returns_num()->i32  {
       5
    }

    fn returns_self()-> Self  {
       Self{}
    }
}

impl Trait for OtherType {
    fn returns_num()->i32  {
        6
    }

    fn returns_self()-> Self  {
        Self {  }
    }
}