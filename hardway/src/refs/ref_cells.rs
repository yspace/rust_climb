use std::{any::Any, cell::RefCell, collections::HashMap};

#[test]
fn test_refcell() {
    let x = RefCell::new(vec![1, 2, 3, 4]);
    {
        println!("{:?}", *x.borrow())
    }

    {
        let mut my_ref = x.borrow_mut();
        my_ref.push(1);
    }
}

mod ref_cell_map {
    use std::{cell::RefCell, collections::HashMap, rc::Rc};

    pub struct MyMap<T> {
        map: Rc<RefCell<HashMap<String, T>>>,
    }

    impl<T> MyMap<T> {
        pub fn with_value<F, O>(&self, key: &str, f: F) -> O
        where
            F: FnOnce(Option<&T>) -> O,
        {
            f(self.map.borrow().get(key))
        }
    }

    #[test]
    fn main() {
        let map: MyMap<u32> = MyMap {
            map: Rc::new(RefCell::new(HashMap::from([
                ("meaning".to_string(), 42),
                ("nice".to_string(), 69),
            ]))),
        };

        map.with_value("meaning", |value| {
            println!("{:?}", value);
        });
    }
}

#[derive(Debug, Default)]
pub struct Context {
    map: RefCell<HashMap<String, Box<dyn Any>>>,
}

impl Context {
    fn lookup(&self, k: impl Into<String>) -> Option<&Box<dyn Any>> {
        let k = k.into();
        let map = self.map.borrow();
        // map.get(&k).map(|x| x.clone())
        // map.get(&k).clone()
        // 这个问题应该是stackoverflow上面提到的 https://cloud.tencent.com/developer/ask/sof/107055674
        // 就是用 Ref::map 这个方法做

        // 还有一个回答是 多一个函数传递进来作为回调 即cps风格 Continuation-passing style

        // 不要跟csp 混淆了 csp常代表 Communicating Sequential Processes 是go语言的重要哲学思想https://www.kancloud.cn/mutouzhang/go/596820
        None
    }
}

#[test]
fn test_context() {
    fn do_something_with_context(cx: &Context) {}

    let ctxt = Context::default();

    do_something_with_context(&ctxt);
}

// @see https://lucumr.pocoo.org/2022/1/6/rust-extension-map/
mod extensions_1 {
    use std::any::{Any, TypeId};
    use std::collections::HashMap;

    #[derive(Default)]
    pub struct Extensions {
        map: HashMap<TypeId, Box<dyn Any>>,
    }

    impl Extensions {
        pub fn insert<T: 'static>(&mut self, value: T) {
            self.map.insert(TypeId::of::<T>(), Box::new(value));
        }

        pub fn get<T: 'static>(&self) -> &T {
            self.map
                .get(&TypeId::of::<T>())
                .and_then(|b| b.downcast_ref())
                .unwrap()
        }

        pub fn get_mut<T: Default + 'static>(&mut self) -> &mut T {
            self.ensure::<T>();
            self.map
                .get_mut(&TypeId::of::<T>())
                .and_then(|b| b.downcast_mut())
                .unwrap()
        }

        fn ensure<T: Default + 'static>(&mut self) {
            if self.map.get(&TypeId::of::<T>()).is_none() {
                self.insert(T::default());
            }
        }
    }
}

mod extensions_interior_mutability {
    use std::any::{Any, TypeId};
    use std::cell::{Ref, RefCell, RefMut};
    use std::collections::HashMap;

    #[derive(Default)]
    pub struct Extensions {
        map: RefCell<HashMap<TypeId, Box<dyn Any>>>,
    }

    impl Extensions {
        pub fn insert<T: 'static>(&self, value: T) {
            self.map
                .borrow_mut()
                .insert(TypeId::of::<T>(), Box::new(value));
        }

        pub fn get<T: Default + 'static>(&self) -> Ref<'_, T> {
            self.ensure::<T>();
            Ref::map(self.map.borrow(), |m| {
                m.get(&TypeId::of::<T>())
                    .and_then(|b| b.downcast_ref())
                    .unwrap()
            })
        }

        pub fn get_mut<T: Default + 'static>(&self) -> RefMut<'_, T> {
            self.ensure::<T>();
            RefMut::map(self.map.borrow_mut(), |m| {
                m.get_mut(&TypeId::of::<T>())
                    .and_then(|b| b.downcast_mut())
                    .unwrap()
            })
        }

        fn ensure<T: Default + 'static>(&self) {
            if self.map.borrow().get(&TypeId::of::<T>()).is_none() {
                self.insert(T::default());
            }
        }
    }
}

mod extensions_going_sync {
    use parking_lot::{
        MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
    };
    use std::any::{Any, TypeId};
    use std::collections::HashMap;

    #[derive(Default)]
    pub struct Extensions {
        map: RwLock<HashMap<TypeId, Box<dyn Any>>>,
    }

    impl Extensions {
        pub fn insert<T: Send + Sync + 'static>(&self, value: T) {
            self.map.write().insert(TypeId::of::<T>(), Box::new(value));
        }

        pub fn get<T: Send + Sync + Default + 'static>(&self) -> MappedRwLockReadGuard<'_, T> {
            self.ensure::<T>();
            RwLockReadGuard::map(self.map.read(), |m| {
                m.get(&TypeId::of::<T>())
                    .and_then(|b| b.downcast_ref())
                    .unwrap()
            })
        }

        pub fn get_mut<T: Send + Sync + Default + 'static>(&self) -> MappedRwLockWriteGuard<'_, T> {
            self.ensure::<T>();
            RwLockWriteGuard::map(self.map.write(), |m| {
                m.get_mut(&TypeId::of::<T>())
                    .and_then(|b| b.downcast_mut())
                    .unwrap()
            })
        }

        fn ensure<T: Default + Send + Sync + 'static>(&self) {
            if self.map.read().get(&TypeId::of::<T>()).is_none() {
                self.insert(T::default());
            }
        }
    }
}

mod exceptions_debuggable {
    // @see https://lucumr.pocoo.org/2022/1/7/as-any-hack/

    use std::any::{Any, TypeId};
    use std::cell::{Ref, RefCell, RefMut};
    use std::collections::HashMap;
    use std::fmt::Debug;

    trait DebugAny: Any + Debug {
        fn as_any(&self) -> &dyn Any;
        fn as_any_mut(&mut self) -> &mut dyn Any;
    }

    impl<T: Any + Debug + 'static> DebugAny for T {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[derive(Default, Debug)]
    pub struct Extensions {
        map: RefCell<HashMap<TypeId, Box<dyn DebugAny>>>,
    }

    impl Extensions {
        pub fn insert<T: Debug + 'static>(&self, value: T) {
            self.map
                .borrow_mut()
                .insert(TypeId::of::<T>(), Box::new(value));
        }

        pub fn get<T: Default + Debug + 'static>(&self) -> Ref<'_, T> {
            self.ensure::<T>();
            Ref::map(self.map.borrow(), |m| {
                m.get(&TypeId::of::<T>())
                    .and_then(|b| (**b).as_any().downcast_ref())
                    .unwrap()
            })
        }

        pub fn get_mut<T: Default + Debug + 'static>(&self) -> RefMut<'_, T> {
            self.ensure::<T>();
            RefMut::map(self.map.borrow_mut(), |m| {
                m.get_mut(&TypeId::of::<T>())
                    .and_then(|b| (**b).as_any_mut().downcast_mut())
                    .unwrap()
            })
        }

        fn ensure<T: Default + Debug + 'static>(&self) {
            if self.map.borrow().get(&TypeId::of::<T>()).is_none() {
                self.insert(T::default());
            }
        }
    }
}

mod retaining_type_name {
    use std::any::{type_name, TypeId};
    use std::fmt::{self, Debug};
    use std::hash::{Hash, Hasher};

    // 替换上面extetions 中的typeId
    pub struct TypeKey(TypeId, &'static str);

    impl TypeKey {
        pub fn of<T: 'static>() -> TypeKey {
            TypeKey(TypeId::of::<T>(), type_name::<T>())
        }
    }

    impl Hash for TypeKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl PartialEq for TypeKey {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Eq for TypeKey {}

    impl Debug for TypeKey {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.1)
        }
    }
}
