use std::{  fs::{self, File}, path::Path};

#[test]
fn test_create() {
    let result = fs::create_dir("./_runtime"); // create_dir_all
    if result.is_err() {
        println!("file alwrady created");
    } 

    let result = fs::create_dir_all("./_runtime/my_dir"); // create_dir_all

    if result.is_err() {
      println!("[create_dir_all error]: {:?}", result);
    }else{
      println!("[create_dir_all ok]");
    }

    // 创建路径
    let path = Path::new("./_runtime/my_dir/my_text.txt");
    let parent = path.parent();
    println!("{:?}", parent);
    let file_name = path.file_stem() ;
    println!("file name {:?}", file_name);

}

const MY_FILE: &'static str = "./_runtime/my_dir/my_text.txt" ;
// const MY_FILE: &'static str = "my_text.txt" ;

// use std::io::{Read};
use std::io::prelude::* ;
#[test]
fn test_read_file(){
   let mut f = File::open("./runtime/my_dir/my_text.txt") ;
   if f.is_ok() {
      let mut buf = [0; 1024];
      let n = f.unwrap().read(&mut buf[..]) ;
   }

   // 2
   let mut f = File::open(MY_FILE) ;
   if f.is_ok() {
      let mut buf = String::new() ;
      f.unwrap().read_to_string(&mut buf);

      println!("{}", buf);

      // 2
      let mut f = File::open(MY_FILE);
      let mut buf: Vec<u8> = Vec::new();
      f.unwrap().read_to_end(&mut buf);
      let s = String::from_utf8(buf).unwrap() ;
      println!("content2: {}", s);
   }
}

#[test]
fn test_write_file(){
   let   f = File::create(MY_FILE);
   if f.is_ok() {
      println!("file created {}" , MY_FILE);

      let mut file = f.unwrap() ;

      let _ = file.write_all(b"text for test");

      let _ = file.flush() ;

   }else{
      println!("file not created ,error {:?}",f);
   }
}

#[test]
fn test_options() {
   use std::fs::OpenOptions ;

   let f = OpenOptions::new() 
      .create(true)
      .write(true)
      .read(true)
      .append(true)
      .open(MY_FILE)
      ;

   if f.is_ok() {
      let mut file = f.unwrap() ;

      let mut s = String::new() ;
      let result = file.read_to_string(&mut s);

      if result.is_ok(){
         println!("file content is =>  {}", s);
      }

      file.write(b"\r\n some appended content here");
   }
      
}


#[test]
fn test_buf_read(){
   use std::io::BufReader ;

   let mut f = File::open(MY_FILE).expect("file not exists!");

   let bf = BufReader::new(f);

   for line in bf.lines(){
      let s = line.unwrap() ;
      println!("line: {:?}", s);
   }
}