
#[derive(Debug)]
pub struct RandomInfo{
    pub some_bool: bool,
    pub some_int: i64,
}

impl RandomInfo {
    pub fn new(some_bool: bool) -> Self {
        Self { 
            some_bool,
             some_int: 0
             }
    }

    pub fn is_smaller(&self, other: &Self) -> bool {
        self.some_int < other.some_int
    }
}

pub trait SomeTrait{
     fn is_valid(&self) -> bool ;
}

impl SomeTrait for RandomInfo{
     fn is_valid(&self) -> bool  {
       self.some_bool
    }
}

pub fn print_if_is_valid(check_me: & dyn SomeTrait){
   if check_me.is_valid() {
        println!("ok it is valid") ;
   }
}

// ### 内置的常用trait
impl  Default for RandomInfo{
    fn default() -> Self {
        Self { some_bool: Default::default(), some_int: Default::default() }
    }
}

#[test]
pub fn test_is_valid() {
    let r1 = RandomInfo::new(true) ;
    print_if_is_valid(&r1) ;
}

#[test]
pub fn test_is_smaller(){
    let r1 = RandomInfo::new(true) ;
    let mut r2 = RandomInfo::new(true) ;
    r2.some_int = 2 ;

    assert!(r1.is_smaller(&r2));
}

#[test]
pub fn test_is_larger(){
    // 此功能只限此函数中 
    impl RandomInfo {
        pub fn is_larger(&self, other: &Self) -> bool {
            &self.some_int > &other.some_int 
        }
    }


    let r1 = RandomInfo::new(true) ;
    let mut r2 = RandomInfo::new(true) ;
    r2.some_int = 2 ;

    assert!(r2.is_larger(&r1));
}