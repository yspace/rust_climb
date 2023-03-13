use crate::user_module::{usecase::repository::IUserRepository, domain::model::User};


struct  UserRepository{
    // db: db::Database
}

pub fn new_user_repo(/*db: db::Database*/) -> Box<dyn IUserRepository>{
    Box::new( UserRepository{
        // db: db
    })
}

impl IUserRepository for UserRepository{
    fn find_all(&self) -> Result<Vec<User> , Box<dyn std::error::Error>> {
      
        // todo: should query users from db ;
       Ok(vec![
        User::default(),
       ])
    }
}