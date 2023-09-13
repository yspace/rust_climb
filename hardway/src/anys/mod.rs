pub mod as_return;
pub mod ay_any_hack;
pub mod extension_map;

pub fn print_type_name<T>(_val: &T) {
    println!("{}", std::any::type_name::<T>());
}

#[test]
fn test_type_id() {
    use std::any::{Any, TypeId};

    fn is_string(s: &dyn Any) -> bool {
        TypeId::of::<String>() == s.type_id()
    }

    assert_eq!(is_string(&0), false);
    assert_eq!(is_string(&"cookie monster".to_string()), true);
}

#[test]
fn test_print_type_name() {
    let s = "cookie monster";   // &str

    print_type_name(&s);

}

pub fn main() {
    trait_object_conversion::run() ;
}

mod trait_object_conversion{
    use std::any::Any;

    mod sized_component{
        use std::any::Any;

        trait Component: Any + Sized {
            fn as_any(&self) -> &Any {
                self
            }
        
            fn as_any_mut(&mut self) -> &mut Any {
                self
            }
        }
    }
    mod conditional_available{
        use std::any::Any;

        // This is
        trait Component: Any {
            fn as_any(&self) -> &Any
                where Self: Sized
            {
                self
            }
        
            fn as_any_mut(&mut self) -> &mut Any
                where Self: Sized
            {
                self
            }
        }
    }

    pub trait Component {
        // ...
    }
    
    pub trait ComponentAny: Component + Any {
        fn as_any(&self) -> &Any;
        fn as_any_mut(&mut self) -> &mut Any;
    }
    
    impl<T> ComponentAny for T
        where T: Component + Any
    {
        fn as_any(&self) -> &Any {
            self
        }
    
        fn as_any_mut(&mut self) -> &mut Any {
            self
        }
    }

    pub fn run(){
        struct MyComponent ;
        // impl ComponentAny for MyComponent {}
        impl Component for MyComponent {}
        impl MyComponent{
            fn say_world(&self){
                println!("hello from MyComponent") ;
            }
        }

        fn as_component(comp:Box< dyn ComponentAny>) -> Box<dyn ComponentAny> {
            comp
        }

        let my_comp = MyComponent{};
        // let c1 = &my_comp ;
        // c1.say_world() ;
        let my_comp = as_component(Box::new(my_comp)) ;

        let c2 = my_comp.as_any().downcast_ref::<MyComponent> ();
        let c2 = c2.unwrap();
        c2.say_world();
    }
}