
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
        //  crate::chapters::ch2::complex::main() ;
        //  crate::chapters::ch2::flow_control::main() ;
        //  crate::chapters::ch2::functions::main() ;
        //  crate::chapters::ch2::functions::main() ;
        //  crate::chapters::ch2::explicit_lifetime_annotations::main() ;
        //  crate::chapters::ch2::generic_functions::main() ;
        crate::chapters::ch2::main() ;
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
     router.insert("ch3", || {
         //println!("chapters/ch3");
         crate::chapters::ch3::main();
     });
     router.insert("ch4", || {
         crate::chapters::ch4::main();
     });
     router.insert("ch5", || {
         crate::chapters::ch5::main();
     });
     router.insert("ch6", || {
         crate::chapters::ch6::main();
     });
     router.insert("ch7", || {
         crate::chapters::ch7::main();
     });
     router.insert("ch8", || {
         crate::chapters::ch8::main();
     });
     router.insert("ch9", || {
         crate::chapters::ch9::main();
     });
     router.insert("ch10", || {
         crate::chapters::ch10::main();
     });
 }