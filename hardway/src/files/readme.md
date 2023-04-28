## 读书
https://rust-lang-nursery.github.io/rust-cookbook/file/dir.html

https://www.tutorialspoint.com/rust/rust_file_input_output.htm

https://www.includehelp.com/rust/programs.aspx#File-IO

## 博客

https://mkaz.blog/working-with-rust/files-and-dirs/

https://github.com/fasterthanlime 
[Reading files the hard way](https://fasterthanli.me/series/reading-files-the-hard-way)

### 存在性

~~~rust

use std::path::Path;

fn main() {
    println!("{}", Path::new("/etc/hosts").exists());
}

// 标准库
    pub fn exists(&self) -> bool {
        fs::metadata(self).is_ok()
    }
    // tokio::fs::metadata 里面也有

~~~


## global

You could also use glob, which is expressly for this purpose.

```rust
extern crate glob;
use self::glob::glob;

let files:Vec<Path> = glob("*").collect();


// iterator
let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }

// Display file metadata like owner and group using Rust
{
    use std::fs::*;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::mem::transmute;


fn main(){
    let meta = metadata("./test.txt");
    if meta.is_ok(){
        println!("{}", meta.st_gid());

// Owner is meta.unwrap().uid() and group is meta.unwrap().gid(). They are u32 each, which is what Linux uses.

        let m:u32 = meta.unwrap().permissions().mode();
        //let bytes: [u8; 4] = unsafe { transmute(m.to_be()) };//etv. used later

        print!("{}",if (m & (0x1<<9)) >= 1 {"d"}else{"-"});
        print!("{}",if (m & (0x1<<8)) >= 1 {"r"}else{"-"});
        print!("{}",if (m & (0x1<<7)) >= 1 {"w"}else{"-"});
        print!("{}",if (m & (0x1<<6)) >= 1 {"x"}else{"-"});
        print!("{}",if (m & (0x1<<5)) >= 1 {"r"}else{"-"});
        print!("{}",if (m & (0x1<<4)) >= 1 {"w"}else{"-"});
        print!("{}",if (m & (0x1<<3)) >= 1 {"x"}else{"-"});
        print!("{}",if (m & (0x1<<2)) >= 1 {"r"}else{"-"});
        print!("{}",if (m & (0x1<<1)) >= 1 {"w"}else{"-"});
        println!("{}",if (m & 0x1) >= 1 {"x"}else{"-"});
        println!("{:b}",m);
    }
}
}

```
