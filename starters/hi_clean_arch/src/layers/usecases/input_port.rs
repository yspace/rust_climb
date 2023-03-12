use crate::layers::entities::User;

// 实质是用例层外面的接口层 用来隔离用例层的 
pub trait UserInput {

    fn get(&self,id: i64) -> Result<(),Box<dyn std::error::Error>>;
}


pub trait UserRepository {
   fn get_by_id(&self,id: i64) -> Result<User,Box<dyn std::error::Error>> ;
}