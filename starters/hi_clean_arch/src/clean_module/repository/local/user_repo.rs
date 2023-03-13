use std::collections::HashMap;

use crate::clean_module::{models::User, errors::BoxedDynStdError, repository::IUserRepo};


pub fn new_user_local_storage()-> Result<Box<dyn IUserRepo>, BoxedDynStdError>{
    
    Ok(
        Box::new(
            UserLocalStorage::new()
        )
    )
}

#[derive(Debug, Default)]
struct UserLocalStorage{
    users : HashMap<String,User>
}

impl UserLocalStorage {
    pub fn new()-> Self {
        // todo：实例化成员变量
        Default::default()
    }
}

impl IUserRepo for UserLocalStorage {
    fn create(&self,user: User)->Result<(),Box<dyn std::error::Error>> {
        todo!()
    }

    fn get_user(&self,name:String)->Result<User,BoxedDynStdError>  {
        todo!()
    }
}