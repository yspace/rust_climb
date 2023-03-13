pub mod local;
pub mod mongo;

use super::{models::User, errors::BoxedDynStdError};

pub trait IUserRepo {
    fn create(&self,user: User)->Result<(),Box<dyn std::error::Error>>;

    fn get_user(&self,name:String)->Result<User,BoxedDynStdError> ;
}
