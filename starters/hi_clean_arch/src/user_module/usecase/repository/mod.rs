use crate::user_module::domain::model::User;


pub trait IUserRepository{
    fn find_all(&self) -> Result<Vec<User> , Box<dyn std::error::Error>>;
}