pub fn main(){

    // 每个十六进制数字表示4bits 
let opcode: u16 = 0x71E4 ;
// & 操作相当于位置过滤器 只保留位置是1的数据
let c = (opcode & 0xF000) >> 12 ;
let x = (opcode & 0x0F00) >> 8 ;
let y = (opcode & 0x00F0) >> 4 ;
let d = (opcode & 0x000F) >> 0 ;

assert_eq!(c, 0x7);
assert_eq!(x, 0x1);
assert_eq!(y, 0xE);
assert_eq!(d, 0x4);

let nnn = opcode & 0x0FFF;
let kk = opcode & 0x00FF;

assert_eq!(nnn, 0x1E4);
assert_eq!(kk, 0xE4);

}