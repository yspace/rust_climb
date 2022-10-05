
 use whale::route::router::Router ;


 pub fn configure(router: &mut Router) {
     router.insert("/", || {
         println!("fn: /");
     });
     
     
 }