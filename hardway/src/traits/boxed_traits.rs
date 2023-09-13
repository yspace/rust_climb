// https://bennett.dev/dont-use-boxed-trait-objects-for-struct-internals/
trait Person {
    fn say_hello(&self);
    // fn as_any(&self) -> &dyn Any;
}

mod dyn_trait {
    trait Person {
        fn say_hello(&self);
    }

    struct PeopleZoo {
        people: Vec<Box<dyn Person>>,
    }

    impl PeopleZoo {
        fn add_person(&mut self, person: Box<dyn Person>) {
            self.people.push(person);
        }

        // fn last_person(&self) -> Option<&Person> {
        fn last_person(&self) -> Option<&Box<dyn Person>> {
            self.people.last()
        }
    }

    struct Me {
        name: &'static str,
    }

    impl Person for Me {
        fn say_hello(&self) {
            println!("Hello, it's me.");
        }
    }

    #[test]
    fn main1() {
        let mut zoo: PeopleZoo = PeopleZoo { people: vec![] };
        zoo.add_person(Box::new(Me { name: "Bennett" }));

        // How can I figure out that this is "Me"?
        let person = zoo.last_person().unwrap();
    }
}

mod retrofit {
    use std::any::Any;

    trait Person {
        fn say_hello(&self);
        fn as_any(&self) -> &dyn Any;
    }

    struct Me {
        name: &'static str,
    }

    impl Person for Me {
        fn say_hello(&self) {
            println!("Hello, it's me.")
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    struct PeopleZoo {
        people: Vec<Box<dyn Person>>,
    }

    impl PeopleZoo {
        fn add_person(&mut self, person: Box<dyn Person>) {
            self.people.push(person);
        }

        // fn last_person(&self) -> Option<&Person> {
        fn last_person(&self) -> Option<&Box<dyn Person>> {
            self.people.last()
        }
    }

    #[test]
    fn run() {
        let mut zoo: PeopleZoo = PeopleZoo { people: vec![] };
        zoo.add_person(Box::new(Me { name: "Bennett" }));

        let me: &Me = zoo
            .last_person()
            .unwrap()
            .as_ref()
            .as_any()
            .downcast_ref::<Me>()
            .unwrap();
    }
}

mod generic {
    trait Person {
        fn say_hello(&self);
        // fn as_any(&self) -> &dyn Any;
    }

    struct PeopleZoo<P: Person> {
        people: Vec<P>,
    }

    impl<P: Person> PeopleZoo<P> {
        fn add_person(&mut self, person: P) {
            self.people.push(person);
        }

        fn last_person(&self) -> Option<&P> {
            self.people.last()
        }
    }

    #[derive(Debug)]
    struct Me {
        name: &'static str,
    }
    impl Person for Me {
        fn say_hello(&self) {
            println!("Hello, it's me.")
        }
    }

    #[test]
    fn run() {
        let mut zoo: PeopleZoo<Me> = PeopleZoo { people: vec![] };
        zoo.add_person(Me { name: "Bennett" });

        let me: &Me = zoo.last_person().unwrap();

        println!("{:#?}", me);
    }
}

mod enum_wrapper4trait_object {
    use super::Person;

    struct Grandma {
        age: usize,
    }

    impl Person for Grandma {
        fn say_hello(&self) {
            println!("G'day!")
        }
    }

    #[derive(Debug)]
    struct Me {
        name: &'static str,
    }
    impl Person for Me {
        fn say_hello(&self) {
            println!("Hello, it's me.")
        }
    }

    enum People {
        Grandma(Grandma),
        Me(Me),
    }

    impl Person for People {
        fn say_hello(&self) {
            match self {
                People::Grandma(grandma) => grandma.say_hello(),
                People::Me(me) => me.say_hello(),
            }
        }
    }


    struct PeopleZoo<P: Person> {
        people: Vec<P>,
    }

    impl<P: Person> PeopleZoo<P> {
        fn add_person(&mut self, person: P) {
            self.people.push(person);
        }

        fn last_person(&self) -> Option<&P> {
            self.people.last()
        }
    }

    #[test]
    fn run() {
        let mut zoo: PeopleZoo<People> = PeopleZoo { people: vec![] };
        zoo.add_person(People::Me(Me { name: "Bennett" }));

        if let Some(People::Me(me)) = zoo.last_person() {
            println!("My name is {}.", me.name)
        }
    }
}


mod box_dyn_trait_trick{
    trait Person {
        fn say_hello(&self);
    }
    
    /**
     * it's good practice to give a default implementation of your trait for it's boxed counterpart.
     * This can be done by calling as_ref or as_mut on the Box and calling the references relevant method.
     */
    impl Person for Box<dyn Person> {
        fn say_hello(&self) {
            self.as_ref().say_hello() 

        }
    }
    
    // fn main() {
    //   let mut zoo: PeopleZoo<Box<dyn Person>> = PeopleZoo {
    //     people: vec![]
    //   };
    // }
}