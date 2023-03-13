use super::{infrastructure::datastore::DataStore, adapter::controller::{AppController, new_user_controller}};


mod user ;

pub trait  IRegistry {

     fn new_app_controller(&self) -> AppController ;
    
}

pub struct Registry{
    db: DataStore
}

pub fn new_registry(db: DataStore) -> Box<dyn IRegistry>{
    Box::new(Registry{db})
}

impl IRegistry for Registry{
    fn new_app_controller(&self) -> AppController  {
        return AppController::new(self.new_user_controller());
    }
}