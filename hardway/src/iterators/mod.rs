
pub fn main(){
    println!("hi iterators");
 
    let v = vec![1,2,3,4,5] ;
    // for item in v {
    //     dbg!(item) ;
    // }

    //  上面的仅是语法糖
    // let mut iter = v.into_iter() ;
    // while let Some(item) = iter.next() {
    //     dbg!(item) ;
    // }

    // let mut idx = 0 ;
    // while let Some(item) = v.get(idx) {
    //     idx +=1 ;
    //     dbg!(item) ;
    // }

    let mut vec2 = vec![] ;
    let mut idx = 0 ;
    while let Some(item) = v.get(idx) {
        idx += 1; 
        if item % 2  == 0 {
            vec2.push(item + 1) ;
        }
    }
    dbg!(vec2) ;

    // 
    chain_usage() ;
}

fn chain_usage() {
    let v1 = vec![1,2,3,4,5] ;
    let v2 = v1.into_iter()
        .filter(|item| item %2 == 0 )
        .map(|item| item + 1) ;

       // dbg!(v2) ;
        
    let v2 = v2.collect::<Vec<i32>>() ;
    dbg!(v2) ;

}

mod my1{
    use std::usize;

    trait Iterator{
        type Item ;
        fn next(&mut self)-> Option<Self::Item> ;
        fn take(self, count: usize)-> Take<Self>
        where 
        Self: Sized {
            Take{
                iter: self ,
                count ,
            }
        }
    }

    struct Take<I>{
        iter: I,
        count: usize ,
    }
    impl<I> Iterator for Take<I>
    where 
    I: Iterator {
        // type Item = I::Item ;
        type Item = <I as Iterator>::Item ;

        fn next(&mut self)-> Option<Self::Item>  {
          if self.count == 0 {
              None
          }else{
              self.count -= 1 ;
              self.iter.next()
          }
     }
    }

    struct  VecIter<T>{
        vec: Vec<T>,
    }

    impl<T> VecIter<T>{
        fn new(vec: Vec<T>) -> Self{
            Self{
                vec
            }
        }
    }

    impl<T> Iterator for VecIter<T> {
        type Item = T ;

        fn next(&mut self)-> Option<Self::Item>  {
          if self.vec.is_empty() {
              None
          }else{
              // 能工作但不高效
              Some(self.vec.remove(0))
          }
     }
    }

    fn repeat<T: Clone>(t: T) -> impl Iterator<Item = T>{
        Repeat(t)
    }

    struct Repeat<T>(T) ;

    impl<T: Clone> Iterator for Repeat<T> {
        type Item = T;

        fn next(&mut self)-> Option<Self::Item>  {
            Some(self.0.clone())
        }
    }

    #[cfg(test)]
    mod test{
        use super::* ;
        
        #[test]
         fn vec_iter(){
            let mut iter = VecIter::new(vec![1,2,3,4,5,6]) ;
            assert_eq!(iter.next(), Some(1)) ;
         }

         #[test]
         fn test_repeat(){
            let mut iter = repeat(1) ;
            assert_eq!(iter.next(), Some(1)) ;
         }
         #[test]
         fn repeat_take(){
            let mut iter = repeat(1).take(3) ;
            assert_eq!(iter.next(), Some(1)) ;
            assert_eq!(iter.next(), Some(1)) ;
            assert_eq!(iter.next(), Some(1)) ;
            assert_eq!(iter.next(), None) ;
         }
         #[test]
         fn vec_iter_take(){
            let mut iter = VecIter::new(
vec![1,2,3,4,5,6]
            ) .take(3) ;
            assert_eq!(iter.next(), Some(1)) ;
            assert_eq!(iter.next(), Some(2)) ;
            assert_eq!(iter.next(), Some(3)) ;
            assert_eq!(iter.next(), None) ;
         }
    }
}

