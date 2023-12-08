pub mod app_data;

//copy from https://docs.rs/actix-web/latest/src/actix_web/data.rs.html#90
use std::{any::type_name, ops::Deref, sync::Arc};
use seahorse::extensions::Extensions;

// use actix_http::Extensions;
use serde::{de, Serialize};
// use futures_core::future::LocalBoxFuture;

// use crate::{dev::Payload, error, Error, FromRequest, HttpRequest};

/// Data factory.
pub(crate) trait DataFactory {
    /// Return true if modifications were made to extensions map.
    fn create(&self, extensions: &mut Extensions) -> bool;
}

// pub(crate) type FnDataFactory =
// Box<dyn Fn() -> LocalBoxFuture<'static, Result<Box<dyn DataFactory>, ()>>>;


#[doc(alias = "state")]
#[derive(Debug)]
pub struct Data<T: ?Sized>(Arc<T>);

impl<T> Data<T> {
    /// Create new `Data` instance.
    pub fn new(state: T) -> Data<T> {
        Data(Arc::new(state))
    }
}

impl<T: ?Sized> Data<T> {
    /// Returns reference to inner `T`.
    pub fn get_ref(&self) -> &T {
        self.0.as_ref()
    }

    /// Unwraps to the internal `Arc<T>`
    pub fn into_inner(self) -> Arc<T> {
        self.0
    }
}

impl<T: ?Sized> Deref for Data<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Arc<T> {
        &self.0
    }
}

impl<T: ?Sized> Clone for Data<T> {
    fn clone(&self) -> Data<T> {
        Data(Arc::clone(&self.0))
    }
}

impl<T: ?Sized> From<Arc<T>> for Data<T> {
    fn from(arc: Arc<T>) -> Self {
        Data(arc)
    }
}

impl<T: Default> Default for Data<T> {
    fn default() -> Self {
        Data::new(T::default())
    }
}

impl<T> Serialize for Data<T>
    where
        T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de, T> de::Deserialize<'de> for Data<T>
    where
        T: de::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
    {
        Ok(Data::new(T::deserialize(deserializer)?))
    }
}


impl<T: ?Sized + 'static> DataFactory for Data<T> {
    fn create(&self, extensions: &mut Extensions) -> bool {
        extensions.insert(Data(self.0.clone()));
        true
    }
}