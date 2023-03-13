use crate::user_module::{adapter::{controller::{IUserController, new_user_controller}, repository::new_user_repo}, usecase::usecase::new_user_usecase};


impl  super::Registry {

  pub fn new_user_controller(&self)-> Box< dyn IUserController> {
    let user_repo = new_user_repo(
        // self.db
    ) ;
   
    let user_usecase = 
    new_user_usecase(user_repo);
    new_user_controller(user_usecase)
  }   
}