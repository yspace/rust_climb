use crate::user_module::domain::model::User;

use super::repository::IUserRepository;


pub trait IUser {
    fn list(&self) -> Result<Vec<User>,Box<dyn std::error::Error>> ;
}

struct UserUsecase{
    user_repository: Box<dyn IUserRepository>,
}

pub fn new_user_usecase(repo: Box<dyn IUserRepository>)-> Box<dyn IUser> {
    return  Box::new(UserUsecase{
        user_repository: repo,
    })
}

impl IUser for UserUsecase{
    fn list(&self) -> Result<Vec<User>,Box<dyn std::error::Error>>  {
        self.user_repository.find_all()
    }
}