
pub fn main(){

}

fn borrow(){
    // 我们需要“可变的”借用指针
    fn foo(v: &mut Vec<i32>){
        v.push(5) ;
    }

    //
    let mut v = vec![] ;
    foo(&mut v) ;
}