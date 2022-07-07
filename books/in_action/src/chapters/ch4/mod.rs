//! Lifetimes, ownership,and borrowing.

mod check_sats_1;
// mod check_sats_2;
mod check_sats_3;

// mod reduce_access_level;
mod sat_mailbox;
mod resolving_ownership_issues;
mod short_lived_strategy;
mod impl_copy;
mod impl_copy_manually;
mod rc_groundstation;
mod interior_mutability;

pub fn main() {
    // check_sats_1::main() ;
    // check_sats_2::main() ;
    // check_sats_3::main() ;
    // sat_mailbox::main() ;
    // resolving_ownership_issues::main() ;
    // short_lived_strategy::main() ;
    // impl_copy::main() ;
    // impl_copy_manually::main() ;
    // rc_groundstation::main() ;
    interior_mutability::main() ;
}