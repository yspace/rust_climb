
 use whale::route::router::Router ;


 pub fn configure(router: &mut Router) {
     router.insert("/", || {
         println!("fn: /");
     });
    
     router.insert("ch6", || {
         crate::chapters::ch6::run();
     });
 }