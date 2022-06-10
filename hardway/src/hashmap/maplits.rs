use std::collections::HashMap ;
// 字面量库 弥补rust中没有哈希字面量构造的缺陷
use maplit::* ;

// fn all_instructions() -> HashMap<Instr,u64>{
fn all_instructions() -> HashMap<&'static str,u64>{
    maplit::hashmap! {
        // Instr::Add => 0,
        // Instr::Sub => 1,
        // Instr::Mul => 2,
        // Instr::Div => 3,
        // Instr::Mod => 4,
        // Instr::And => 5,
        // Instr::Or => 6,
        // Instr::Xor => 7,
        // Instr::Shl => 8,
        // Instr::Shr => 9,
        // Instr::Not => 10,
        // Instr::Neg => 11,
        // Instr::Mov => 12,
        // Instr::Load => 13,
        // Instr::Store => 14,
        // Instr::Call => 15,
        // Instr::Ret => 16,
        "Add" => 0,
        "Sub" => 1,
    }
    
     
}