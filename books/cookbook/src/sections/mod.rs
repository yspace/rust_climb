use whale::route::router::Router;

mod algorithms;


pub fn register_routes(router: &mut Router) {
    router.insert("/index", || {
        println!("fn: /index");
    });
}