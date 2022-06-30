
 use whale::route::router::Router ;


 pub fn configure(router: &mut Router) {
     router.insert("/", || {
         println!("fn: /");
     });
     router.insert("ch1", || {
         println!("chapters/ch1");
         crate::chapters::ch1::run() ;
     });
     router.insert("ch2", || {
         println!("chapters/ch2"); 
         crate::chapters::ch2::ok::main() ;

         // println!("{:?}",router.descendents("ch2"));
        //  crate::chapters::ch2::comparisons::main() ;
        //  crate::chapters::ch2::add_floats::main() ;
         crate::chapters::ch2::complex::main() ;
     });
     router.insert("ch2/first_steps", || {
         println!("chapters/ch2");
         crate::chapters::ch2::first_steps::main();
     });
     router.insert("ch2/intro-to-numbers", || {
         println!("chapters/ch2");
         crate::chapters::ch2::intro_to_numbers::main();
     });
     router.insert("ch2/non-base2", || {
         println!("chapters/ch2");
         crate::chapters::ch2::non_base2::main();
     });
 }