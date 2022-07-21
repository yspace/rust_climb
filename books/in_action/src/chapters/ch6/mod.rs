pub mod pointer_intro ;
pub mod ref_and_box ;
pub mod listing_6_3 ;
pub mod listing_6_4 ;
pub mod listing_6_5 ;
pub mod dereferencing ;
pub mod heap_via_box ;
// 下面这个模块因有全局内存分配器 所以会影响其他模块的运行的 排除掉！
// pub mod particles ;
pub mod memscan1 ;

pub fn main() {
    println!("in module {}",module_path!());
    // pointer_intro::main() ;
    // ref_and_box::main() ;
    // listing_6_3::main() ;
    // listing_6_4::main() ;
    // listing_6_5::main() ;
    // dereferencing::main() ; 
    // heap_via_box::main() ; 
    // particles::main() ;
    memscan1::main() ;
}