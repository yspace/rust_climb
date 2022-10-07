use fxhash::FxHashMap;
use std::any::{type_name, Any, TypeId};

// @see https://gitlab.redox-os.org/redox-os/dces-rust/-/blob/develop/src/resources.rs
// @see https://users.rust-lang.org/t/a-key-value-data-structure-keyed-by-trait/9579/2
// @see https://users.rust-lang.org/t/ask-for-dynamic-dispatch-in-rust-way/9137/18
// you could check: [typemap crate](https://github.com/reem/rust-typemap)
// 原来的typemap库没人维护了 新的： https://crates.io/crates/typemap_rev

// Every Component also implements Any trait
pub trait Component: Any {}

#[derive(Default)]
pub struct Registry {
    // 多线程环境试试 HashMap<TypeId, Box<(dyn Any + Send + Sync)>>
    resources: FxHashMap<TypeId, Box<dyn Any>>,
}

impl Registry {
    pub fn contains<C: Component>(&self) -> bool {
        self.resources.contains_key(&TypeId::of::<C>())
    }

    pub fn get<C: Component>(&self) -> &C {
        self.resources
            .get(&TypeId::of::<C>())
            .unwrap_or_else(|| panic!("Registry.get(): can't find type {}.", type_name::<C>()))
            .downcast_ref()
            .unwrap_or_else(|| {
                panic!(
                    "Registry.get(): cannot convert to type: {}",
                    type_name::<C>()
                )
            })
    }

    pub fn get_mut<C: Component>(&mut self) -> &mut C {
        self.resources
            .get_mut(&TypeId::of::<C>())
            .unwrap_or_else(|| {
                panic!(
                    "Registry.get(): type {} could not be found.",
                    type_name::<C>()
                )
            })
            .downcast_mut()
            .unwrap_or_else(|| {
                panic!(
                    "Registry.get(): cannot convert to type: {}",
                    type_name::<C>()
                )
            })
    }
    /// Inserts a new resource into the Registry map.
    pub fn insert<C: Component>(&mut self, resource: C) {
        self.resources.insert(TypeId::of::<C>(), Box::new(resource));
    }

    /// Returns true if the resources contains no elements.
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    /// Returns the number of elements in the resources.
    pub fn len(&self) -> usize {
        self.resources.len()
    }

    /// Creates a service resources with an empty Registry map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Try to get an element from the resources.
    pub fn try_get<C: Component>(&self) -> Option<&C> {
        if let Some(e) = self.resources.get(&TypeId::of::<C>()) {
            if let Some(r) = e.downcast_ref() {
                return Some(r);
            }
        }

        None
    }

    /// Try to get an element from the resources.
    pub fn try_get_mut<C: Component>(&mut self) -> Option<&mut C> {
        if let Some(e) = self.resources.get_mut(&TypeId::of::<C>()) {
            if let Some(r) = e.downcast_mut() {
                return Some(r);
            }
        }

        None
    }
}

mod internal {
    use std::any::{type_name, Any, TypeId};
    // https://willcrichton.net/rust-api-type-patterns/registries.html
    use std::collections::HashMap;
    struct TypeMap(HashMap<TypeId, Box<dyn Any>>);
    impl TypeMap {
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
}


#[cfg(test)]
mod tests {
    use super::*;

    struct ServiceOne;
    struct ServiceTwo;

    impl Component for ServiceOne {}
    impl Component for ServiceTwo {}

    #[test]
    fn insert() {
        let mut resources = Registry::new();
        resources.insert(ServiceOne);
        resources.insert(ServiceTwo);

        assert!(resources.try_get::<ServiceOne>().is_some());
        assert!(resources.try_get::<ServiceTwo>().is_some());
    }

    #[test]
    fn try_get_mut() {
        let mut resources = Registry::new();
        resources.insert(ServiceOne);
        resources.insert(ServiceTwo);

        assert!(resources.try_get_mut::<ServiceOne>().is_some());
        assert!(resources.try_get_mut::<ServiceTwo>().is_some());
    }

    #[test]
    fn contains() {
        let mut resources = Registry::new();
        resources.insert(ServiceOne);

        assert!(resources.contains::<ServiceOne>());
        assert!(!resources.contains::<ServiceTwo>());
    }

    #[test]
    fn len() {
        let mut resources = Registry::new();
        assert_eq!(resources.len(), 0);

        resources.insert(ServiceOne);
        assert_eq!(resources.len(), 1);

        resources.insert(ServiceTwo);
        assert_eq!(resources.len(), 2);
    }

    #[test]
    fn is_empty() {
        let mut resources = Registry::new();
        assert!(resources.is_empty());

        resources.insert(ServiceOne);
        assert!(!resources.is_empty());
    }


}