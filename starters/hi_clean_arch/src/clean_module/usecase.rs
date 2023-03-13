pub mod user_interactor;

pub trait IUserInteractor{

   fn  sign_up(
    /*ctx context.Context,*/ 
    username: String, 
    password: String) ;
}