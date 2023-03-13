mod js_like {
    use std::{any::Any, collections::HashMap};

    type EventListener = Box<dyn Fn(&dyn Any)>;

    #[derive(Default)]
    struct EventRegistry {
        listeners: HashMap<String, Vec<EventListener>>,
    }

    impl EventRegistry {
        fn add_event_listener(&mut self, event: String, f: EventListener) {
            self.listeners.entry(event).or_insert_with(Vec::new).push(f);
        }

        fn trigger(&self, event: String, data: &dyn Any) {
            let listeners = self.listeners.get(&event).unwrap();
            for listener in listeners.iter() {
                listener(data);
            }
        }
    }

    struct OnClick {
        mouse_x: f32,
        mouse_y: f32,
    }

    #[test]
    fn main() {
        let mut events = EventRegistry::default();
        events.add_event_listener(
            "click".to_owned(),
            Box::new(|event| {
                let event = event.downcast_ref::<OnClick>().unwrap();
                assert_eq!(event.mouse_x, 1.);
            }),
        );

        let event = OnClick {
            mouse_x: 1.,
            mouse_y: 3.,
        };
        events.trigger("click".to_owned(), &event);
    }
}

mod type_safe_events {
    use std::collections::HashMap;

    trait Event = 'static;
    // trait Event{}
    trait EventListener<E> = Fn(&E) -> () + 'static;

    struct EventRegistry<E> {
        listeners: HashMap<String, Vec<Box<dyn EventListener<E>>>>,
    }

    // impl EventRegistry<E> {
    //     fn add_event_listener<E: Event>(&mut self, f: impl EventListener<E>) {
    //         /* .. */
    //     }

    //     fn trigger<E: Event>(&self, event: &E) {
    //         /* .. */
    //     }
    // }
}

mod types_to_values {
    use std::any::{Any, TypeId};
    use std::collections::HashMap;

    #[derive(Debug, Default)]
    struct TypeMap(HashMap<TypeId, Box<dyn Any>>);

    impl TypeMap {
        pub fn new() -> Self {
            Default::default()
        }
        pub fn set<T: Any + 'static>(&mut self, t: T) {
            self.0.insert(TypeId::of::<T>(), Box::new(t));
        }
    }

    impl TypeMap {
        pub fn has<T: Any + 'static>(&self) -> bool {
            self.0.contains_key(&TypeId::of::<T>())
        }

        pub fn get_mut<T: Any + 'static>(&mut self) -> Option<&mut T> {
            self.0
                .get_mut(&TypeId::of::<T>())
                .map(|t| t.downcast_mut::<T>().unwrap())
        }
    }

    #[test]
    fn test_run() {
        let mut map = TypeMap::new();
        map.set::<i32>(1);
    }

    // 实现事件系统

/*
    trait Event = 'static;
    // trait Event{}
    trait EventListener<E> = Fn(&E) -> () + 'static;

    struct EventDispatcher(TypeMap);
    type ListenerVec<E> = Vec<Box<dyn EventListener<E>>>;

    impl EventDispatcher {
        fn add_event_listener<E>(&mut self, f: impl EventListener<E>) {
            if !self.0.has::<ListenerVec<E>>() {
                self.0.set::<ListenerVec<E>>(Vec::new());
            }

            let listeners = self.0.get_mut::<ListenerVec<E>>().unwrap();
            listeners.push(Box::new(f));
        }

        fn trigger<E>(&self, event: &E) {
            if let Some(listeners) = self.0.get_mut::<ListenerVec<E>>() {
                for callback in listeners {
                    callback(event);
                }
            }
        }
    }
     */
}
