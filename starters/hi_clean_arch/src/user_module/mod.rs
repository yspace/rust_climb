use self::{infrastructure::{datastore::new_db, router::build_router}, registry::new_registry};

mod adapter ;
mod domain ;
mod infrastructure ;
mod usecase ;

// The registry handles resolving dependencies between ports and adapters, using constructor injection.
mod registry ;

pub fn boot(){
    let db = new_db() ;

    let registry = new_registry(db) ;

     build_router(registry.new_app_controller());

     // to be continue ...
}