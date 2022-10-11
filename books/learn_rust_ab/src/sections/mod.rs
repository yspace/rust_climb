use whale::route::router::Router;

mod data_types; 


pub fn register_routes(router: &mut Router) {
    router.insert("/index", || {
        println!("fn: /index");
    });
    router.insert("data_types", || {
        println!("data types");
        data_types::run();
    });
}