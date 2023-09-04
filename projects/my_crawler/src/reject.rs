use std::any::Any;
use std::convert::Infallible;
use std::error::Error as StdError;
use std::fmt;



// pub(crate) use self::sealed::{CombineRejection, IsReject};





/// Rejects a request with a custom cause.
///
/// A [`recover`][] filter should convert this `Rejection` into a `Reply`,
/// or else this will be returned as a `500 Internal Server Error`.
///
/// [`recover`]: ../trait.Filter.html#method.recover
pub fn custom<T: Reject>(err: T) -> Rejection {
    Rejection::custom(Box::new(err))
}

/// Protect against re-rejecting a rejection.
///
/// ```compile_fail
/// fn with(r: warp::Rejection) {
///     let _wat = warp::reject::custom(r);
/// }
/// ```
fn __reject_custom_compilefail() {}

/// A marker trait to ensure proper types are used for custom rejections.
///
/// Can be converted into Rejection.
///
/// # Example
///
/// ```
/// use warp::{Filter, reject::Reject};
///
/// #[derive(Debug)]
/// struct RateLimited;
///
/// impl Reject for RateLimited {}
///
/// let route = warp::any().and_then(|| async {
///     Err::<(), _>(warp::reject::custom(RateLimited))
/// });
/// ```
// Require `Sized` for now to prevent passing a `Box<dyn Reject>`, since we
// would be double-boxing it, and the downcasting wouldn't work as expected.
pub trait Reject: fmt::Debug + Sized + Send + Sync + 'static {}

trait Cause: fmt::Debug + Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;
}

impl<T> Cause for T
where
    T: fmt::Debug + Send + Sync + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl dyn Cause {
    fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

// pub(crate) fn known<T: Into<Known>>(err: T) -> Rejection {
//     Rejection::known(err.into())
// }

/// Rejection of a request by a [`Filter`](crate::Filter).
///
/// See the [`reject`](module@crate::reject) documentation for more.
pub struct Rejection {
    reason: Reason,
}

enum Reason {
    NotFound,
    Other(Box<Rejections>),
}

enum Rejections {
    Known(Known),
    Custom(Box<dyn Cause>),
    Combined(Box<Rejections>, Box<Rejections>),
}

macro_rules! enum_known {
     ($($(#[$attr:meta])* $var:ident($ty:path),)+) => (
        pub(crate) enum Known {
            $(
            $(#[$attr])*
            $var($ty),
            )+
        }

        impl Known {
            fn inner_as_any(&self) -> &dyn Any {
                match *self {
                    $(
                    $(#[$attr])*
                    Known::$var(ref t) => t,
                    )+
                }
            }
        }

        impl fmt::Debug for Known {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match *self {
                    $(
                    $(#[$attr])*
                    Known::$var(ref t) => t.fmt(f),
                    )+
                }
            }
        }

        impl fmt::Display for Known {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match *self {
                    $(
                    $(#[$attr])*
                    Known::$var(ref t) => t.fmt(f),
                    )+
                }
            }
        }

        $(
        #[doc(hidden)]
        $(#[$attr])*
        impl From<$ty> for Known {
            fn from(ty: $ty) -> Known {
                Known::$var(ty)
            }
        }
        )+
    );
}

// enum_known! {
//     MethodNotAllowed(MethodNotAllowed),
//     InvalidHeader(InvalidHeader),
//     MissingHeader(MissingHeader),
//     MissingCookie(MissingCookie),
//     InvalidQuery(InvalidQuery),
//     LengthRequired(LengthRequired),
//     PayloadTooLarge(PayloadTooLarge),
//     UnsupportedMediaType(UnsupportedMediaType),
//     FileOpenError(crate::fs::FileOpenError),
//     FilePermissionError(crate::fs::FilePermissionError),
//     BodyReadError(crate::body::BodyReadError),
//     BodyDeserializeError(crate::body::BodyDeserializeError),
//     CorsForbidden(crate::cors::CorsForbidden),
//     #[cfg(feature = "websocket")]
//     MissingConnectionUpgrade(crate::ws::MissingConnectionUpgrade),
//     MissingExtension(crate::ext::MissingExtension),
//     BodyConsumedMultipleTimes(crate::body::BodyConsumedMultipleTimes),
// }

impl Rejection {
    fn known(known: Known) -> Self {
        Rejection {
            reason: Reason::Other(Box::new(Rejections::Known(known))),
        }
    }

    fn custom(other: Box<dyn Cause>) -> Self {
        Rejection {
            reason: Reason::Other(Box::new(Rejections::Custom(other))),
        }
    }

    
    
    pub fn find<T: 'static>(&self) -> Option<&T> {
        if let Reason::Other(ref rejections) = self.reason {
            return rejections.find();
        }
        None
    }
   
}

