
/*

trait Subtrait: Supertrait {}

the above is just syntax sugar for:

trait Subtrait where Self: Supertrait {}

 */

 trait Supertrait {
    fn method(&self) {
        println!("in supertrait");
    }
}

trait Subtrait: Supertrait {
    // this looks like it might impl or
    // override Supertrait::method but it
    // does not
    fn method(&self) {
        println!("in subtrait")
    }
}

struct SomeType;

// adds Supertrait::method to SomeType
impl Supertrait for SomeType {}

// adds Subtrait::method to SomeType
impl Subtrait for SomeType {}

// both methods exist on SomeType simultaneously
// neither overriding or shadowing the other

#[test]
fn main() {
    // SomeType.method(); // ❌ ambiguous method call
    // must disambiguate using fully-qualified syntax
    <SomeType as Supertrait>::method(&SomeType); // ✅ prints "in supertrait"
    <SomeType as Subtrait>::method(&SomeType); // ✅ prints "in subtrait"
}

mod call_each_other{
    trait Supertrait {
        fn super_method(&mut self);
    }
    
    trait Subtrait: Supertrait {
        fn sub_method(&mut self);
    }
    
    struct CallSuperFromSub;
    
    impl Supertrait for CallSuperFromSub {
        fn super_method(&mut self) {
            println!("in super");
        }
    }
    
    impl Subtrait for CallSuperFromSub {
        fn sub_method(&mut self) {
            println!("in sub");
            self.super_method();
        }
    }
    
    struct CallSubFromSuper;
    
    impl Supertrait for CallSubFromSuper {
        fn super_method(&mut self) {
            println!("in super");
            self.sub_method();
        }
    }
    
    impl Subtrait for CallSubFromSuper {
        fn sub_method(&mut self) {
            println!("in sub");
        }
    }
    
    struct CallEachOther(bool);
    
    impl Supertrait for CallEachOther {
        fn super_method(&mut self) {
            println!("in super");
            if self.0 {
                self.0 = false;
                self.sub_method();
            }
        }
    }
    
    impl Subtrait for CallEachOther {
        fn sub_method(&mut self) {
            println!("in sub");
            if self.0 {
                self.0 = false;
                self.super_method();
            }
        }
    }
    
    fn main() {
        CallSuperFromSub.super_method(); // prints "in super"
        CallSuperFromSub.sub_method(); // prints "in sub", "in super"
        
        CallSubFromSuper.super_method(); // prints "in super", "in sub"
        CallSubFromSuper.sub_method(); // prints "in sub"
        
        CallEachOther(true).super_method(); // prints "in super", "in sub"
        CallEachOther(true).sub_method(); // prints "in sub", "in super"
    }
}

// generic types depend on their trait bounds.
fn function<T: Clone>(t: T) {
    // impl
}

// trait Copy: Clone {} // 跟上面👆的心智模型看似相同 Copy依赖Clone的方法 但实际不是
// the most simple and elegant mental model for understanding the relationship between subtraits and supertraits is: subtraits refine their supertraits.