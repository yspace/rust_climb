pub trait Sort {
    /// Re-arrange contents into sorted order.
    fn sort(&mut self);
}

/**
 * A marker trait has no methods, but an implementation still has to declare that it is implementing the trait – which acts as a promise from the implementer: "I solemnly swear that my implementation sorts stably". Code that relies on a stable sort can then specify the StableSort trait bound, relying on the honour system to preserve its invariants. Use marker traits to distinguish behaviours that cannot be expressed in the trait method signatures.
 */
/// Marker trait to indicate that a [`Sortable`] sorts stably.
pub trait StableSort: Sort {}
