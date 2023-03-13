use crate::{user_module::usecase::usecase::IUser, Context};

pub trait IUserController {
    fn get_users(&self, ctx: &mut Context) -> Result<(), Box<dyn std::error::Error>>;
}

struct UserController {
    user_usecase: Box<dyn IUser>,
}

pub fn new_user_controller(user_usecase: Box<dyn IUser>) -> Box<dyn IUserController> {
    Box::new(UserController { user_usecase })
}


impl IUserController for UserController {
    fn get_users(&self, ctx: &mut Context) -> Result<(), Box<dyn std::error::Error>> {
       let results =  self.user_usecase.list()?;

        // use context to render the users to clients ;
       return Ok(());

    }
}


// 作为应用控制器 聚集所有其他控制器
pub struct AppController{
    user_controller: Box<dyn IUserController>,
}
impl AppController {
    pub fn new(user_controller: Box<dyn IUserController>)-> Self{
        Self{
            user_controller: user_controller
        }
    }
}