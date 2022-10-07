use whale::route::router::Router;

mod ch10 ;
mod ch11 ;

pub fn register_routes(router: &mut Router) {
    router.insert("/index", || {
        println!("fn: /index");
    });
    router.insert("ch10", || {
        println!("ch10");
        ch10::run() ;
    });
    router.insert("ch11", || {
        println!("ch11");
        ch11::run() ;
    });
}