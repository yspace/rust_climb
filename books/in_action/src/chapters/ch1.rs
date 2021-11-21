
pub fn run(){
    greet_world() ;
    listing1_2::run() ;
    listing1_3::run() ;
    // listing1_5::run() ;
    listing_1_7::run() ;
}

fn greet_world(){
    println!("hello, world!") ;
    let southen_germany = "xxxx";
    let japan = "小 日 本" ;
    let regions = [
      southen_germany ,
      japan
    ] ;
    for region in regions.iter() {
        println!("{}",region) ;
    }
}

mod listing1_2{
    pub fn run(){
        let penguin_data = "\
        common name, length (cm)
        Little penguin,33
        yellow-eyed penguin,65
        Fiordland penguin,60
        Invalid,data       
        " ;

        let records = penguin_data.lines() ;

        for (i, record) in records.enumerate() {
            if i ==0 || record.trim().len() == 0 {
                continue;
            } // 空行跳过 第一行跳过

            let fields: Vec<_> = record
            .split(',')
            .map(|field| field.trim() )
            .collect() ;

            if cfg!(debug_assertions) {
                eprintln!("debug: {:?} => {:?} ",        record , fields);
            }

            let name = fields[0] ;
            if let Ok(length) = fields[1].parse::<f32>() {
                println!("{}, {}cm", name, length) ;
            }

        }
    }
}

mod listing1_3{
    pub fn run (){
        let mut grains: Vec<Cereal> = vec![];
        grains.push(Cereal::Rye);
        // drop(grains);
        println!("{:?}", grains) ;
    }

    #[derive(Debug)]
    enum Cereal {
        Barley, Millet, Rice,
        Rye, Spelt, Wheat, 
    }

}


mod listing1_4{
    use std::thread;

    pub fn run(){
        // let mut data = 100 ;

        // thread::spawn(||{
        //     data = 500 ;
        // }) ;

        // thread::spawn(||{
        //     data = 1000 ;
        // }) ;

        // println!("{}", data) ;
    }
}

mod listing1_5{
    pub fn run(){
        let fruit = vec![
            'a','b','c'
        ];
        let buffer_overflow = fruit[4] ;
        assert_eq!(buffer_overflow , 'd') ;
    }
}

mod listing1_6{
    pub fn run(){
        let mut letters = vec![
"a","b","c"
        ];

        for letter in letters {
            println!("{}", letter) ;
           // letters.push(letter.clone()) ;
        }
    }
}
mod listing1_6_1{
    pub fn run(){
        let a = 10 ;

        if a == 10 {
            println!("a equals ten") ;
        }
    }
}

mod listing_1_7{
    use std::rc::Rc ;
    use std::sync::{
        Arc, Mutex
    };
    pub fn run(){
        let a = 10 ;
        let b = Box::new(20) ;
        let c = Rc::new(Box::new(30)) ;
        let d = Arc::new(Mutex::new(40)) ;

        println!("a: {:?} , b: {:?}, c: {:?}, d: {:?}",a,b,c,d);
    }
}