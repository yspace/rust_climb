// @see https://github.com/rust-in-action/code/blob/1st-edition/ch5/ch5-cpu4/src/main.rs

struct CPU {
    // 16bits = 2bytes    十六进制 0-F 刚好可以表示这十六个寄存器索引
    registers: [u8; 16],
    // 此即 `program counter`
    position_in_memory: usize,
    // 4096 bytes 2的12次方是4096
    memory: [u8; 0x1000],

    // 最大高度就16 在16个嵌套方法调用后程序会遭遇栈溢出
    stack: [u16; 16],
    // usize类型可以在栈中方便索引某值
    stack_pointer: usize,
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        (op_byte1 << 8) | (op_byte2)
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            // 递增pc 以指向到下个指令
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            let nnn = opcode & 0x0FFF ;
            // let kk = (opcode & 0x00FF) as u8 ;

            match (c, x, y, d) {
                // 短路掉函数来终结执行 当遇到opcode 为0x0000时
                (0, 0, 0, 0) => {
                    return;
                }
                (0,0,0xE, 0xE) => self.ret(),
                (0x2, _,_,_) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x} ,parsed as: {:?}", opcode, (c, x, y, d)),
                // _ => println!("opcode {:04x} (c,x,y,d) is {:?}", opcode,),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        // CHIP-8 中 最后一个寄存器作为一个 carry flag
        // 当被设置时 此flag指示出一个操作已经溢出u8寄存器的大小了
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn call(&mut self, address: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack ;

        if sp > stack.len() {
            panic!("stack overflow");
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = address as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("stack underflow");
        }

        self.stack_pointer -= 1;
        let call_addr = self.stack[self.stack_pointer];
        self.position_in_memory = call_addr as usize;
    }
}

pub fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0 ,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
     

    let mem = &mut cpu.memory;
    mem[0x000] = 0x21 ; mem[0x001] = 0x00 ;
    mem[0x002] = 0x21 ; mem[0x003] = 0x00 ;
    mem[0x004] = 0x00 ; mem[0x005] = 0x00 ;

    mem[0x100] = 0x80 ; mem[0x101] = 0x14 ;
    mem[0x102] = 0x80 ; mem[0x103] = 0x14 ;
    mem[0x104] = 0x00 ; mem[0x105] = 0xEE ;


    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}

fn main_() {
    let mut memory: [u8; 4096] = [0; 4096];
    let mem = &mut memory;
    let add_twice = [0x80, 0x14, 0x80, 0x14, 0x00, 0xEE];
    mem[0x100..0x106].copy_from_slice(&add_twice);
    println!("{:?}", &mem[0x100..0x106]);
}

// without requiring a temporary array is to overwrite bytes directly
fn main_2() {
    let mut memory: [u8; 4096] = [0; 4096];
    let mem = &mut memory;
    mem[0x100] = 0x80;
    mem[0x101] = 0x14;

    mem[0x102] = 0x80;
    mem[0x103] = 0x14;

    mem[0x104] = 0x00;
    mem[0x105] = 0xEE;
    println!("{:?}", &mem[0x100..0x106]);
}
// Prints [128, 20, 128, 20, 0, 238]
