### Generic Types vs Associated Types

Both generic types and associated types defer the decision to the implementer on which concrete types should be used in the trait's functions and methods, so this section seeks to explain when to use one over the other.

The general rule-of-thumb is:

    Use associated types when there should only be a single impl of the trait per type.
    Use generic types when there can be many possible impls of the trait per type.


### prelude

The standard library prelude is a module in the standard library, i.e. std::prelude::v1, that gets auto imported at the top of every other module, i.e. use std::prelude::v1::*. Thus the following traits are always in scope and we never have to explicitly import them ourselves because they're part of the prelude:

    AsMut
    AsRef
    Clone
    Copy
    Default
    Drop
    Eq
    Fn
    FnMut
    FnOnce
    From
    Into
    ToOwned
    IntoIterator
    Iterator
    PartialEq
    PartialOrd
    Send
    Sized
    Sync
    ToString
    Ord
