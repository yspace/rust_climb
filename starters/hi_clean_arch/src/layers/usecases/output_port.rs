use crate::layers::entities::User;

// 这里很奇怪 为啥不直接返回用户？
pub trait UserOutput {
    fn render(&self,user: &User) ;
}