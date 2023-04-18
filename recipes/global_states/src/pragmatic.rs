// @see https://adventures.michaelfbryan.com/posts/pragmatic-global-state/

use std::marker::PhantomData;

use std::sync::atomic::{AtomicBool, Ordering};

static LIBRARY_IN_USE: AtomicBool = AtomicBool::new(false);


/// A handle to the `stateful` library.
pub struct Library {
    _not_send: PhantomData<*const ()>,
}


impl Library {
    pub fn new() -> Result<Library, Error> {
        if LIBRARY_IN_USE.compare_and_swap(false, true, Ordering::SeqCst)
            == false
        {
            Ok(Library { _not_send: PhantomData })
        } else {
            Err(Error::AlreadyInUse)
        }
    }

    pub fn set_parameters(&mut self) -> SettingParameters<'_> {
        SettingParameters { _library: self }
    }

    pub fn create_recipe(&mut self) -> RecipeBuilder<'_> {
        RecipeBuilder { _library: self }
    }
}

pub struct SettingParameters<'lib> {
    _library: &'lib mut Library,
}

impl<'lib> SettingParameters<'lib> {
    pub fn boolean(&mut self, _name: &str, _value: bool) -> &mut Self {
        unimplemented!()
    }

    pub fn integer(&mut self, _name: &str, _value: i32) -> &mut Self {
        unimplemented!()
    }
}

pub struct RecipeBuilder<'lib> {
    _library: &'lib mut Library,
}

impl<'lib> RecipeBuilder<'lib> {
    pub fn add_item(&mut self, _name: &str, _value: i32) -> &mut Self {
        unimplemented!()
    }
}

impl Drop for Library {
    fn drop(&mut self) { LIBRARY_IN_USE.store(false, Ordering::SeqCst); }
}

// unsafe impl Send for Library {}



/// The various error cases that may be encountered while using this library.
#[derive(Debug, Copy, Clone, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("The library is already in use")]
    AlreadyInUse,
}

#[cfg(test)]
mod tests {
    use super::*;

    static_assertions::assert_not_impl_any!(Library: Send, Sync);

    #[test]
    fn cant_create_multiple_library_handles_at_the_same_time() {
        let first_library = Library::new().unwrap();

        // make sure the flag is set
        assert!(LIBRARY_IN_USE.load(Ordering::SeqCst));

        // then try to create another handle
        assert!(Library::new().is_err());

        // explicitly drop the first library so we know it clears the flag
        drop(first_library);

        assert!(!LIBRARY_IN_USE.load(Ordering::SeqCst));

        // now the old handle is destroyed, we can create another
        let _another = Library::new().unwrap();
    }

}